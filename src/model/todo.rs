use uuid::Uuid;

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
}
