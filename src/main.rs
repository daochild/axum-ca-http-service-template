mod domain;
mod infrastructure;
mod application;
mod presentation;

use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_subscriber;

use infrastructure::{Config, PostgresMessageRepository, RedisPubSub};
use application::{SendMessageUseCase, HealthCheckUseCase};
use presentation::handlers::websocket::AppState;
use presentation::create_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("Starting WebSocket server...");

    // Load configuration
    let config = Config::from_env()?;
    info!("Configuration loaded");

    // Initialize PostgreSQL connection pool
    info!("Connecting to PostgreSQL at {}...", config.database_url);
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;
    info!("PostgreSQL connection established");

    // Run migrations
    info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await?;
    info!("Database migrations completed");

    // Initialize Redis
    info!("Connecting to Redis at {}...", config.redis_url);
    let redis_pubsub = Arc::new(RedisPubSub::new(&config.redis_url).await?);
    info!("Redis connection established");

    // Initialize repositories and services
    let message_repository = Arc::new(PostgresMessageRepository::new(db_pool.clone()));

    // Initialize use cases
    let send_message_use_case = Arc::new(SendMessageUseCase::new(
        message_repository.clone(),
        redis_pubsub.clone(),
    ));

    let health_check_use_case = Arc::new(HealthCheckUseCase::new(
        message_repository.clone(),
        redis_pubsub.clone(),
    ));

    // Create application state
    let app_state = AppState {
        send_message_use_case: send_message_use_case.clone(),
        redis_pubsub: redis_pubsub.clone(),
    };

    // Create router
    let app = create_router(app_state, health_check_use_case);

    // Start server
    let addr = format!("{}:{}", config.server_host, config.server_port);
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}