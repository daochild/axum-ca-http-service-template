use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use crate::application::HealthCheckUseCase;
use crate::presentation::handlers::{health_handler, websocket_handler, websocket::AppState};

pub fn create_router(
    app_state: AppState,
    health_check: Arc<HealthCheckUseCase>,
) -> Router {
    Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(app_state)
        .route("/health", get(health_handler))
        .with_state(health_check)
        .layer(TraceLayer::new_for_http())
}
