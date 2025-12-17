use async_trait::async_trait;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client};
use crate::domain::services::PubSubService;

#[derive(Clone)]
pub struct RedisPubSub {
    client: Client,
    conn_manager: ConnectionManager,
}

impl RedisPubSub {
    pub async fn new(redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::open(redis_url)?;
        let conn_manager = ConnectionManager::new(client.clone()).await?;
        
        Ok(Self {
            client,
            conn_manager,
        })
    }

    pub fn get_connection_manager(&self) -> &ConnectionManager {
        &self.conn_manager
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }
}

#[async_trait]
impl PubSubService for RedisPubSub {
    async fn publish(&self, channel: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.conn_manager.clone();
        conn.publish(channel, message).await?;
        Ok(())
    }

    async fn subscribe(&self, _channel: &str) -> Result<(), Box<dyn std::error::Error>> {
        // This method is not used in this implementation
        // Subscription is handled in the WebSocket handler
        Ok(())
    }

    async fn health_check(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let mut conn = self.conn_manager.clone();
        let _: String = redis::cmd("PING").query_async(&mut conn).await?;
        Ok(true)
    }
}
