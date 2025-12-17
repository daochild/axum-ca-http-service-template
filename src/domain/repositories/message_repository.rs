use async_trait::async_trait;
use crate::domain::entities::Message;
use uuid::Uuid;

#[async_trait]
pub trait MessageRepository: Send + Sync {
    async fn save(&self, message: &Message) -> Result<(), Box<dyn std::error::Error>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Message>, Box<dyn std::error::Error>>;
    async fn find_recent(&self, limit: i64) -> Result<Vec<Message>, Box<dyn std::error::Error>>;
    async fn health_check(&self) -> Result<bool, Box<dyn std::error::Error>>;
}
