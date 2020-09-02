use std::fmt;
use std::sync::{Mutex, Arc, Weak};
use crate::model::user::User;
use std::fmt::Formatter;
use std::fs;
use log::info;
use crate::tools::ensure_file_exists;

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
        ensure_file_exists(&file_path, "{}").unwrap();

        let json = json::parse(&fs::read_to_string(&file_path).unwrap()).unwrap();
        let list = json.members()
            .map(|user_json| {
                Arc::new(User::from_json(user_json))
            })
            .collect::<Vec<Arc<User>>>();

        info!("Read UserDB from {}.", &file_path);
        UserDB {
            file_path,
            list: Mutex::new(list),
        }
    }

    pub fn save(self: &UserDB) {
        fs::write(&self.file_path, self.serialize()).unwrap();
        info!("UserDB saved to {}.", &self.file_path);
    }

    fn serialize(self: &Self) -> String {
        let list = self.list.lock().unwrap();
        let mut json = json::JsonValue::new_array();
        list.iter().for_each(|user| {
            json.push(user.to_json()).unwrap();
        });

        json.dump()
    }

    pub fn add(self: &UserDB, name: &str, password: &str) -> Result<Arc<User>, UserDBError> {
        match self.find(name) {
            None => {
                let user = User {
                    name: String::from(name),
                    password: String::from(password),
                };
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
