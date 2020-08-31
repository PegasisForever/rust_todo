use std::sync::Mutex;

#[derive(Debug)]
pub struct User {
    name: String,
    password: String,
}

impl User {
    pub fn new(name: String, password: String) -> User {
        User {
            name,
            password,
        }
    }
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

    pub fn size(self: &UserDB) -> usize {
        self.list.lock().unwrap().len()
    }

    pub fn add(self: &UserDB, user: User) {
        self.list.lock().unwrap().push(user);
    }
}
