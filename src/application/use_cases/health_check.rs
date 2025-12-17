use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::domain::repositories::MessageRepository;
use crate::domain::services::PubSubService;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub services: ServicesHealth,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServicesHealth {
    pub postgres: ServiceStatus,
    pub redis: ServiceStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    Up,
    Down,
}

pub struct HealthCheckUseCase {
    message_repository: Arc<dyn MessageRepository>,
    pubsub_service: Arc<dyn PubSubService>,
}

impl HealthCheckUseCase {
    pub fn new(
        message_repository: Arc<dyn MessageRepository>,
        pubsub_service: Arc<dyn PubSubService>,
    ) -> Self {
        Self {
            message_repository,
            pubsub_service,
        }
    }

    pub async fn execute(&self) -> HealthStatus {
        let postgres_status = match self.message_repository.health_check().await {
            Ok(true) => ServiceStatus::Up,
            _ => ServiceStatus::Down,
        };

        let redis_status = match self.pubsub_service.health_check().await {
            Ok(true) => ServiceStatus::Up,
            _ => ServiceStatus::Down,
        };

        let overall_status = match (&postgres_status, &redis_status) {
            (ServiceStatus::Up, ServiceStatus::Up) => "healthy",
            _ => "unhealthy",
        };

        HealthStatus {
            status: overall_status.to_string(),
            services: ServicesHealth {
                postgres: postgres_status,
                redis: redis_status,
            },
        }
    }
}
