use std::sync::Weak;
use crate::model::user::User;
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub user: Weak<User>,
    pub expire_time: Instant,
}

const EXPIRE_DURATION: Duration = Duration::from_secs(60 * 10);

impl Session {
    pub fn new(user: Weak<User>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user,
            expire_time: Instant::now() + EXPIRE_DURATION,
        }
    }

    pub fn is_valid(self: &Self) -> bool {
        self.expire_time > Instant::now() && self.user.upgrade().is_some()
    }
}
