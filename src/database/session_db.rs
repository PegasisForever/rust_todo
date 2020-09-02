use std::sync::{Mutex};
use crate::model::session::Session;
use uuid::Uuid;
use log::info;
use std::fs;
use crate::database::user_db::UserDB;
use crate::tools::ensure_file_exists;

pub struct SessionDB {
    file_path: String,
    list: Mutex<Vec<Session>>,
}

impl SessionDB {
    pub fn new(file_path: String, user_db: &UserDB) -> Self {
        ensure_file_exists(&file_path, "{}").unwrap();

        let json = json::parse(&fs::read_to_string(&file_path).unwrap()).unwrap();
        let list = json.members()
            .map(|session_json| {
                Session::from_json(session_json, user_db)
            })
            .collect::<Vec<Session>>();

        info!("Read SessionDB from {}.", &file_path);
        Self {
            file_path,
            list: Mutex::new(list),
        }
    }

    pub fn save(self: &Self) {
        fs::write(&self.file_path, self.serialize()).unwrap();
        info!("SessionDB saved to {}.", &self.file_path);
    }

    fn serialize(self: &Self) -> String {
        let list = self.list.lock().unwrap();
        let mut json = json::JsonValue::new_array();
        list.iter()
            .filter_map(|session| {
                session.to_json()
            })
            .for_each(|session_json| {
                json.push(session_json).unwrap();
            });

        json.dump()
    }

    pub fn add(self: &Self, session: Session) {
        self.list.lock().unwrap().push(session)
    }

    pub fn find(self: &Self, id: &Uuid) -> Option<Session> {
        self.list.lock().unwrap()
            .iter()
            .find(|session| {
                session.id == *id
            })
            .filter(|session| { session.is_valid() })
            .map(|session| { session.clone() })
    }
}
