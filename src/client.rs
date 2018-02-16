
use std::collections::HashMap;

use serde_json::Value;

use config::LDConfig;
use feature_store::FeatureStore;
use polling::PollingProcessor;
use user::LDUser;


pub struct LDClient {
	poller: PollingProcessor,
	store: FeatureStore
}

impl LDClient {
	pub fn new(sdk_key: String, config: LDConfig) -> LDClient {
		let mut client = LDClient {
			poller: PollingProcessor::new(sdk_key, &config),
			store: FeatureStore::new()
		};
		client.poller.poll(&mut client.store);
		client
	}

	pub fn variation(&self, flag_key: String, user: &LDUser, default: Value) -> Value {
		match self.store.get(&flag_key) {
			Some(flag) => {
				let eval_result = flag.evaluate(user, &self.store);
				// TODO: send events (eval_result.1)
				eval_result.0
			}
			None => default
		}
	}

	pub fn all_flags(&self, user: &LDUser) -> HashMap<String, Value> {
		let flags = self.store.all();
		let mut ret: HashMap<String, Value> = HashMap::new();
		for (key, flag) in flags.iter() {
			ret.insert(key.clone(), flag.evaluate(user, &self.store).0);
		}
		ret
	}
}
