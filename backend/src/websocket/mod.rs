//! WebSocket handling module

use axum::{
    extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade},
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::Sender;

use crate::AppState;

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
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state.fs_tx))
}

/// Handle WebSocket connection
async fn handle_socket(socket: WebSocket, tx: Sender<FileChangeEvent>) {
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

    // Spawn task to receive messages from client (heartbeat)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if matches!(msg, Message::Close(_)) {
                break;
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}
