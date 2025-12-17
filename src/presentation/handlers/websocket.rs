use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::Response,
};
use axum::extract::ws::Message as WsMessage;
use futures::{sink::SinkExt, stream::StreamExt};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info};
use crate::application::SendMessageUseCase;
use crate::infrastructure::RedisPubSub;

#[derive(Clone)]
pub struct AppState {
    pub send_message_use_case: Arc<SendMessageUseCase>,
    pub redis_pubsub: Arc<RedisPubSub>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IncomingMessage {
    content: String,
    user_id: String,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| websocket_connection(socket, state))
}

async fn websocket_connection(ws: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = ws.split();

    info!("WebSocket client connected");

    // Create Redis connection for pub/sub
    let redis_client = state.redis_pubsub.get_client();
    let pubsub_conn = match redis_client.get_async_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get Redis connection: {}", e);
            return;
        }
    };
    
    let mut pubsub = pubsub_conn.into_pubsub();
    
    if let Err(e) = pubsub.subscribe("messages").await {
        error!("Failed to subscribe to Redis channel: {}", e);
        return;
    }

    // Spawn task to handle incoming Redis messages
    let mut send_task = tokio::spawn(async move {
        let mut pubsub_stream = pubsub.on_message();
        
        while let Some(msg) = pubsub_stream.next().await {
            let payload: String = match msg.get_payload() {
                Ok(p) => p,
                Err(e) => {
                    error!("Failed to get payload: {}", e);
                    continue;
                }
            };

            if sender.send(WsMessage::Text(payload)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming WebSocket messages
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let WsMessage::Text(text) = msg {
                match serde_json::from_str::<IncomingMessage>(&text) {
                    Ok(incoming) => {
                        match state
                            .send_message_use_case
                            .execute(incoming.content, incoming.user_id)
                            .await
                        {
                            Ok(message) => {
                                info!("Message saved: {:?}", message.id);
                            }
                            Err(e) => {
                                error!("Failed to save message: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse message: {}", e);
                    }
                }
            } else if let WsMessage::Close(_) = msg {
                break;
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = &mut send_task => {
            recv_task.abort();
        }
        _ = &mut recv_task => {
            send_task.abort();
        }
    }

    info!("WebSocket client disconnected");
}
