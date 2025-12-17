use async_trait::async_trait;

#[async_trait]
pub trait PubSubService: Send + Sync {
    async fn publish(&self, channel: &str, message: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn subscribe(&self, channel: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn health_check(&self) -> Result<bool, Box<dyn std::error::Error>>;
}
