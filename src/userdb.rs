extern crate serde;

use std::fmt;
use std::sync::Mutex;
use serde::Deserialize;
use self::serde::export::Formatter;

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    name: String,
    password: String,
}

pub struct UserDB {
    list: Mutex<Vec<User>>
}

#[derive(Debug)]
pub enum DBError {
    UserExists,
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DBError::UserExists => write!(f, "User already exists")
        }
    }
}

impl UserDB {
    pub fn get() -> UserDB {
        UserDB {
            list: Mutex::new(vec![]),
        }
    }

    pub fn add(self: &UserDB, user: User) -> Result<(), DBError> {
        match self.find(&user.name) {
            None => {
                self.list.lock().unwrap().push(user);
                Ok(())
            }
            Some(_) => {
                Err(DBError::UserExists)
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
