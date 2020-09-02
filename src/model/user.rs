use serde::Deserialize;
use std::hash::Hasher;
use json::{object, JsonValue};

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub name: String,
    pub password: String,
}

impl User {
    pub fn from_json(json: &JsonValue) -> Self {
        Self {
            name: String::from(json["name"].as_str().unwrap()),
            password: String::from(json["password"].as_str().unwrap()),
        }
    }

    pub fn to_json(self: &Self) -> JsonValue {
        object! {
            name: self.name.clone(),
            password: self.password.clone(),
        }
    }

    pub fn verify_password(self: &Self, password: &str) -> bool {
        self.password == password
    }
}

impl std::cmp::PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.password == other.password
    }
}

impl std::cmp::Eq for User {}

impl std::hash::Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.password.hash(state);
    }
}
