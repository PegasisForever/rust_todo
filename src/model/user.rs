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
        Self {
            name: String::from(name),
            password_hash: User::hash_password(name, password),
        }
    }

    fn hash_password(name: &str, password: &str) -> String {
        let mut config = Config::default();
        config.time_cost = 4;
        let name_hash = argon2::hash_raw(name.as_bytes(), SALT, &config).unwrap();
        let hash = argon2::hash_raw(password.as_bytes(), &name_hash, &config).unwrap();
        hex::encode(hash)
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
        User::hash_password(&self.name, password) == self.password_hash
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
