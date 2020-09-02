use std::sync::Weak;
use crate::model::user::User;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use json::{object, JsonValue};
use crate::database::user_db::UserDB;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub user: Weak<User>,
    pub expire_time: SystemTime,
}

const EXPIRE_DURATION: Duration = Duration::from_secs(60 * 10);

impl Session {
    pub fn new(user: Weak<User>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user,
            expire_time: SystemTime::now() + EXPIRE_DURATION,
        }
    }

    pub fn is_valid(self: &Self) -> bool {
        self.expire_time > SystemTime::now() && self.user.upgrade().is_some()
    }

    pub fn from_json(json: &JsonValue, user_db: &UserDB) -> Self {
        Self {
            id: Uuid::parse_str(json["id"].as_str().unwrap()).unwrap(),
            user: user_db.find(json["user_name"].as_str().unwrap()).unwrap(),
            expire_time: UNIX_EPOCH + Duration::from_secs(json["expire_time"].as_u64().unwrap()),
        }
    }

    pub fn to_json(self: &Self) -> Option<JsonValue> {
        if !self.is_valid() { return None; }
        Some(object! {
            id:self.id.to_string(),
            user_name:self.user.upgrade().unwrap().name.clone(),
            expire_time: self.expire_time.duration_since(UNIX_EPOCH).unwrap().as_secs(),
        })
    }
}
