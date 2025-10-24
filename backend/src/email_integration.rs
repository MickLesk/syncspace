/// Email integration (IMAP/POP3) to fetch emails and attachments
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

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
    pub last_fetch_at: Option<String>,
    pub created_at: String,
}

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailMessage {
    pub id: String,
    pub account_id: String,
    pub message_id: String,
    pub subject: String,
    pub from_address: String,
    pub to_address: String,
    pub date: String,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub has_attachments: bool,
    pub is_read: bool,
    pub fetched_at: String,
}

/// Add email account
pub async fn add_email_account(
    pool: &SqlitePool,
    user_id: &str,
    req: CreateEmailAccountRequest,
) -> Result<EmailAccount, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    
    // TODO: Encrypt password properly (using encryption module)
    let password_encrypted = base64::encode(&req.password);
    
    sqlx::query(
        "INSERT INTO email_accounts 
         (id, user_id, name, email_address, protocol, server, port, username, password_encrypted, 
          use_tls, auto_fetch, fetch_interval_minutes, store_attachments, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))"
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
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM email_accounts WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Fetch emails from IMAP server (placeholder - requires imap crate)
pub async fn fetch_imap_emails(
    _account: &EmailAccount,
) -> Result<Vec<EmailMessage>, Box<dyn std::error::Error + Send + Sync>> {
    // Placeholder implementation
    // In production: use `async-imap` or `imap` crate
    /*
    Example with async-imap:
    
    let tls = async_native_tls::TlsConnector::new();
    let client = async_imap::connect((account.server.as_str(), account.port as u16), &account.server, &tls).await?;
    
    let mut session = client
        .login(&account.username, &password)
        .await
        .map_err(|e| e.0)?;
    
    session.select("INBOX").await?;
    
    let messages = session.fetch("1:10", "RFC822").await?;
    
    for msg in messages.iter() {
        // Parse email with mail-parser crate
        // Extract attachments
        // Store in database
    }
    */
    
    Ok(Vec::new())
}

/// Fetch emails from POP3 server (placeholder - requires pop3 crate)
pub async fn fetch_pop3_emails(
    _account: &EmailAccount,
) -> Result<Vec<EmailMessage>, Box<dyn std::error::Error + Send + Sync>> {
    // Placeholder implementation
    // In production: use `async-pop` or similar crate
    
    Ok(Vec::new())
}

/// List email accounts for user
pub async fn list_email_accounts(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<Vec<EmailAccount>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM email_accounts WHERE user_id = ? ORDER BY created_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

/// Delete email account
pub async fn delete_email_account(
    pool: &SqlitePool,
    account_id: &str,
    user_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM email_accounts WHERE id = ? AND user_id = ?"
    )
    .bind(account_id)
    .bind(user_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Store email attachment as file
pub async fn store_email_attachment(
    pool: &SqlitePool,
    user_id: &str,
    email_id: &str,
    filename: &str,
    data: &[u8],
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let file_id = Uuid::new_v4().to_string();
    let storage_path = format!("./data/email_attachments/{}/{}", user_id, file_id);
    
    // Create directory
    tokio::fs::create_dir_all(format!("./data/email_attachments/{}", user_id)).await?;
    
    // Write file
    tokio::fs::write(&storage_path, data).await?;
    
    // Store metadata in database
    sqlx::query(
        "INSERT INTO files (id, name, path, owner_id, size_bytes, storage_path, mime_type, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
    )
    .bind(&file_id)
    .bind(filename)
    .bind(format!("/email_attachments/{}", filename))
    .bind(user_id)
    .bind(data.len() as i64)
    .bind(&storage_path)
    .bind("application/octet-stream")
    .execute(pool)
    .await?;
    
    // Link to email
    sqlx::query(
        "INSERT INTO email_attachments (id, email_id, file_id, filename, created_at)
         VALUES (?, ?, ?, ?, datetime('now'))"
    )
    .bind(Uuid::new_v4().to_string())
    .bind(email_id)
    .bind(&file_id)
    .bind(filename)
    .execute(pool)
    .await?;
    
    Ok(file_id)
}
