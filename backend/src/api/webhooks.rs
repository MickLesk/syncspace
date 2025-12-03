//! Webhooks API endpoints
//!
//! Provides webhook management for integrations with external services.
//! Webhooks can be triggered on file events (upload, delete, share, etc.)

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use uuid::Uuid;

use crate::{auth::UserInfo, AppState};

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Webhook database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Webhook {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub url: String,
    pub events: String, // JSON array of subscribed events
    pub secret: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_triggered_at: Option<DateTime<Utc>>,
    pub failure_count: i32,
}

/// Webhook delivery log entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WebhookDelivery {
    pub id: String,
    pub webhook_id: String,
    pub event_type: String,
    pub payload: String,
    pub response_status: Option<i32>,
    pub response_body: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
    pub delivered_at: DateTime<Utc>,
    pub duration_ms: i32,
}

/// Create webhook request
#[derive(Debug, Deserialize)]
pub struct CreateWebhookRequest {
    pub name: String,
    pub url: String,
    pub events: Vec<String>,
    pub secret: Option<String>,
}

/// Update webhook request
#[derive(Debug, Deserialize)]
pub struct UpdateWebhookRequest {
    pub name: Option<String>,
    pub url: Option<String>,
    pub events: Option<Vec<String>>,
    pub secret: Option<String>,
    pub is_active: Option<bool>,
}

/// Test webhook request
#[derive(Debug, Deserialize)]
pub struct TestWebhookRequest {
    pub event_type: Option<String>,
}

/// Webhook response with parsed events
#[derive(Debug, Serialize)]
pub struct WebhookResponse {
    pub id: String,
    pub name: String,
    pub url: String,
    pub events: Vec<String>,
    pub has_secret: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_triggered_at: Option<DateTime<Utc>>,
    pub failure_count: i32,
}

impl From<Webhook> for WebhookResponse {
    fn from(w: Webhook) -> Self {
        let events: Vec<String> = serde_json::from_str(&w.events).unwrap_or_default();
        Self {
            id: w.id,
            name: w.name,
            url: w.url,
            events,
            has_secret: w.secret.is_some(),
            is_active: w.is_active,
            created_at: w.created_at,
            last_triggered_at: w.last_triggered_at,
            failure_count: w.failure_count,
        }
    }
}

/// Available webhook events
pub const WEBHOOK_EVENTS: &[(&str, &str)] = &[
    ("file.uploaded", "File uploaded"),
    ("file.downloaded", "File downloaded"),
    ("file.deleted", "File deleted"),
    ("file.moved", "File moved"),
    ("file.renamed", "File renamed"),
    ("file.shared", "File shared"),
    ("file.unshared", "Share removed"),
    ("folder.created", "Folder created"),
    ("folder.deleted", "Folder deleted"),
    ("user.login", "User logged in"),
    ("user.logout", "User logged out"),
    ("user.created", "User created"),
    ("backup.started", "Backup started"),
    ("backup.completed", "Backup completed"),
    ("backup.failed", "Backup failed"),
];

// ============================================================================
// ROUTER
// ============================================================================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/webhooks", get(list_webhooks).post(create_webhook))
        .route("/webhooks/events", get(get_available_events))
        .route(
            "/webhooks/{id}",
            get(get_webhook).put(update_webhook).delete(delete_webhook),
        )
        .route("/webhooks/{id}/test", post(test_webhook))
        .route("/webhooks/{id}/deliveries", get(get_webhook_deliveries))
        .route("/webhooks/{id}/reset-failures", post(reset_failure_count))
}

// ============================================================================
// HANDLERS
// ============================================================================

/// List all webhooks for the current user
async fn list_webhooks(
    user_info: UserInfo,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let webhooks = sqlx::query_as::<_, Webhook>(
        r#"
        SELECT id, user_id, name, url, events, secret, is_active, 
               created_at, last_triggered_at, failure_count
        FROM webhooks 
        WHERE user_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(&user_info.id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to list webhooks: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response: Vec<WebhookResponse> = webhooks.into_iter().map(Into::into).collect();

    Ok(Json(json!({
        "webhooks": response,
        "total": response.len()
    })))
}

/// Get available webhook events
async fn get_available_events() -> Json<serde_json::Value> {
    let events: Vec<serde_json::Value> = WEBHOOK_EVENTS
        .iter()
        .map(|(id, label)| json!({ "id": id, "label": label }))
        .collect();

    Json(json!({ "events": events }))
}

/// Get a specific webhook
async fn get_webhook(
    user_info: UserInfo,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<WebhookResponse>, StatusCode> {
    let webhook = sqlx::query_as::<_, Webhook>(
        r#"
        SELECT id, user_id, name, url, events, secret, is_active,
               created_at, last_triggered_at, failure_count
        FROM webhooks 
        WHERE id = ? AND user_id = ?
        "#,
    )
    .bind(&id)
    .bind(&user_info.id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get webhook: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(webhook.into()))
}

/// Create a new webhook
async fn create_webhook(
    user_info: UserInfo,
    State(state): State<AppState>,
    Json(req): Json<CreateWebhookRequest>,
) -> Result<Json<WebhookResponse>, StatusCode> {
    // Validate URL
    if !req.url.starts_with("http://") && !req.url.starts_with("https://") {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Validate events
    let valid_events: Vec<&str> = WEBHOOK_EVENTS.iter().map(|(id, _)| *id).collect();
    for event in &req.events {
        if !valid_events.contains(&event.as_str()) {
            tracing::warn!("Invalid webhook event: {}", event);
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let id = Uuid::new_v4().to_string();
    let events_json = serde_json::to_string(&req.events).unwrap_or_default();
    let now = Utc::now();

    sqlx::query(
        r#"
        INSERT INTO webhooks (id, user_id, name, url, events, secret, is_active, created_at, failure_count)
        VALUES (?, ?, ?, ?, ?, ?, 1, ?, 0)
        "#
    )
    .bind(&id)
    .bind(&user_info.id)
    .bind(&req.name)
    .bind(&req.url)
    .bind(&events_json)
    .bind(&req.secret)
    .bind(now)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create webhook: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let webhook = Webhook {
        id,
        user_id: user_info.id,
        name: req.name,
        url: req.url,
        events: events_json,
        secret: req.secret,
        is_active: true,
        created_at: now,
        last_triggered_at: None,
        failure_count: 0,
    };

    tracing::info!("Webhook created: {}", webhook.id);
    Ok(Json(webhook.into()))
}

/// Update a webhook
async fn update_webhook(
    user_info: UserInfo,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateWebhookRequest>,
) -> Result<Json<WebhookResponse>, StatusCode> {
    // Verify ownership
    let existing = sqlx::query_as::<_, Webhook>(
        "SELECT id, user_id, name, url, events, secret, is_active, created_at, last_triggered_at, failure_count FROM webhooks WHERE id = ? AND user_id = ?"
    )
    .bind(&id)
    .bind(&user_info.id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch webhook: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Validate URL if provided
    if let Some(ref url) = req.url {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    // Validate events if provided
    if let Some(ref events) = req.events {
        let valid_events: Vec<&str> = WEBHOOK_EVENTS.iter().map(|(id, _)| *id).collect();
        for event in events {
            if !valid_events.contains(&event.as_str()) {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }

    let name = req.name.unwrap_or(existing.name);
    let url = req.url.unwrap_or(existing.url);
    let events_json = req
        .events
        .map(|e| serde_json::to_string(&e).unwrap_or_default())
        .unwrap_or(existing.events);
    let is_active = req.is_active.unwrap_or(existing.is_active);

    // Handle secret: if provided update it, otherwise keep existing
    let secret = if req.secret.is_some() {
        req.secret
    } else {
        existing.secret
    };

    sqlx::query(
        r#"
        UPDATE webhooks 
        SET name = ?, url = ?, events = ?, secret = ?, is_active = ?
        WHERE id = ? AND user_id = ?
        "#,
    )
    .bind(&name)
    .bind(&url)
    .bind(&events_json)
    .bind(&secret)
    .bind(is_active)
    .bind(&id)
    .bind(&user_info.id)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update webhook: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let updated = Webhook {
        id,
        user_id: user_info.id,
        name,
        url,
        events: events_json,
        secret,
        is_active,
        created_at: existing.created_at,
        last_triggered_at: existing.last_triggered_at,
        failure_count: existing.failure_count,
    };

    Ok(Json(updated.into()))
}

/// Delete a webhook
async fn delete_webhook(
    user_info: UserInfo,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // Also delete related deliveries
    sqlx::query("DELETE FROM webhook_deliveries WHERE webhook_id = ?")
        .bind(&id)
        .execute(&state.db_pool)
        .await
        .ok(); // Ignore error if table doesn't exist yet

    let result = sqlx::query("DELETE FROM webhooks WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user_info.id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete webhook: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    tracing::info!("Webhook deleted: {}", id);
    Ok(StatusCode::NO_CONTENT)
}

/// Test a webhook by sending a test payload
async fn test_webhook(
    user_info: UserInfo,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<TestWebhookRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let webhook = sqlx::query_as::<_, Webhook>(
        "SELECT id, user_id, name, url, events, secret, is_active, created_at, last_triggered_at, failure_count FROM webhooks WHERE id = ? AND user_id = ?"
    )
    .bind(&id)
    .bind(&user_info.id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch webhook: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    let event_type = req.event_type.unwrap_or_else(|| "test.ping".to_string());

    // Create test payload
    let payload = json!({
        "event": event_type,
        "timestamp": Utc::now().to_rfc3339(),
        "test": true,
        "data": {
            "message": "This is a test webhook delivery from SyncSpace",
            "webhook_id": webhook.id,
            "webhook_name": webhook.name
        }
    });

    let start = std::time::Instant::now();

    // Send webhook
    let client = reqwest::Client::new();
    let mut request_builder = client
        .post(&webhook.url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "SyncSpace-Webhook/1.0")
        .header("X-Webhook-Event", &event_type)
        .header("X-Webhook-Delivery", Uuid::new_v4().to_string());

    // Add HMAC signature if secret is configured
    if let Some(ref secret) = webhook.secret {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256 = Hmac<Sha256>;

        let payload_str = serde_json::to_string(&payload).unwrap_or_default();
        let mut mac =
            HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
        mac.update(payload_str.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());
        request_builder =
            request_builder.header("X-Webhook-Signature", format!("sha256={}", signature));
    }

    let response = request_builder
        .json(&payload)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await;

    let duration_ms = start.elapsed().as_millis() as i32;

    match response {
        Ok(res) => {
            let status = res.status().as_u16() as i32;
            let success = res.status().is_success();
            let body = res.text().await.ok();

            // Log delivery
            let delivery_id = Uuid::new_v4().to_string();
            let _ = sqlx::query(
                r#"
                INSERT INTO webhook_deliveries (id, webhook_id, event_type, payload, response_status, response_body, success, delivered_at, duration_ms)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#
            )
            .bind(&delivery_id)
            .bind(&webhook.id)
            .bind(&event_type)
            .bind(serde_json::to_string(&payload).unwrap_or_default())
            .bind(status)
            .bind(&body)
            .bind(success)
            .bind(Utc::now())
            .bind(duration_ms)
            .execute(&state.db_pool)
            .await;

            // Update last triggered
            let _ = sqlx::query("UPDATE webhooks SET last_triggered_at = ? WHERE id = ?")
                .bind(Utc::now())
                .bind(&webhook.id)
                .execute(&state.db_pool)
                .await;

            Ok(Json(json!({
                "success": success,
                "status": status,
                "duration_ms": duration_ms,
                "response": body.unwrap_or_default()
            })))
        }
        Err(e) => {
            // Log failed delivery
            let delivery_id = Uuid::new_v4().to_string();
            let _ = sqlx::query(
                r#"
                INSERT INTO webhook_deliveries (id, webhook_id, event_type, payload, success, error_message, delivered_at, duration_ms)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                "#
            )
            .bind(&delivery_id)
            .bind(&webhook.id)
            .bind(&event_type)
            .bind(serde_json::to_string(&payload).unwrap_or_default())
            .bind(false)
            .bind(e.to_string())
            .bind(Utc::now())
            .bind(duration_ms)
            .execute(&state.db_pool)
            .await;

            // Increment failure count
            let _ = sqlx::query("UPDATE webhooks SET failure_count = failure_count + 1, last_triggered_at = ? WHERE id = ?")
                .bind(Utc::now())
                .bind(&webhook.id)
                .execute(&state.db_pool)
                .await;

            Ok(Json(json!({
                "success": false,
                "error": e.to_string(),
                "duration_ms": duration_ms
            })))
        }
    }
}

/// Get webhook delivery history
async fn get_webhook_deliveries(
    user_info: UserInfo,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Verify ownership
    let webhook = sqlx::query_as::<_, Webhook>(
        "SELECT id, user_id, name, url, events, secret, is_active, created_at, last_triggered_at, failure_count FROM webhooks WHERE id = ? AND user_id = ?"
    )
    .bind(&id)
    .bind(&user_info.id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to verify webhook ownership: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    let deliveries = sqlx::query_as::<_, WebhookDelivery>(
        r#"
        SELECT id, webhook_id, event_type, payload, response_status, response_body, 
               success, error_message, delivered_at, duration_ms
        FROM webhook_deliveries 
        WHERE webhook_id = ?
        ORDER BY delivered_at DESC
        LIMIT 100
        "#,
    )
    .bind(&webhook.id)
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "deliveries": deliveries,
        "total": deliveries.len()
    })))
}

/// Reset failure count for a webhook
async fn reset_failure_count(
    user_info: UserInfo,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("UPDATE webhooks SET failure_count = 0 WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user_info.id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to reset failure count: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// WEBHOOK DISPATCHER (for use by other modules)
// ============================================================================

/// Dispatch a webhook event to all subscribed webhooks
pub async fn dispatch_webhook_event(
    state: &AppState,
    user_id: &str,
    event_type: &str,
    data: serde_json::Value,
) {
    // Find all active webhooks subscribed to this event
    let webhooks = match sqlx::query_as::<_, Webhook>(
        r#"
        SELECT id, user_id, name, url, events, secret, is_active, 
               created_at, last_triggered_at, failure_count
        FROM webhooks 
        WHERE user_id = ? AND is_active = 1
        "#,
    )
    .bind(user_id)
    .fetch_all(&state.db_pool)
    .await
    {
        Ok(w) => w,
        Err(e) => {
            tracing::error!("Failed to fetch webhooks for dispatch: {}", e);
            return;
        }
    };

    for webhook in webhooks {
        // Check if webhook is subscribed to this event
        let events: Vec<String> = serde_json::from_str(&webhook.events).unwrap_or_default();
        if !events.contains(&event_type.to_string()) {
            continue;
        }

        // Skip if too many failures (disable after 10 consecutive failures)
        if webhook.failure_count >= 10 {
            tracing::warn!("Webhook {} disabled due to too many failures", webhook.id);
            continue;
        }

        let payload = json!({
            "event": event_type,
            "timestamp": Utc::now().to_rfc3339(),
            "test": false,
            "data": data
        });

        // Spawn async task to send webhook (don't block)
        let state_clone = state.clone();
        let webhook_clone = webhook.clone();
        let payload_clone = payload.clone();
        let event_type_owned = event_type.to_string();

        tokio::spawn(async move {
            send_webhook_delivery(
                &state_clone,
                &webhook_clone,
                &event_type_owned,
                payload_clone,
            )
            .await;
        });
    }
}

/// Send a single webhook delivery
async fn send_webhook_delivery(
    state: &AppState,
    webhook: &Webhook,
    event_type: &str,
    payload: serde_json::Value,
) {
    let start = std::time::Instant::now();
    let delivery_id = Uuid::new_v4().to_string();

    let client = reqwest::Client::new();
    let mut request_builder = client
        .post(&webhook.url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "SyncSpace-Webhook/1.0")
        .header("X-Webhook-Event", event_type)
        .header("X-Webhook-Delivery", &delivery_id);

    // Add HMAC signature if secret is configured
    if let Some(ref secret) = webhook.secret {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256 = Hmac<Sha256>;

        let payload_str = serde_json::to_string(&payload).unwrap_or_default();
        if let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) {
            mac.update(payload_str.as_bytes());
            let signature = hex::encode(mac.finalize().into_bytes());
            request_builder =
                request_builder.header("X-Webhook-Signature", format!("sha256={}", signature));
        }
    }

    let response = request_builder
        .json(&payload)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await;

    let duration_ms = start.elapsed().as_millis() as i32;

    match response {
        Ok(res) => {
            let status = res.status().as_u16() as i32;
            let success = res.status().is_success();
            let body = res.text().await.ok();

            // Log delivery
            let _ = sqlx::query(
                r#"
                INSERT INTO webhook_deliveries (id, webhook_id, event_type, payload, response_status, response_body, success, delivered_at, duration_ms)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#
            )
            .bind(&delivery_id)
            .bind(&webhook.id)
            .bind(event_type)
            .bind(serde_json::to_string(&payload).unwrap_or_default())
            .bind(status)
            .bind(&body)
            .bind(success)
            .bind(Utc::now())
            .bind(duration_ms)
            .execute(&state.db_pool)
            .await;

            // Update webhook
            if success {
                let _ = sqlx::query(
                    "UPDATE webhooks SET last_triggered_at = ?, failure_count = 0 WHERE id = ?",
                )
                .bind(Utc::now())
                .bind(&webhook.id)
                .execute(&state.db_pool)
                .await;
            } else {
                let _ = sqlx::query("UPDATE webhooks SET last_triggered_at = ?, failure_count = failure_count + 1 WHERE id = ?")
                    .bind(Utc::now())
                    .bind(&webhook.id)
                    .execute(&state.db_pool)
                    .await;
            }
        }
        Err(e) => {
            // Log failed delivery
            let _ = sqlx::query(
                r#"
                INSERT INTO webhook_deliveries (id, webhook_id, event_type, payload, success, error_message, delivered_at, duration_ms)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                "#
            )
            .bind(&delivery_id)
            .bind(&webhook.id)
            .bind(event_type)
            .bind(serde_json::to_string(&payload).unwrap_or_default())
            .bind(false)
            .bind(e.to_string())
            .bind(Utc::now())
            .bind(duration_ms)
            .execute(&state.db_pool)
            .await;

            // Increment failure count
            let _ = sqlx::query("UPDATE webhooks SET failure_count = failure_count + 1, last_triggered_at = ? WHERE id = ?")
                .bind(Utc::now())
                .bind(&webhook.id)
                .execute(&state.db_pool)
                .await;

            tracing::warn!("Webhook delivery failed for {}: {}", webhook.id, e);
        }
    }
}
