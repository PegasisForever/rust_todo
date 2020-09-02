use uuid::Uuid;
use json::{object, JsonValue};

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub id: Uuid,
    pub name: String,
    pub completed: bool,
}

impl TodoItem {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            completed: false,
        }
    }

    pub fn from_json(json: &JsonValue) -> Self {
        Self {
            id: uuid::Uuid::parse_str(json["id"].as_str().unwrap()).unwrap(),
            name: String::from(json["name"].as_str().unwrap()),
            completed: json["completed"].as_bool().unwrap(),
        }
    }

    pub fn to_json(self: &Self) -> JsonValue {
        object! {
            id: self.id.to_string(),
            name: self.name.clone(),
            completed: self.completed
        }
    }
}
