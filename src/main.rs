
extern crate ldclient;
extern crate serde_json;

use serde_json::Value;

use ldclient::client::LDClient;
use ldclient::config;
use ldclient::user::LDUser;

fn main() {
	let config = config::default();
    let sdk_key = "sdk-03947004-7d32-4878-a80b-ade2314efece".to_owned();
    let client = LDClient::new(sdk_key, config);

    let user = LDUser::new(String::from("bozo"))
        .with_email(Some(String::from("test@example.com")));
    let result = client.variation(String::from("new.dashboard"), &user, Value::Null);
    println!("");
    println!("flag value is: {} !!!!!", result);
    println!("");
}
