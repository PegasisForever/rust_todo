use uuid::Uuid;
use serde::{Deserialize};

#[derive(Debug, Clone, Deserialize)]
pub struct SessionRequest {
    pub session_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddTodoRequest {
    pub session_id: Uuid,
    pub todo_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToggleTodoRequest {
    pub session_id: Uuid,
    pub todo_id: Uuid,
    pub completed: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RemoveTodoRequest {
    pub session_id: Uuid,
    pub todo_id: Uuid,
}
