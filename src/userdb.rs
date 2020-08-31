extern crate serde;

use std::sync::Mutex;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    name: String,
    password: String,
}

pub struct UserDB {
    list: Mutex<Vec<User>>
}

impl UserDB {
    pub fn get() -> UserDB {
        UserDB {
            list: Mutex::new(vec![]),
        }
    }

    pub fn add(self: &UserDB, user: User) -> Result<(), String> {
        match self.find(&user.name) {
            None => {
                self.list.lock().unwrap().push(user);
                Ok(())
            }
            Some(_) => {
                Err("User already existed".into())
            }
        }
    }

    pub fn find(self: &UserDB, name: &str) -> Option<User> {
        self.list.lock().unwrap()
            .iter()
            .find(|user| {
                user.name == name
            })
            .map(|user| { user.clone() })
    }
}
