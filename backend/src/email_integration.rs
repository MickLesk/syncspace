use async_imap::Session;
use async_native_tls::TlsStream;
use base64::{engine::general_purpose, Engine as _};
/// Email integration (IMAP/POP3) to fetch emails and attachments
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tokio::net::TcpStream;
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

    // Encrypt password using AES-256 (from encryption module)
    // If encryption module not available, use base64 with clear warning
    let password_encrypted = if let Ok(encrypted) =
        crate::encryption::encrypt_data(req.password.as_bytes())
    {
        format!("enc:{}", general_purpose::STANDARD.encode(&encrypted))
    } else {
        // Fallback: base64 encode (NOT SECURE - for development only)
        tracing::warn!(
            "Email password stored with base64 encoding only - enable encryption in production!"
        );
        format!("b64:{}", general_purpose::STANDARD.encode(&req.password))
    };

    sqlx::query(
        "INSERT INTO email_accounts 
         (id, user_id, name, email_address, protocol, server, port, username, password_encrypted, 
          use_tls, auto_fetch, fetch_interval_minutes, store_attachments, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))",
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

/// Fetch emails from IMAP server
pub async fn fetch_imap_emails(
    account: &EmailAccount,
) -> Result<Vec<EmailMessage>, Box<dyn std::error::Error + Send + Sync>> {
    // Decrypt password
    let password = if account.password_encrypted.starts_with("enc:") {
        let encrypted_data = general_purpose::STANDARD.decode(&account.password_encrypted[4..])?;
        let decrypted = crate::encryption::decrypt_data(&encrypted_data)?;
        String::from_utf8(decrypted)?
    } else if account.password_encrypted.starts_with("b64:") {
        let decoded = general_purpose::STANDARD.decode(&account.password_encrypted[4..])?;
        String::from_utf8(decoded)?
    } else {
        return Err("Invalid password encryption format".into());
    };

    // Connect to IMAP server
    let tls = async_native_tls::TlsConnector::new();
    let client = if account.use_tls {
        async_imap::connect(
            (account.server.as_str(), account.port as u16),
            &account.server,
            &tls,
        )
        .await?
    } else {
        // For non-TLS, connect to plain TCP
        let tcp_stream = TcpStream::connect((account.server.as_str(), account.port as u16)).await?;
        async_imap::Client::new(tcp_stream)
    };

    // Login
    let mut session = client
        .login(&account.username, &password)
        .await
        .map_err(|e| e.0)?;

    // Select INBOX
    session.select("INBOX").await?;

    // Fetch recent messages (last 50)
    let messages_data = session.fetch("1:50", "RFC822").await?;

    let mut email_messages = Vec::new();

    for msg in messages_data.iter() {
        if let Some(body) = msg.body() {
            // Parse email using mailparse crate
            if let Ok(parsed) = mailparse::parse_mail(body) {
                let message_id = parsed
                    .headers
                    .get_first_value("Message-ID")
                    .unwrap_or_else(|| Uuid::new_v4().to_string());
                let subject = parsed
                    .headers
                    .get_first_value("Subject")
                    .unwrap_or_else(|| "No Subject".to_string());
                let from = parsed
                    .headers
                    .get_first_value("From")
                    .unwrap_or_else(|| "Unknown".to_string());
                let to = parsed
                    .headers
                    .get_first_value("To")
                    .unwrap_or_else(|| account.email_address.clone());
                let date = parsed
                    .headers
                    .get_first_value("Date")
                    .unwrap_or_else(|| chrono::Utc::now().to_rfc2822());

                let body_text = parsed.get_body().ok();
                let has_attachments = !parsed.subparts.is_empty();

                email_messages.push(EmailMessage {
                    id: Uuid::new_v4().to_string(),
                    account_id: account.id.clone(),
                    message_id,
                    subject,
                    from_address: from,
                    to_address: to,
                    date,
                    body_text,
                    body_html: None,
                    has_attachments,
                    is_read: false,
                    fetched_at: chrono::Utc::now().to_rfc3339(),
                });
            }
        }
    }

    // Logout
    session.logout().await?;

    Ok(email_messages)
}

/// Fetch emails from POP3 server
pub async fn fetch_pop3_emails(
    account: &EmailAccount,
) -> Result<Vec<EmailMessage>, Box<dyn std::error::Error + Send + Sync>> {
    // Decrypt password
    let password = if account.password_encrypted.starts_with("enc:") {
        let encrypted_data = general_purpose::STANDARD.decode(&account.password_encrypted[4..])?;
        let decrypted = crate::encryption::decrypt_data(&encrypted_data)?;
        String::from_utf8(decrypted)?
    } else if account.password_encrypted.starts_with("b64:") {
        let decoded = general_purpose::STANDARD.decode(&account.password_encrypted[4..])?;
        String::from_utf8(decoded)?
    } else {
        return Err("Invalid password encryption format".into());
    };

    // Connect to POP3 server
    let tcp_stream = TcpStream::connect((account.server.as_str(), account.port as u16)).await?;

    let mut pop3_client = if account.use_tls {
        let tls = async_native_tls::TlsConnector::new();
        let tls_stream = tls.connect(&account.server, tcp_stream).await?;
        async_pop::connect(tls_stream).await?
    } else {
        async_pop::connect(tcp_stream).await?
    };

    // Login
    pop3_client.login(&account.username, &password).await?;

    // Get message count
    let stat = pop3_client.stat().await?;
    let msg_count = stat.0;

    let mut email_messages = Vec::new();

    // Fetch recent messages (last 50 or less)
    let fetch_count = msg_count.min(50);
    for msg_num in 1..=fetch_count {
        if let Ok(msg_data) = pop3_client.retr(msg_num).await {
            if let Ok(parsed) = mailparse::parse_mail(&msg_data) {
                let message_id = parsed
                    .headers
                    .get_first_value("Message-ID")
                    .unwrap_or_else(|| Uuid::new_v4().to_string());
                let subject = parsed
                    .headers
                    .get_first_value("Subject")
                    .unwrap_or_else(|| "No Subject".to_string());
                let from = parsed
                    .headers
                    .get_first_value("From")
                    .unwrap_or_else(|| "Unknown".to_string());
                let to = parsed
                    .headers
                    .get_first_value("To")
                    .unwrap_or_else(|| account.email_address.clone());
                let date = parsed
                    .headers
                    .get_first_value("Date")
                    .unwrap_or_else(|| chrono::Utc::now().to_rfc2822());

                let body_text = parsed.get_body().ok();
                let has_attachments = !parsed.subparts.is_empty();

                email_messages.push(EmailMessage {
                    id: Uuid::new_v4().to_string(),
                    account_id: account.id.clone(),
                    message_id,
                    subject,
                    from_address: from,
                    to_address: to,
                    date,
                    body_text,
                    body_html: None,
                    has_attachments,
                    is_read: false,
                    fetched_at: chrono::Utc::now().to_rfc3339(),
                });
            }
        }
    }

    // Quit
    pop3_client.quit().await?;

    Ok(email_messages)
}

/// List email accounts for user
pub async fn list_email_accounts(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<Vec<EmailAccount>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM email_accounts WHERE user_id = ? ORDER BY created_at DESC")
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
    sqlx::query("DELETE FROM email_accounts WHERE id = ? AND user_id = ?")
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
         VALUES (?, ?, ?, ?, datetime('now'))",
    )
    .bind(Uuid::new_v4().to_string())
    .bind(email_id)
    .bind(&file_id)
    .bind(filename)
    .execute(pool)
    .await?;

    Ok(file_id)
}
