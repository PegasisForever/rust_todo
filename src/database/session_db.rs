use std::sync::{Mutex};
use crate::model::session::Session;
use uuid::Uuid;

pub struct SessionDB {
    list: Mutex<Vec<Session>>
}

impl SessionDB {
    pub fn get() -> Self {
        Self {
            list: Mutex::new(vec![]),
        }
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
            .map(|session| { session.clone() })
    }
}
