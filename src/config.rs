
// TODO: The feature store should be part of the configuration, but I haven't figured out
// how to mark things with the proper "lifetime" specifier to keep Rust happy when the store
// is referenced elsewhere.

pub struct LDConfig {
	pub base_uri: String,
	pub polling_interval_millis: u64
}

pub fn default() -> LDConfig {
	LDConfig {
		base_uri: String::from("https://app.launchdarkly.com"),
		polling_interval_millis: 30000
	}
}
