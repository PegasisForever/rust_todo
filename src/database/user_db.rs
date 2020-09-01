use std::fmt;
use std::sync::{Mutex, Arc, Weak};
use crate::model::user::User;
use std::fmt::Formatter;
use std::fs;
use log::info;
use std::path::Path;
use serde_json::from_str;

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

impl std::error::Error for UserDBError {}

pub struct UserDB {
    file_path: String,
    list: Mutex<Vec<Arc<User>>>,
}

impl UserDB {
    pub fn new(file_path: String) -> UserDB {
        let path = Path::new(&file_path);
        if !path.exists() {
            path.parent().map(|parent| {
                fs::create_dir_all(parent).unwrap();
            });
            fs::write(path, "[]").unwrap();
        }
        let json = fs::read_to_string(&file_path).unwrap();

        let deserialized = from_str::<Vec<User>>(&json)
            .unwrap()
            .into_iter()
            .map(|user| Arc::new(user))
            .collect::<Vec<Arc<User>>>();
        info!("Read UserDB from {}.", &file_path);
        UserDB {
            file_path,
            list: Mutex::new(deserialized),
        }
    }

    pub fn save(self: &UserDB) {
        let list = self.list.lock().unwrap();
        let mapped = list.iter()
            .map(|item| { item.as_ref() })
            .collect::<Vec<&User>>();
        let serialized = serde_json::to_string(&mapped).unwrap();
        fs::write(&self.file_path, serialized).unwrap();
        info!("UserDB saved to {}.", &self.file_path);
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
