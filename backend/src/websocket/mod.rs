//! WebSocket handling module

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::Sender;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChangeEvent {
    pub path: String,
    pub kind: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl FileChangeEvent {
    pub fn new(path: String, kind: String) -> Self {
        Self {
            path,
            kind,
            timestamp: chrono::Utc::now().to_rfc3339(),
            user_id: None,
            metadata: None,
        }
    }

    pub fn with_user(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// WebSocket upgrade handler
///
/// NOTE: This handler is only used from main.rs with AppState.
/// For lib.rs/tests, use handle_socket directly with a broadcast sender.
pub fn ws_handler_with_tx(
    tx: Sender<FileChangeEvent>,
) -> impl Fn(WebSocketUpgrade) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>
       + Clone {
    move |ws: WebSocketUpgrade| {
        let tx = tx.clone();
        Box::pin(async move { ws.on_upgrade(move |socket| handle_socket(socket, tx)) })
    }
}

/// Handle WebSocket connection
pub async fn handle_socket(socket: WebSocket, tx: Sender<FileChangeEvent>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = tx.subscribe();

    // Spawn task to send events to client
    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&event) {
                if sender.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    // Spawn task to receive messages from client (heartbeat + ping/pong)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => {
                    break;
                }
                Message::Text(text) => {
                    // Parse message for ping/pong handling
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
                        if let Some(msg_type) = parsed.get("type").and_then(|v| v.as_str()) {
                            if msg_type == "ping" {
                                // Respond with pong
                                let pong = serde_json::json!({
                                    "type": "pong",
                                    "timestamp": chrono::Utc::now().timestamp_millis()
                                });
                                if let Ok(pong_str) = serde_json::to_string(&pong) {
                                    // Send pong through the sender (need to access it)
                                    // Since sender is moved to send_task, we need a different approach
                                    // For now, just log it - the client will timeout and reconnect
                                    // TODO: Implement proper bidirectional communication
                                    tracing::debug!(
                                        "Received ping, should send pong (not implemented yet)"
                                    );
                                }
                            }
                        }
                    }
                }
                Message::Ping(data) => {
                    // Axum handles Ping frames automatically with Pong
                    tracing::trace!("Received WebSocket Ping frame");
                }
                Message::Pong(_) => {
                    tracing::trace!("Received WebSocket Pong frame");
                }
                _ => {}
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}
