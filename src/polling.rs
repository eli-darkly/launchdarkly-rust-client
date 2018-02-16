
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time;

use config::LDConfig;
use feature_store::FeatureStore;
use requestor::Requestor;


pub struct PollingProcessor {
	requestor: Requestor,
	interval: u64
}

impl PollingProcessor {
	pub fn new(sdk_key: String, config: &LDConfig) -> PollingProcessor {
		PollingProcessor {
			requestor: Requestor::new(sdk_key, config),
			interval: config.polling_interval_millis
		}
	}

	pub fn poll(&self, store: &mut FeatureStore) {
		match self.requestor.get_all_flags() {
			Ok(flags) => store.init(&flags),
			_ => () // TODO
		}
	}

	// TODO: make a thread - I have no idea how to make this work
	// pub fn start(&self, store: &mut FeatureStore) {
	// 	thread::spawn(move || {
	// 		loop {
	// 			self.poll(store);
	// 			thread::sleep(time::Duration::from_millis(self.interval));
	// 		}
	// 	});
	// }
}
