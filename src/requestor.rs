
use std::collections::HashMap;

use reqwest;
use serde_json;

use config::LDConfig;
use flag::FeatureFlag;


pub struct Requestor {
	sdk_key: String,
	base_uri: String,
	client: reqwest::Client
}

#[derive(Deserialize)]
struct AllData {
	pub flags: HashMap<String, FeatureFlag>
}

impl Requestor {

	pub fn new(sdk_key: String, config: &LDConfig) -> Requestor {
		Requestor {
			sdk_key: sdk_key,
			base_uri: config.base_uri.clone(),
			client: reqwest::Client::new()
		}
	}

	pub fn get_all_flags(&self) -> Result<HashMap<String, FeatureFlag>, String> {
		let uri = format!("{}/sdk/latest-all", self.base_uri);
		let resp_result = self.client.get(&uri)
			.header(reqwest::header::Authorization(self.sdk_key.clone()))
			.send();
		match resp_result {
			Ok(mut resp) => match resp.text() {
				Ok(resp_str) => {
					let try_parse: Result<AllData, serde_json::Error> = serde_json::from_str(&resp_str);
					match try_parse {
						Ok(all_data) => Ok(all_data.flags),
						Err(_) => Err("unable to parse json".to_owned())
					}
				},
				Err(_) => Err("unable to get response".to_owned())
			},
			Err(_) => Err("unable to get response".to_owned())
		}
	}
}
