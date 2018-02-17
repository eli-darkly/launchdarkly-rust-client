
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde_json::Value;

use config::LDConfig;
use feature_store::FeatureStore;
use polling::PollingProcessor;
use user::LDUser;


pub struct LDClient {
	poller: PollingProcessor,
	store: Arc<Mutex<FeatureStore>>
}

impl LDClient {
	pub fn new(sdk_key: String, config: LDConfig) -> LDClient {
		let store_impl = (*config.feature_store_factory)();
		let store = Arc::new(Mutex::new(store_impl));
		let poller: PollingProcessor =
			PollingProcessor::new(sdk_key, &store, &config.base_uri, config.polling_interval_millis);
		let mut client = LDClient {
			poller: poller,
			store: store
		};
		client.start();
		client
	}

	fn start(&mut self) {
		let ready = self.poller.start();
		match ready.recv() {
			Ok(_) => (),
			Err(_) => () // TODO: error logging
		}
	}

	pub fn variation(&self, flag_key: &String, user: &LDUser, default: Value) -> Value {
		let store = self.store.lock().unwrap();
		match store.get(flag_key) {
			Some(flag) => {
				let eval_result = flag.evaluate(user, &store);
				// TODO: send events (eval_result.1)
				eval_result.0
			}
			None => default
		}
	}

	pub fn all_flags(&self, user: &LDUser) -> HashMap<String, Value> {
		let store = self.store.lock().unwrap();
		let flags = store.all();
		let mut ret: HashMap<String, Value> = HashMap::new();
		for (key, flag) in flags.iter() {
			ret.insert(key.clone(), flag.evaluate(user, &store).0);
		}
		ret
	}
}
