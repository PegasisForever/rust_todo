use weak_table::WeakKeyHashMap;
use std::sync::{Mutex, Arc, Weak};
use crate::model::user::User;
use crate::model::todo::TodoItem;
use uuid::Uuid;

pub enum TodoDBError {
    UserDoesntExist,
    TodoItemDoesntExist,
}

pub struct TodoDB {
    pub list: Mutex<WeakKeyHashMap<Weak<User>, Vec<TodoItem>>>
}

impl TodoDB {
    pub fn new() -> Self {
        Self {
            list: Mutex::new(WeakKeyHashMap::new()),
        }
    }

    pub fn regi_user(self: &Self, user: Arc<User>) {
        let mut list = self.list.lock().unwrap();
        list.insert(user, vec![]);
    }

    pub fn add_todo(self: &Self, user: &User, todo_item: TodoItem) -> Result<(), TodoDBError> {
        match self.list.lock().unwrap().get_mut(user) {
            None => Err(TodoDBError::UserDoesntExist),
            Some(list) => {
                list.push(todo_item);
                Ok(())
            }
        }
    }

    pub fn remove_todo(self: &Self, user: &User, todo_id: Uuid) -> Result<(), TodoDBError> {
        match self.list.lock().unwrap().get_mut(user) {
            None => Err(TodoDBError::UserDoesntExist),
            Some(list) => {
                if let Some(i) = list.iter().position(|item| item.id == todo_id) {
                    list.remove(i);
                    Ok(())
                } else {
                    Err(TodoDBError::TodoItemDoesntExist)
                }
            }
        }
    }

    pub fn toggle_todo(self: &Self, user: &User, todo_id: Uuid, completed: bool) -> Result<(), TodoDBError> {
        match self.list.lock().unwrap().get_mut(user) {
            None => Err(TodoDBError::UserDoesntExist),
            Some(list) => {
                if let Some(i) = list.iter().position(|item| item.id == todo_id) {
                    list[i].completed = completed;
                    Ok(())
                } else {
                    Err(TodoDBError::TodoItemDoesntExist)
                }
            }
        }
    }
}
