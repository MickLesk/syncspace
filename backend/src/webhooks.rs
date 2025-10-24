/// Webhook system for external integrations
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use reqwest;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Webhook {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub url: String,
    pub events: String, // JSON array of subscribed events
    pub secret: Option<String>, // For HMAC signature
    pub is_active: bool,
    pub created_at: String,
    pub last_triggered_at: Option<String>,
    pub failure_count: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebhookPayload {
    pub event: String,
    pub timestamp: String,
    pub user_id: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateWebhookRequest {
    pub name: String,
    pub url: String,
    pub events: Vec<String>,
    pub secret: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateWebhookRequest {
    pub name: Option<String>,
    pub url: Option<String>,
    pub events: Option<Vec<String>>,
    pub secret: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebhookEvent {
    pub event_type: String,
    pub user_id: String,
    pub file_path: Option<String>,
    pub metadata: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Create a new webhook
pub async fn create_webhook(
    pool: &SqlitePool,
    user_id: &str,
    req: CreateWebhookRequest,
) -> Result<Webhook, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let events_json = serde_json::to_string(&req.events).unwrap_or_else(|_| "[]".to_string());
    
    sqlx::query(
        "INSERT INTO webhooks (id, user_id, name, url, events, secret, is_active, created_at, failure_count)
         VALUES (?, ?, ?, ?, ?, ?, 1, datetime('now'), 0)"
    )
    .bind(&id)
    .bind(user_id)
    .bind(&req.name)
    .bind(&req.url)
    .bind(&events_json)
    .bind(&req.secret)
    .execute(pool)
    .await?;
    
    // Fetch created webhook
    let webhook = sqlx::query_as::<_, Webhook>(
        "SELECT * FROM webhooks WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(pool)
    .await?;
    
    Ok(webhook)
}

/// List webhooks for a user
pub async fn list_webhooks(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<Vec<Webhook>, sqlx::Error> {
    sqlx::query_as::<_, Webhook>(
        "SELECT * FROM webhooks WHERE user_id = ? ORDER BY created_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

/// Delete webhook
pub async fn delete_webhook(
    pool: &SqlitePool,
    webhook_id: &str,
    user_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM webhooks WHERE id = ? AND user_id = ?"
    )
    .bind(webhook_id)
    .bind(user_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Get a single webhook
pub async fn get_webhook(
    pool: &SqlitePool,
    webhook_id: &str,
    user_id: &str,
) -> Result<Webhook, sqlx::Error> {
    sqlx::query_as::<_, Webhook>(
        "SELECT * FROM webhooks WHERE id = ? AND user_id = ?"
    )
    .bind(webhook_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
}

/// Update webhook
pub async fn update_webhook(
    pool: &SqlitePool,
    webhook_id: &str,
    user_id: &str,
    req: UpdateWebhookRequest,
) -> Result<Webhook, sqlx::Error> {
    // Build dynamic update query
    let mut updates = Vec::new();
    let mut params: Vec<String> = Vec::new();
    
    if let Some(name) = req.name {
        updates.push("name = ?");
        params.push(name);
    }
    if let Some(url) = req.url {
        updates.push("url = ?");
        params.push(url);
    }
    if let Some(events) = req.events {
        updates.push("events = ?");
        params.push(serde_json::to_string(&events).unwrap_or_else(|_| "[]".to_string()));
    }
    if let Some(secret) = req.secret {
        updates.push("secret = ?");
        params.push(secret);
    }
    if let Some(is_active) = req.is_active {
        updates.push("is_active = ?");
        params.push(if is_active { "1" } else { "0" }.to_string());
    }
    
    if !updates.is_empty() {
        let query = format!(
            "UPDATE webhooks SET {} WHERE id = ? AND user_id = ?",
            updates.join(", ")
        );
        
        let mut q = sqlx::query(&query);
        for param in params {
            q = q.bind(param);
        }
        q = q.bind(webhook_id).bind(user_id);
        
        q.execute(pool).await?;
    }
    
    // Return updated webhook
    get_webhook(pool, webhook_id, user_id).await
}

/// Toggle webhook active status
pub async fn toggle_webhook(
    pool: &SqlitePool,
    webhook_id: &str,
    user_id: &str,
    is_active: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE webhooks SET is_active = ? WHERE id = ? AND user_id = ?"
    )
    .bind(is_active)
    .bind(webhook_id)
    .bind(user_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Trigger webhooks for an event
pub async fn trigger_webhooks(
    pool: &SqlitePool,
    event: &str,
    user_id: &str,
    data: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Find all active webhooks that subscribe to this event
    let webhooks: Vec<Webhook> = sqlx::query_as(
        "SELECT * FROM webhooks 
         WHERE user_id = ? 
         AND is_active = 1 
         AND (events LIKE ? OR events LIKE '%\"*\"%')"
    )
    .bind(user_id)
    .bind(format!("%\"{}%", event))
    .fetch_all(pool)
    .await?;
    
    // Trigger each webhook asynchronously
    for webhook in webhooks {
        let pool_clone = pool.clone();
        let webhook_clone = webhook.clone();
        let event_str = event.to_string();
        let user_id_str = user_id.to_string();
        let data_clone = data.clone();
        
        tokio::spawn(async move {
            if let Err(e) = send_webhook_internal(&pool_clone, &webhook_clone, &event_str, &user_id_str, data_clone).await {
                eprintln!("⚠️ Webhook {} failed: {}", webhook_clone.id, e);
            }
        });
    }
    
    Ok(())
}

/// Send a webhook HTTP request
pub async fn send_webhook(
    webhook: &Webhook,
    event: &WebhookEvent,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let payload = serde_json::json!({
        "event": event.event_type,
        "timestamp": event.timestamp.to_rfc3339(),
        "user_id": event.user_id,
        "file_path": event.file_path,
        "metadata": event.metadata,
    });
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    
    let mut request = client.post(&webhook.url)
        .json(&payload);
    
    // Add HMAC signature if secret is set
    if let Some(secret) = &webhook.secret {
        let payload_str = serde_json::to_string(&payload)?;
        let signature = compute_hmac(secret, &payload_str);
        request = request.header("X-Webhook-Signature", signature);
    }
    
    let response = request.send().await?;
    
    if response.status().is_success() {
        println!("✅ Webhook {} triggered successfully", webhook.id);
        Ok(())
    } else {
        Err(format!("Webhook returned status {}", response.status()).into())
    }
}

/// Send a webhook HTTP request (internal version with pool)
async fn send_webhook_internal(
    pool: &SqlitePool,
    webhook: &Webhook,
    event: &str,
    user_id: &str,
    data: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let payload = WebhookPayload {
        event: event.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        user_id: user_id.to_string(),
        data,
    };
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    
    let mut request = client.post(&webhook.url)
        .json(&payload);
    
    // Add HMAC signature if secret is set
    if let Some(secret) = &webhook.secret {
        let payload_str = serde_json::to_string(&payload)?;
        let signature = compute_hmac(secret, &payload_str);
        request = request.header("X-Webhook-Signature", signature);
    }
    
    let response = request.send().await?;
    
    if response.status().is_success() {
        // Update last_triggered_at and reset failure_count
        let _ = sqlx::query(
            "UPDATE webhooks SET last_triggered_at = datetime('now'), failure_count = 0 WHERE id = ?"
        )
        .bind(&webhook.id)
        .execute(pool)
        .await;
        
        println!("✅ Webhook {} triggered successfully", webhook.id);
    } else {
        // Increment failure count
        let _ = sqlx::query(
            "UPDATE webhooks SET failure_count = failure_count + 1 WHERE id = ?"
        )
        .bind(&webhook.id)
        .execute(pool)
        .await;
        
        return Err(format!("Webhook returned status {}", response.status()).into());
    }
    
    Ok(())
}

/// Compute HMAC-SHA256 signature
fn compute_hmac(secret: &str, payload: &str) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    
    type HmacSha256 = Hmac<Sha256>;
    
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(payload.as_bytes());
    
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

/// Test webhook by sending a ping
pub async fn test_webhook(
    webhook: &Webhook,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let payload = WebhookPayload {
        event: "ping".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        user_id: webhook.user_id.clone(),
        data: serde_json::json!({ "message": "Webhook test ping" }),
    };
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    
    let response = client.post(&webhook.url)
        .json(&payload)
        .send()
        .await?;
    
    Ok(format!("Status: {}", response.status()))
}
