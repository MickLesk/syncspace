//! Email Integration Module
//! Fetches emails via IMAP/POP3 and stores attachments in SyncSpace

use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

/// Email account configuration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmailAccount {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub email_address: String,
    pub protocol: String, // "imap" or "pop3"
    pub server: String,
    pub port: i32,
    pub username: String,
    pub password_encrypted: String,
    pub use_tls: bool,
    pub auto_fetch: bool,
    pub fetch_interval_minutes: i32,
    pub store_attachments: bool,
    pub attachment_folder: Option<String>,
    pub last_fetch_at: Option<String>,
    pub last_fetch_status: Option<String>,
    pub last_fetch_error: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to create email account
#[derive(Debug, Clone, Deserialize)]
pub struct CreateEmailAccountRequest {
    pub name: String,
    pub email_address: String,
    pub protocol: String,
    pub server: String,
    pub port: i32,
    pub username: String,
    pub password: String,
    pub use_tls: bool,
    pub auto_fetch: bool,
    pub fetch_interval_minutes: i32,
    pub store_attachments: bool,
    pub attachment_folder: Option<String>,
}

/// Request to update email account
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateEmailAccountRequest {
    pub name: Option<String>,
    pub email_address: Option<String>,
    pub protocol: Option<String>,
    pub server: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub use_tls: Option<bool>,
    pub auto_fetch: Option<bool>,
    pub fetch_interval_minutes: Option<i32>,
    pub store_attachments: Option<bool>,
    pub attachment_folder: Option<String>,
}

/// Email message
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmailMessage {
    pub id: String,
    pub account_id: String,
    pub message_id: String,
    pub subject: Option<String>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub date: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub has_attachments: bool,
    pub is_read: bool,
    pub fetched_at: String,
}

/// Email attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAttachment {
    pub id: String,
    pub email_id: String,
    pub file_id: String,
    pub filename: String,
    pub content_type: Option<String>,
    pub size_bytes: Option<i64>,
}

/// Fetch result summary
#[derive(Debug, Clone, Serialize)]
pub struct FetchResult {
    pub account_id: String,
    pub messages_fetched: u32,
    pub attachments_saved: u32,
    pub errors: u32,
    pub error_messages: Vec<String>,
    pub started_at: String,
    pub completed_at: String,
}

/// Create email account
pub async fn create_email_account(
    pool: &SqlitePool,
    user_id: &str,
    req: CreateEmailAccountRequest,
) -> Result<EmailAccount, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let password_encrypted = general_purpose::STANDARD.encode(&req.password);

    sqlx::query(
        r#"INSERT INTO email_accounts 
           (id, user_id, name, email_address, protocol, server, port, username, password_encrypted,
            use_tls, auto_fetch, fetch_interval_minutes, store_attachments, attachment_folder, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"#,
    )
    .bind(&id)
    .bind(user_id)
    .bind(&req.name)
    .bind(&req.email_address)
    .bind(&req.protocol)
    .bind(&req.server)
    .bind(req.port)
    .bind(&req.username)
    .bind(&password_encrypted)
    .bind(if req.use_tls { 1 } else { 0 })
    .bind(if req.auto_fetch { 1 } else { 0 })
    .bind(req.fetch_interval_minutes)
    .bind(if req.store_attachments { 1 } else { 0 })
    .bind(req.attachment_folder.unwrap_or_else(|| "/email_attachments".to_string()))
    .execute(pool)
    .await?;

    get_email_account(pool, &id, user_id).await
}

/// Get email account by ID
pub async fn get_email_account(
    pool: &SqlitePool,
    id: &str,
    user_id: &str,
) -> Result<EmailAccount, sqlx::Error> {
    sqlx::query_as("SELECT * FROM email_accounts WHERE id = ? AND user_id = ?")
        .bind(id)
        .bind(user_id)
        .fetch_one(pool)
        .await
}

/// List all email accounts for user
pub async fn list_email_accounts(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<Vec<EmailAccount>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM email_accounts WHERE user_id = ? ORDER BY created_at DESC")
        .bind(user_id)
        .fetch_all(pool)
        .await
}

/// Update email account
pub async fn update_email_account(
    pool: &SqlitePool,
    id: &str,
    user_id: &str,
    req: UpdateEmailAccountRequest,
) -> Result<EmailAccount, sqlx::Error> {
    let account = get_email_account(pool, id, user_id).await?;

    let password_encrypted = req
        .password
        .map(|p| general_purpose::STANDARD.encode(&p))
        .unwrap_or(account.password_encrypted);

    sqlx::query(
        r#"UPDATE email_accounts SET
           name = ?, email_address = ?, protocol = ?, server = ?, port = ?,
           username = ?, password_encrypted = ?, use_tls = ?, auto_fetch = ?,
           fetch_interval_minutes = ?, store_attachments = ?, attachment_folder = ?,
           updated_at = datetime('now')
           WHERE id = ? AND user_id = ?"#,
    )
    .bind(req.name.unwrap_or(account.name))
    .bind(req.email_address.unwrap_or(account.email_address))
    .bind(req.protocol.unwrap_or(account.protocol))
    .bind(req.server.unwrap_or(account.server))
    .bind(req.port.unwrap_or(account.port))
    .bind(req.username.unwrap_or(account.username))
    .bind(&password_encrypted)
    .bind(if req.use_tls.unwrap_or(account.use_tls) { 1 } else { 0 })
    .bind(if req.auto_fetch.unwrap_or(account.auto_fetch) { 1 } else { 0 })
    .bind(req.fetch_interval_minutes.unwrap_or(account.fetch_interval_minutes))
    .bind(if req.store_attachments.unwrap_or(account.store_attachments) { 1 } else { 0 })
    .bind(req.attachment_folder.or(account.attachment_folder))
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;

    get_email_account(pool, id, user_id).await
}

/// Delete email account
pub async fn delete_email_account(
    pool: &SqlitePool,
    id: &str,
    user_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM email_accounts WHERE id = ? AND user_id = ?")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Test email connection
pub async fn test_email_connection(
    account: &EmailAccount,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let password = general_purpose::STANDARD.decode(&account.password_encrypted)?;
    let _password = String::from_utf8(password)?;

    // For now, just validate the configuration
    // Full IMAP/POP3 testing requires async-imap which has complex dependencies
    
    if account.server.is_empty() {
        return Err("Server address is required".into());
    }
    
    if account.port <= 0 || account.port > 65535 {
        return Err("Invalid port number".into());
    }

    if !["imap", "pop3"].contains(&account.protocol.to_lowercase().as_str()) {
        return Err("Protocol must be 'imap' or 'pop3'".into());
    }

    // TODO: Implement actual connection test with async-imap/async-pop
    Ok(true)
}

/// Fetch emails from account
pub async fn fetch_emails(
    pool: &SqlitePool,
    account: &EmailAccount,
) -> Result<FetchResult, Box<dyn std::error::Error + Send + Sync>> {
    let started_at = chrono::Utc::now().to_rfc3339();
    
    let mut result = FetchResult {
        account_id: account.id.clone(),
        messages_fetched: 0,
        attachments_saved: 0,
        errors: 0,
        error_messages: Vec::new(),
        started_at,
        completed_at: String::new(),
    };

    // Note: Full IMAP implementation requires async-imap crate
    // This is a placeholder that shows the structure
    
    result.error_messages.push(
        "Email fetching is not yet fully implemented. Requires async-imap integration.".to_string()
    );
    result.errors += 1;

    result.completed_at = chrono::Utc::now().to_rfc3339();

    // Update fetch status
    sqlx::query(
        "UPDATE email_accounts SET last_fetch_at = datetime('now'), last_fetch_status = ?, last_fetch_error = ? WHERE id = ?",
    )
    .bind(if result.errors == 0 { "success" } else { "error" })
    .bind(result.error_messages.first())
    .bind(&account.id)
    .execute(pool)
    .await?;

    Ok(result)
}

/// Get messages for account
pub async fn get_messages(
    pool: &SqlitePool,
    account_id: &str,
    limit: i32,
    offset: i32,
) -> Result<Vec<EmailMessage>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM email_messages WHERE account_id = ? ORDER BY fetched_at DESC LIMIT ? OFFSET ?",
    )
    .bind(account_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

/// Mark message as read
pub async fn mark_as_read(
    pool: &SqlitePool,
    message_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE email_messages SET is_read = 1 WHERE id = ?")
        .bind(message_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete message
pub async fn delete_message(
    pool: &SqlitePool,
    message_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM email_messages WHERE id = ?")
        .bind(message_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Store email attachment as file
pub async fn store_attachment(
    pool: &SqlitePool,
    user_id: &str,
    email_id: &str,
    filename: &str,
    content_type: Option<&str>,
    data: &[u8],
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let file_id = Uuid::new_v4().to_string();
    let storage_path = format!("./data/email_attachments/{}/{}", user_id, file_id);

    // Create directory
    tokio::fs::create_dir_all(format!("./data/email_attachments/{}", user_id)).await?;

    // Write file
    tokio::fs::write(&storage_path, data).await?;

    // Store in database
    sqlx::query(
        r#"INSERT INTO email_attachments (id, email_id, file_id, filename, content_type, size_bytes, created_at)
           VALUES (?, ?, ?, ?, ?, ?, datetime('now'))"#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind(email_id)
    .bind(&file_id)
    .bind(filename)
    .bind(content_type)
    .bind(data.len() as i64)
    .execute(pool)
    .await?;

    Ok(file_id)
}
