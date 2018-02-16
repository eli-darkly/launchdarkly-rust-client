
use std::collections::HashMap;

use flag::FeatureFlag;

// TODO: make this into a trait so there can be both an in-memory implementation and a Redis one.

pub struct FeatureStore {
	items: HashMap<String, FeatureFlag>
}

impl FeatureStore {

	pub fn new() -> FeatureStore {
		let items: HashMap<String, FeatureFlag> = HashMap::new();
		FeatureStore { items }
	}

	pub fn get(&self, key: &String) -> Option<FeatureFlag> {
		let opt_flag = self.items.get(key);
		match opt_flag {
			Some(flag) => if flag.deleted { None } else { Some(flag.clone()) },
			None => None
		}
	}

	pub fn all(&self) -> HashMap<String, FeatureFlag> {
		self.items.clone()
	}

	pub fn init(& mut self, flags: &HashMap<String, FeatureFlag>) {
		self.items.clear();
		for (key, value) in flags.iter() {
			self.items.insert(key.clone(), (*value).clone());
		}
	}
}
