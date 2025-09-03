use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Event {
    pub id: i32,
    pub value: Option<String>,
}

impl Event {
    pub fn new(id: i32, value: Option<String>) -> Self {
        Self { id, value }
    }
}
