use std::sync::Arc;
use crate::domain::entities::Message;
use crate::domain::repositories::MessageRepository;
use crate::domain::services::PubSubService;

pub struct SendMessageUseCase {
    message_repository: Arc<dyn MessageRepository>,
    pubsub_service: Arc<dyn PubSubService>,
}

impl SendMessageUseCase {
    pub fn new(
        message_repository: Arc<dyn MessageRepository>,
        pubsub_service: Arc<dyn PubSubService>,
    ) -> Self {
        Self {
            message_repository,
            pubsub_service,
        }
    }

    pub async fn execute(
        &self,
        content: String,
        user_id: String,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let message = Message::new(content, user_id);

        // Save to database
        self.message_repository.save(&message).await?;

        // Publish to Redis
        let message_json = serde_json::to_string(&message)?;
        self.pubsub_service.publish("messages", &message_json).await?;

        Ok(message)
    }
}
