use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;
use crate::application::HealthCheckUseCase;

pub async fn health_handler(
    State(health_check): State<Arc<HealthCheckUseCase>>,
) -> (StatusCode, Json<crate::application::HealthStatus>) {
    let health_status = health_check.execute().await;

    let status_code = if health_status.status == "healthy" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status_code, Json(health_status))
}
