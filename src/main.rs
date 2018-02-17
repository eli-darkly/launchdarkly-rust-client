
extern crate ldclient;
extern crate serde_json;

use std::thread;
use std::time;

use serde_json::Value;

use ldclient::client::LDClient;
use ldclient::config::LDConfig;
use ldclient::user::LDUser;

fn main() {
	let config = LDConfig::default()
        .with_polling_interval_millis(5000);
    let sdk_key = "sdk-03947004-7d32-4878-a80b-ade2314efece".to_owned();
    let client = LDClient::new(sdk_key, config);
    
    let user = LDUser::new(String::from("bozo"))
        .with_email(Some(String::from("test@example.com")));
    let flag_key = String::from("new.dashboard");

    loop {
        let result = client.variation(&flag_key, &user, Value::Null);
        println!("flag value is: {}", result);
        thread::sleep(time::Duration::from_millis(1000));
    }
}
