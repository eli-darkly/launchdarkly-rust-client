
extern crate time;

use serde_json::Value;

use user::LDUser;


pub struct FeatureRequestEvent {
	pub creationDate: u64,
	pub key: String,
	pub kind: String,
	pub user: LDUser,
	pub value: Value,
	pub default: Option<Value>,
	pub version: Option<u32>,
	pub prereqOf: String
}

// pub fn feature_request(key: String, user: &LDUser, value: &Value, default: Option<Value>,
// 					   version: Option<u32>, prereq_of: &String) -> FeatureRequestEvent {
// 	FeatureRequestEvent {
// 		creationDate: current_time_millis(),
// 		key: key,
// 		kind: String::from("feature"),
// 		user: user.clone(),
// 		value: value.clone(),
// 		default: default,
// 		version: version,
// 		prereqOf: prereq_of.clone()
// 	}
// }

// fn current_time_millis() -> u64 {
// 	let ts = time::get_time();
// 	return ts.sec as u64 + ts.nsec as u64 / 1000 / 1000;
// }
