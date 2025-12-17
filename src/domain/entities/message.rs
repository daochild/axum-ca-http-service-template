use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub content: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(content: String, user_id: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            user_id,
            created_at: Utc::now(),
        }
    }
}
