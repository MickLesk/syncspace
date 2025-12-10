/// FTP/FTPS synchronization
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose};
use suppaftp::{FtpStream, Mode};
use std::io::Cursor;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FtpConnection {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password_encrypted: String,
    pub use_ftps: bool,
    pub passive_mode: bool,
    pub remote_path: String,
    pub local_path: String,
    pub sync_direction: String, // "upload", "download", "bidirectional"
    pub auto_sync: bool,
    pub sync_interval_minutes: i32,
    pub last_sync_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateFtpConnectionRequest {
    pub name: String,
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password: String,
    pub use_ftps: bool,
    pub passive_mode: bool,
    pub remote_path: String,
    pub local_path: String,
    pub sync_direction: String,
    pub auto_sync: bool,
    pub sync_interval_minutes: i32,
}

/// Add FTP connection
pub async fn add_ftp_connection(
    pool: &SqlitePool,
    user_id: &str,
    req: CreateFtpConnectionRequest,
) -> Result<FtpConnection, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let password_encrypted = general_purpose::STANDARD.encode(&req.password);
    
    sqlx::query(
        "INSERT INTO ftp_connections 
         (id, user_id, name, host, port, username, password_encrypted, use_ftps, passive_mode,
          remote_path, local_path, sync_direction, auto_sync, sync_interval_minutes, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&id)
    .bind(user_id)
    .bind(&req.name)
    .bind(&req.host)
    .bind(req.port)
    .bind(&req.username)
    .bind(&password_encrypted)
    .bind(if req.use_ftps { 1 } else { 0 })
    .bind(if req.passive_mode { 1 } else { 0 })
    .bind(&req.remote_path)
    .bind(&req.local_path)
    .bind(&req.sync_direction)
    .bind(if req.auto_sync { 1 } else { 0 })
    .bind(req.sync_interval_minutes)
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM ftp_connections WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Sync with FTP server
pub async fn sync_ftp(
    connection: &FtpConnection,
) -> Result<SyncResult, Box<dyn std::error::Error + Send + Sync>> {
    let password_bytes = general_purpose::STANDARD.decode(&connection.password_encrypted)?;
    let password = String::from_utf8(password_bytes)?;
    
    // Connect to FTP server
    let mut ftp_stream = FtpStream::connect(format!("{}:{}", connection.host, connection.port))?;
    ftp_stream.login(&connection.username, &password)?;
    
    // Set passive mode if requested
    if connection.passive_mode {
        ftp_stream.set_mode(Mode::Passive);
    } else {
        ftp_stream.set_mode(Mode::Active);
    }
    
    // Change to remote directory
    ftp_stream.cwd(&connection.remote_path)?;
    
    let mut result = SyncResult {
        uploaded: 0,
        downloaded: 0,
        errors: 0,
    };
    
    match connection.sync_direction.as_str() {
        "upload" => {
            // Upload files from local to remote
            if let Ok(entries) = std::fs::read_dir(&connection.local_path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Ok(metadata) = entry.metadata() {
                            if metadata.is_file() {
                                let filename = entry.file_name().to_string_lossy().to_string();
                                match std::fs::read(entry.path()) {
                                    Ok(file_data) => {
                                        let mut cursor = Cursor::new(file_data);
                                        match ftp_stream.put_file(&filename, &mut cursor) {
                                            Ok(_) => result.uploaded += 1,
                                            Err(_) => result.errors += 1,
                                        }
                                    },
                                    Err(_) => result.errors += 1,
                                }
                            }
                        }
                    }
                }
            }
        },
        "download" => {
            // Download files from remote to local
            match ftp_stream.nlst(None) {
                Ok(files) => {
                    for filename in files {
                        match ftp_stream.retr_as_buffer(&filename) {
                            Ok(data) => {
                                let local_path = format!("{}/{}", connection.local_path, filename);
                                match std::fs::write(&local_path, data) {
                                    Ok(_) => result.downloaded += 1,
                                    Err(_) => result.errors += 1,
                                }
                            },
                            Err(_) => result.errors += 1,
                        }
                    }
                },
                Err(_) => result.errors += 1,
            }
        },
        "bidirectional" => {
            // Two-way sync: upload first, then download
            // Upload files
            if let Ok(entries) = std::fs::read_dir(&connection.local_path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Ok(metadata) = entry.metadata() {
                            if metadata.is_file() {
                                let filename = entry.file_name().to_string_lossy().to_string();
                                match std::fs::read(entry.path()) {
                                    Ok(file_data) => {
                                        let mut cursor = Cursor::new(file_data);
                                        match ftp_stream.put_file(&filename, &mut cursor) {
                                            Ok(_) => result.uploaded += 1,
                                            Err(_) => result.errors += 1,
                                        }
                                    },
                                    Err(_) => result.errors += 1,
                                }
                            }
                        }
                    }
                }
            }
            
            // Download files
            match ftp_stream.nlst(None) {
                Ok(files) => {
                    for filename in files {
                        match ftp_stream.retr_as_buffer(&filename) {
                            Ok(data) => {
                                let local_path = format!("{}/{}", connection.local_path, filename);
                                match std::fs::write(&local_path, data) {
                                    Ok(_) => result.downloaded += 1,
                                    Err(_) => result.errors += 1,
                                }
                            },
                            Err(_) => result.errors += 1,
                        }
                    }
                },
                Err(_) => result.errors += 1,
            }
        },
        _ => {
            return Err(format!("Unknown sync direction: {}", connection.sync_direction).into());
        }
    }
    
    // Close connection
    let _ = ftp_stream.quit();
    
    Ok(result)
}

#[derive(Debug, Clone, Serialize)]
pub struct SyncResult {
    pub uploaded: u32,
    pub downloaded: u32,
    pub errors: u32,
}

/// List FTP connections for user
pub async fn list_ftp_connections(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<Vec<FtpConnection>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM ftp_connections WHERE user_id = ? ORDER BY created_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

/// Delete FTP connection
pub async fn delete_ftp_connection(
    pool: &SqlitePool,
    connection_id: &str,
    user_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM ftp_connections WHERE id = ? AND user_id = ?"
    )
    .bind(connection_id)
    .bind(user_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Update last sync time
pub async fn update_last_sync(
    pool: &SqlitePool,
    connection_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE ftp_connections SET last_sync_at = datetime('now') WHERE id = ?"
    )
    .bind(connection_id)
    .execute(pool)
    .await?;
    
    Ok(())
}
