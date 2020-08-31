extern crate serde;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub password: String,
}
