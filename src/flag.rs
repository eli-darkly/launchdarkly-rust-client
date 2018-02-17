
use regex::Regex;
use semver::Version;
use serde_json::Value;
use sha1;
use time::{Timespec, strptime};

use event::FeatureRequestEvent;
use feature_store::FeatureStore;
use user::LDUser;


#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct FeatureFlag {
	pub key: String,
	pub version: u32,
	pub on: bool,
	pub prerequisites: Vec<Prerequisite>,
	pub salt: String,
	pub targets: Vec<Target>,
	pub rules: Vec<Rule>,
	pub fallthrough: VariationOrRollout,
	pub offVariation: Option<u32>,
	pub variations: Vec<Value>,
	pub deleted: bool
}

impl FeatureFlag {
	pub fn evaluate(&self, user: &LDUser, store: &FeatureStore) -> (Value, Vec<FeatureRequestEvent>) {
		let mut prereq_events: Vec<FeatureRequestEvent> = vec![];
		if self.on {
			let maybe_value = self.evaluate_internal(user, store, &mut prereq_events);
			if let Some(value) = maybe_value {
				return (value, prereq_events);
			}
		}
		let off_variation = self.get_off_variation_value();
		(off_variation, prereq_events)
	}

	fn evaluate_internal(&self, user: &LDUser, store: &FeatureStore, prereq_events: &mut Vec<FeatureRequestEvent>) -> Option<Value> {
		for prereq in &self.prerequisites {
			match store.get(&prereq.key) {
				Some(prereq_flag) => {
					if prereq_flag.on {
						let prereq_result = prereq_flag.evaluate_internal(user, store, prereq_events);
						let variation = prereq_flag.get_variation(Some(prereq.variation));
						if prereq_result != variation {
							return None;
						}
						// TODO: create event
						// let event = event::feature_request(prereq_flag.key, user, &prereq_result,
						// 	None, Some(prereq_flag.version), &self.key);
						// prereq_events.push(event);
					} else {
						return None;
					}
				}
				None => {
					// log: could not retrieve flag
					return None;
				}
			}
		}
		self.get_variation(self.evaluate_index(user))
	}

	fn get_variation(&self, index: Option<u32>) -> Option<Value> {
		match index {
			Some(i) =>
				if i < self.variations.len() as u32 {
					Some(self.variations[i as usize].clone())
				} else {
					// throw
					None
				},
			None => None
		}
	}

	fn get_off_variation_value(&self) -> Value {
		match self.offVariation {
			Some(n) => self.get_variation(Some(n)).unwrap_or(Value::Null),
			None => Value::Null
		}
	}

	fn evaluate_index(&self, user: &LDUser) -> Option<u32> {
		for target in &self.targets {
			for value in &target.values {
				if *value == user.key {
					return Some(target.variation);
				}
			}
		}
		for rule in &self.rules {
			if rule.matches_user(user) {
				return rule.variation_index_for_user(user, &self.key, &self.salt);
			}
		}
		return self.fallthrough.variation_index_for_user(user, &self.key, &self.salt);
	}
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Prerequisite {
	pub key: String,
	pub variation: u32
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Target {
	pub values: Vec<String>,
	pub variation: u32
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Rule {
	// TODO: should share a trait with VariationOrRollout
	pub clauses: Vec<Clause>,
	pub variation: Option<u32>,
	pub rollout: Option<Rollout>
}

impl Rule {
	pub fn matches_user(&self, user: &LDUser) -> bool {
		for clause in &self.clauses {
			if !clause.matches_user(user) {
				return false;
			}
		}
		true
	}

	pub fn variation_index_for_user(&self, user: &LDUser, key: &String, salt: &String) -> Option<u32> {
		// TODO: use a trait
		variation_index(self.variation, &self.rollout, user, key, salt)
	}
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Clause {
	pub attribute: String,
	pub op: String,
	pub values: Vec<Value>,
	pub negate: bool
}

impl Clause {
	fn matches_user(&self, user: &LDUser) -> bool {
		let user_value = user.get_value_for_evaluation(&self.attribute);
		match user_value {
			Value::Null => false,
			Value::Array(values) => self.maybe_negate(self.match_any_of_any(values)),
			_ => self.maybe_negate(self.match_any(&user_value))
		}
	}

	fn maybe_negate(&self, result: bool) -> bool {
		if self.negate {
			!result
		} else {
			result
		}
	}

	fn match_any(&self, user_value: &Value) -> bool {
		for v in &self.values {
			if apply_op(&self.op, user_value, v) {
				return true;
			}
		}
		return false;
	}

	fn match_any_of_any(&self, user_values: Vec<Value>) -> bool {
		for uv in &user_values {
			if self.match_any(uv) {
				return true;
			}
		}
		return false;
	}
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct VariationOrRollout {
	pub variation: Option<u32>,
	pub rollout: Option<Rollout>
}

impl VariationOrRollout {
	pub fn variation_index_for_user(&self, user: &LDUser, key: &String, salt: &String) -> Option<u32> {
		variation_index(self.variation, &self.rollout, user, key, salt)
	}
}

fn variation_index(variation: Option<u32>, rollout: &Option<Rollout>,
				   user: &LDUser, key: &String, salt: &String) -> Option<u32> {
	match variation {
		Some(n) => Some(n),
		None => match rollout {
			&Some(ref roll) => {
				let bucket_by = roll.bucketBy.clone().unwrap_or(String::from("key"));
				let bucket = bucket_user(user, key, &bucket_by, salt);
				let mut sum = 0.0;
				for wv in &roll.variations {
					sum += wv.weight as f32 / 100000.0;
					if bucket < sum {
						return Some(wv.variation)
					}
				}
				None
			},
			&None => None
		}
	}
}

fn bucket_user(user: &LDUser, key: &String, bucket_by: &String, salt: &String) -> f32 {
	let user_value = user.get_value_for_evaluation(bucket_by);
	let maybe_hash_input = get_bucketable_string_value(user_value);
	match maybe_hash_input {
		Some(hash_input) => {
			let hash_str = match user.secondary.clone() {
				Some(sec) => format!("{}.{}", hash_input, sec),
				None => hash_input
			};
			let mut sha = sha1::Sha1::new();
			sha.update(format!("{}.{}.{}", key, salt, hash_str).as_bytes());
			let hash_out: String = sha.digest().to_string().chars().take(15).collect();
			let long_val = i32::from_str_radix(&hash_out, 16).unwrap_or(0);
			(long_val as f32) / 100000.0
		}
		None => 0.0
	}
}

fn get_bucketable_string_value(user_value: Value) -> Option<String> {
	match user_value {
		Value::String(s) => Some(s),
		Value::Number(n) => n.as_i64().map(|n| n.to_string()),
		_ => None
	}
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Rollout {
	pub variations: Vec<WeightedVariation>,
	pub bucketBy: Option<String>
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct WeightedVariation {
	pub variation: u32,
	pub weight: u32
}

fn apply_op(op: &String, user_value: &Value, clause_value: &Value) -> bool {
	match op.as_ref() {
		"in" => user_value == clause_value,
		"endsWith" => apply_string_op(user_value, clause_value, &(|us: &String, cs: &String| us.starts_with(cs))),
		"startsWith" => apply_string_op(user_value, clause_value, &(|us: &String, cs: &String| us.ends_with(cs))),
		"matches" => apply_string_op(user_value, clause_value, &regex_match_op),
		"contains" => apply_string_op(user_value, clause_value, &(|us: &String, cs: &String| us.contains(cs))),
		"lessThan" => apply_numeric_op(user_value, clause_value, &(|un: f64, cn: f64| un < cn)),
		"lessThanOrEqual" => apply_numeric_op(user_value, clause_value, &(|un: f64, cn: f64| un <= cn)),
		"greaterThan" => apply_numeric_op(user_value, clause_value, &(|un: f64, cn: f64| un > cn)),
		"greaterThanOrEqual" => apply_numeric_op(user_value, clause_value, &(|un: f64, cn: f64| un >= cn)),
		"before" => apply_time_op(user_value, clause_value, &(|ut: Timespec, ct: Timespec| ut < ct)),
		"after" => apply_time_op(user_value, clause_value, &(|ut: Timespec, ct: Timespec| ut > ct)),
		"semVerEqual" => apply_semver_op(user_value, clause_value, &(|uv: Version, cv: Version| uv == cv)),
		"semVerLessThan" => apply_semver_op(user_value, clause_value, &(|uv: Version, cv: Version| uv < cv)),
		"semVerGreaterThan" => apply_semver_op(user_value, clause_value, &(|uv: Version, cv: Version| uv > cv)),
		_ => false
	}
}

fn apply_string_op(user_value: &Value, clause_value: &Value, f: &Fn(&String, &String) -> bool) -> bool {
	match user_value {
		&Value::String(ref us) => match clause_value {
			&Value::String(ref cs) => f(us, cs),
			_ => false
		},
		_ => false
	}
}

fn apply_numeric_op(user_value: &Value, clause_value: &Value, f: &Fn(f64, f64) -> bool) -> bool {
	match user_value {
		&Value::Number(ref un) => match clause_value {
			&Value::Number(ref cn) => f(un.as_f64().unwrap_or(0.0), cn.as_f64().unwrap_or(0.0)),
			_ => false
		},
		_ => false
	}
}

fn apply_time_op(user_value: &Value, clause_value: &Value, f: &Fn(Timespec, Timespec) -> bool) -> bool {
	match parse_date_time(user_value) {
		Some(ut) => match parse_date_time(clause_value) {
			Some(ct) => f(ut, ct),
			_ => false
		},
		_ => false
	}
}

fn apply_semver_op(user_value: &Value, clause_value: &Value, f: &Fn(Version, Version) -> bool) -> bool {
	match parse_semver(user_value) {
		Some(uv) => match parse_semver(clause_value) {
			Some(cv) => f(uv, cv),
			_ => false
		},
		_ => false
	}
}

fn regex_match_op(us: &String, cs: &String) -> bool {
	match Regex::new(cs) {
		Ok(re) => re.is_match(us),
		_ => false
	}
}

fn parse_date_time(value: &Value) -> Option<Timespec> {
	match value {
		&Value::String(ref s) => match strptime(&s, &String::from("%FT%TZ")) {
			Ok(tm) => Some(tm.to_timespec()),
			_ => None
		},
		&Value::Number(ref un) => match un.as_i64() {
			Some(n) => Some(Timespec::new(n / 1000, ((n % 1000) * 1000000) as i32)),
			_ => None
		},
		_ => None
	}
}

fn parse_semver(value: &Value) -> Option<Version> {
	match value {
		&Value::String(ref s) => match Version::parse(s) {
			Ok(sv) => Some(sv),
			_ => None
		},
		_ => None
	}
	// TODO: LaunchDarkly allows semvers to omit the minor and/or patch versions, so if we
	// get a parse error here, we should try adding ".0" or ".0.0" at the appropriate point.
}
