use serde::Deserialize;
use std::hash::Hasher;

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub password: String,
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
