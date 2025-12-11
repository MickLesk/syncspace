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
#[serde(tag = "event_type")]
pub enum FileChangeEvent {
    #[serde(rename = "file_change")]
    FileChange {
        path: String,
        kind: String,
        timestamp: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        user_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        metadata: Option<serde_json::Value>,
    },
    #[serde(rename = "conversion_progress")]
    ConversionProgress {
        job_id: String,
        status: String,
        progress: Option<f32>,
        timestamp: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        error_message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_path: Option<String>,
    },
}

impl FileChangeEvent {
    pub fn new(path: String, kind: String) -> Self {
        Self::FileChange {
            path,
            kind,
            timestamp: chrono::Utc::now().to_rfc3339(),
            user_id: None,
            metadata: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_user(self, user_id: String) -> Self {
        match self {
            Self::FileChange { path, kind, timestamp, metadata, .. } => {
                Self::FileChange {
                    path,
                    kind,
                    timestamp,
                    user_id: Some(user_id),
                    metadata,
                }
            }
            other => other,
        }
    }

    #[allow(dead_code)]
    pub fn with_metadata(self, new_metadata: serde_json::Value) -> Self {
        match self {
            Self::FileChange { path, kind, timestamp, user_id, .. } => {
                Self::FileChange {
                    path,
                    kind,
                    timestamp,
                    user_id,
                    metadata: Some(new_metadata),
                }
            }
            other => other,
        }
    }

    pub fn conversion_progress(
        job_id: String,
        status: String,
        progress: Option<f32>,
    ) -> Self {
        Self::ConversionProgress {
            job_id,
            status,
            progress,
            timestamp: chrono::Utc::now().to_rfc3339(),
            error_message: None,
            output_path: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_error(self, error: String) -> Self {
        match self {
            Self::ConversionProgress { job_id, status, progress, timestamp, output_path, .. } => {
                Self::ConversionProgress {
                    job_id,
                    status,
                    progress,
                    timestamp,
                    error_message: Some(error),
                    output_path,
                }
            }
            other => other,
        }
    }

    #[allow(dead_code)]
    pub fn with_output(self, path: String) -> Self {
        match self {
            Self::ConversionProgress { job_id, status, progress, timestamp, error_message, .. } => {
                Self::ConversionProgress {
                    job_id,
                    status,
                    progress,
                    timestamp,
                    error_message,
                    output_path: Some(path),
                }
            }
            other => other,
        }
    }
}

/// WebSocket upgrade handler
///
/// NOTE: This handler is only used from main.rs with AppState.
/// For lib.rs/tests, use handle_socket directly with a broadcast sender.
#[allow(dead_code)]
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
            if let Ok(json) = serde_json::to_string(&event)
                && sender.send(Message::Text(json.into())).await.is_err() {
                    break;
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
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text)
                        && let Some(msg_type) = parsed.get("type").and_then(|v| v.as_str()) {
                            match msg_type {
                                "ping" => {
                                    tracing::debug!("Received ping, responding with pong");
                                    // Pong response will be sent via broadcast channel
                                }
                                "subscribe" => {
                                    // Subscribe to specific file/directory changes
                                    if let Some(path) = parsed.get("path").and_then(|v| v.as_str()) {
                                        tracing::info!("Client subscribed to path: {}", path);
                                        // Track subscription in memory or persist
                                    }
                                }
                                "unsubscribe" => {
                                    // Unsubscribe from specific file/directory changes
                                    if let Some(path) = parsed.get("path").and_then(|v| v.as_str()) {
                                        tracing::info!("Client unsubscribed from path: {}", path);
                                        // Remove subscription tracking
                                    }
                                }
                                "get_status" => {
                                    // Return current connection status
                                    tracing::debug!("Client requested status");
                                    // Status would be sent via broadcast channel with server events
                                }
                                _ => {
                                    tracing::warn!("Unknown message type: {}", msg_type);
                                }
                            }
                        }
                }
                Message::Ping(_data) => {
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
