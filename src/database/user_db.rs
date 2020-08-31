use std::fmt;
use std::sync::{Mutex, Arc, Weak};
use crate::model::user::User;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum UserDBError {
    UserExists,
}

impl fmt::Display for UserDBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            UserDBError::UserExists => write!(f, "User already exists")
        }
    }
}

pub struct UserDB {
    list: Mutex<Vec<Arc<User>>>
}

impl UserDB {
    pub fn get() -> UserDB {
        UserDB {
            list: Mutex::new(vec![]),
        }
    }

    pub fn add(self: &UserDB, user: User) -> Result<Arc<User>, UserDBError> {
        match self.find(&user.name) {
            None => {
                let arc = Arc::new(user);
                self.list.lock().unwrap().push(arc.clone());
                Ok(arc)
            }
            Some(_) => {
                Err(UserDBError::UserExists)
            }
        }
    }

    pub fn find(self: &UserDB, name: &str) -> Option<Weak<User>> {
        self.list.lock().unwrap()
            .iter()
            .find(|user| {
                user.name == name
            })
            .map(|user| { Arc::downgrade(user) })
    }
}
