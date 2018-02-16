
use std::collections::HashMap;

use serde_json::Value;


pub struct LDUser {
	pub key: String,
	pub secondary: Option<String>,
	pub ip: Option<String>,
	pub email: Option<String>,
	pub name: Option<String>,
	pub avatar: Option<String>,
	pub firstName: Option<String>,
	pub lastName: Option<String>,
	pub anonymous: Option<bool>,
	pub country: Option<String>,
	pub custom: Option<HashMap<String, Value>>
}

impl LDUser {

	pub fn new(key: String) -> LDUser {
		LDUser {
			key: key,
			secondary: None,
			ip: None,
			email: None,
			name: None,
			avatar: None,
			firstName: None,
			lastName: None,
			anonymous: None,
			country: None,
			custom: None
		}
	}

	pub fn with_email(&self, value: Option<String>) -> LDUser {
		LDUser {
			key: self.key.clone(),
			secondary: self.secondary.clone(),
			ip: self.ip.clone(),
			email: value.clone(),
			name: self.name.clone(),
			avatar: self.avatar.clone(),
			firstName: self.firstName.clone(),
			lastName: self.lastName.clone(),
			anonymous: self.anonymous.clone(),
			country: self.country.clone(),
			custom: self.custom.clone()
		}
	}

	pub fn with_name(&self, value: Option<String>) -> LDUser {
		LDUser {
			key: self.key.clone(),
			secondary: self.secondary.clone(),
			ip: self.ip.clone(),
			email: self.email.clone(),
			name: value.clone(),
			avatar: self.avatar.clone(),
			firstName: self.firstName.clone(),
			lastName: self.lastName.clone(),
			anonymous: self.anonymous.clone(),
			country: self.country.clone(),
			custom: self.custom.clone()
		}
	}

	pub fn get_value_for_evaluation(&self, attr: &String) -> Value {
		match attr.as_str() {
			"key" => json!(self.key),
			"secondary" => json!(self.secondary),
			"ip" => json!(self.ip),
			"email" => json!(self.email),
			"name" => json!(self.name),
			"avatar" => json!(self.avatar),
			"firstName" => json!(self.firstName),
			"lastName" => json!(self.lastName),
			"anonymous" => json!(self.anonymous),
			"country" => json!(self.country),
			_ => match self.custom {
				Some(ref map) => match map.get(attr) {
					Some(value) => value.clone(),
					None => json!(null)
				},
				None => json!(null)
			}
		}
	}
}
