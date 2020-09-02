use weak_table::WeakKeyHashMap;
use std::sync::{Mutex, Arc, Weak};
use crate::model::user::User;
use crate::model::todo::TodoItem;
use uuid::Uuid;
use std::fmt::Formatter;
use std::path::Path;
use std::fs;
use log::info;
use crate::tools::ensure_file_exists;
use crate::database::user_db::UserDB;

#[derive(Debug)]
pub enum TodoDBError {
    UserDoesntExist,
    TodoItemDoesntExist,
}

impl std::fmt::Display for TodoDBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for TodoDBError {}

pub struct TodoDB {
    file_path: String,
    pub map: Mutex<WeakKeyHashMap<Weak<User>, Vec<TodoItem>>>,
}

impl TodoDB {
    pub fn new(file_path: String, user_db: &UserDB) -> Self {
        ensure_file_exists(&file_path, "{}").unwrap();

        let mut map: WeakKeyHashMap<Weak<User>, Vec<TodoItem>> = WeakKeyHashMap::new();
        let json = json::parse(&fs::read_to_string(&file_path).unwrap()).unwrap();
        json.entries()
            .for_each(|(name, todo_json_array)| {
                let user = user_db.find(name).unwrap().upgrade().unwrap();
                let todo_vec = todo_json_array.members()
                    .map(|todo_json| {
                        TodoItem::from_json(todo_json)
                    })
                    .collect::<Vec<TodoItem>>();
                map.insert(user, todo_vec);
            });

        info!("Read TodoDB from {}.", &file_path);
        Self {
            file_path,
            map: Mutex::new(map),
        }
    }

    pub fn save(self: &Self) {
        fs::write(&self.file_path, self.serialize()).unwrap();
        info!("TodoDB saved to {}.", &self.file_path);
    }

    fn serialize(self: &Self) -> String {
        let map = self.map.lock().unwrap();

        let mut json = json::JsonValue::new_object();
        map.iter().for_each(|(user, todos)| {
            let mut json_array = json::JsonValue::new_array();
            todos.iter().for_each(|todo| {
                json_array.push(todo.to_json()).unwrap();
            });
            json.insert(&user.name, json_array).unwrap();
        });

        json.dump()
    }

    pub fn regi_user(self: &Self, user: Arc<User>) {
        let mut map = self.map.lock().unwrap();
        map.insert(user, vec![]);
    }

    pub fn add_todo(self: &Self, user: &User, todo_item: TodoItem) -> Result<(), TodoDBError> {
        match self.map.lock().unwrap().get_mut(user) {
            None => Err(TodoDBError::UserDoesntExist),
            Some(list) => {
                list.push(todo_item);
                Ok(())
            }
        }
    }

    pub fn remove_todo(self: &Self, user: &User, todo_id: Uuid) -> Result<(), TodoDBError> {
        match self.map.lock().unwrap().get_mut(user) {
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
        match self.map.lock().unwrap().get_mut(user) {
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
