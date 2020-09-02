use serde::Deserialize;
use std::hash::Hasher;
use json::{object, JsonValue};
use argon2::{self, Config};

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub name: String,
    password_hash: String,
}

const SALT: &[u8] = "fguU2N7af73!5^rD!!cZE9Z!5CK&f67yFPYRBvHM4%8UbbBNXVW-d+t7*QQwzn4c".as_bytes();

impl User {
    pub fn new(name: &str, password: &str) -> Self {
        let config = Config::default();
        let hash = argon2::hash_encoded(password.as_bytes(), SALT, &config).unwrap();

        Self {
            name: String::from(name),
            password_hash: hash,
        }
    }

    pub fn from_json(json: &JsonValue) -> Self {
        Self {
            name: String::from(json["name"].as_str().unwrap()),
            password_hash: String::from(json["password_hash"].as_str().unwrap()),
        }
    }

    pub fn to_json(self: &Self) -> JsonValue {
        object! {
            name: self.name.clone(),
            password_hash: self.password_hash.clone(),
        }
    }

    pub fn verify_password(self: &Self, password: &str) -> bool {
        argon2::verify_encoded(&self.password_hash, password.as_bytes()).is_ok()
    }
}

impl std::cmp::PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.password_hash == other.password_hash
    }
}

impl std::cmp::Eq for User {}

impl std::hash::Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.password_hash.hash(state);
    }
}
