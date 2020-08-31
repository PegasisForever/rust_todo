use uuid::Uuid;
use serde::Deserialize;


#[derive(Debug, Clone, Deserialize)]
pub struct SessionRequest {
    pub session_id: Uuid,
}
