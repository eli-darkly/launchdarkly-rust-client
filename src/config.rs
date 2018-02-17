
use feature_store::FeatureStore;

// TODO: The feature store should be part of the configuration, but I haven't figured out
// how to mark things with the proper "lifetime" specifier to keep Rust happy when the store
// is referenced elsewhere.

pub struct LDConfig {
	pub base_uri: String,
	pub polling_interval_millis: u64,
	pub feature_store_factory: &'static Fn() -> FeatureStore
}

impl LDConfig {
	pub fn default() -> LDConfig {
		LDConfig {
			base_uri: String::from("https://app.launchdarkly.com"),
			polling_interval_millis: 30000,
			feature_store_factory: &FeatureStore::in_memory_store
		}
	}

	pub fn with_base_uri(&self, base_uri: String) -> LDConfig {
		LDConfig {
			base_uri: base_uri.clone(),
			polling_interval_millis: self.polling_interval_millis,
			feature_store_factory: self.feature_store_factory
		}
	}

	pub fn with_polling_interval_millis(&self, millis: u64) -> LDConfig {
		LDConfig {
			base_uri: self.base_uri.clone(),
			polling_interval_millis: millis,
			feature_store_factory: self.feature_store_factory
		}
	}

	pub fn with_feature_store_factory(&self, factory: &'static Fn() -> FeatureStore) -> LDConfig {
		LDConfig {
			base_uri: self.base_uri.clone(),
			polling_interval_millis: self.polling_interval_millis,
			feature_store_factory: factory
		}
	}
}
