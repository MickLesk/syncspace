//! FTP/FTPS Synchronization Module
//! Syncs files between SyncSpace and external FTP servers

use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::io::Cursor;
use std::path::PathBuf;
use suppaftp::{FtpStream, Mode};
use tokio::fs;
use uuid::Uuid;

/// FTP connection configuration
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
    pub last_sync_status: Option<String>,
    pub last_sync_error: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to create FTP connection
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

/// Request to update FTP connection
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateFtpConnectionRequest {
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub use_ftps: Option<bool>,
    pub passive_mode: Option<bool>,
    pub remote_path: Option<String>,
    pub local_path: Option<String>,
    pub sync_direction: Option<String>,
    pub auto_sync: Option<bool>,
    pub sync_interval_minutes: Option<i32>,
}

/// Sync result summary
#[derive(Debug, Clone, Serialize)]
pub struct SyncResult {
    pub connection_id: String,
    pub uploaded: u32,
    pub downloaded: u32,
    pub errors: u32,
    pub error_messages: Vec<String>,
    pub started_at: String,
    pub completed_at: String,
    pub duration_ms: u64,
}

/// FTP file entry
#[derive(Debug, Clone, Serialize)]
pub struct FtpFileEntry {
    pub name: String,
    pub is_directory: bool,
    pub size: u64,
    pub modified: Option<String>,
}

/// Create a new FTP connection
pub async fn create_ftp_connection(
    pool: &SqlitePool,
    user_id: &str,
    req: CreateFtpConnectionRequest,
) -> Result<FtpConnection, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let password_encrypted = general_purpose::STANDARD.encode(&req.password);

    sqlx::query(
        r#"INSERT INTO ftp_connections 
           (id, user_id, name, host, port, username, password_encrypted, use_ftps, passive_mode,
            remote_path, local_path, sync_direction, auto_sync, sync_interval_minutes, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"#,
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

    get_ftp_connection(pool, &id, user_id).await
}

/// Get FTP connection by ID
pub async fn get_ftp_connection(
    pool: &SqlitePool,
    id: &str,
    user_id: &str,
) -> Result<FtpConnection, sqlx::Error> {
    sqlx::query_as("SELECT * FROM ftp_connections WHERE id = ? AND user_id = ?")
        .bind(id)
        .bind(user_id)
        .fetch_one(pool)
        .await
}

/// List all FTP connections for user
pub async fn list_ftp_connections(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<Vec<FtpConnection>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM ftp_connections WHERE user_id = ? ORDER BY created_at DESC")
        .bind(user_id)
        .fetch_all(pool)
        .await
}

/// Update FTP connection
pub async fn update_ftp_connection(
    pool: &SqlitePool,
    id: &str,
    user_id: &str,
    req: UpdateFtpConnectionRequest,
) -> Result<FtpConnection, sqlx::Error> {
    let conn = get_ftp_connection(pool, id, user_id).await?;

    let password_encrypted = req
        .password
        .map(|p| general_purpose::STANDARD.encode(&p))
        .unwrap_or(conn.password_encrypted);

    sqlx::query(
        r#"UPDATE ftp_connections SET
           name = ?, host = ?, port = ?, username = ?, password_encrypted = ?,
           use_ftps = ?, passive_mode = ?, remote_path = ?, local_path = ?,
           sync_direction = ?, auto_sync = ?, sync_interval_minutes = ?, updated_at = datetime('now')
           WHERE id = ? AND user_id = ?"#,
    )
    .bind(req.name.unwrap_or(conn.name))
    .bind(req.host.unwrap_or(conn.host))
    .bind(req.port.unwrap_or(conn.port))
    .bind(req.username.unwrap_or(conn.username))
    .bind(&password_encrypted)
    .bind(if req.use_ftps.unwrap_or(conn.use_ftps) { 1 } else { 0 })
    .bind(if req.passive_mode.unwrap_or(conn.passive_mode) { 1 } else { 0 })
    .bind(req.remote_path.unwrap_or(conn.remote_path))
    .bind(req.local_path.unwrap_or(conn.local_path))
    .bind(req.sync_direction.unwrap_or(conn.sync_direction))
    .bind(if req.auto_sync.unwrap_or(conn.auto_sync) { 1 } else { 0 })
    .bind(req.sync_interval_minutes.unwrap_or(conn.sync_interval_minutes))
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;

    get_ftp_connection(pool, id, user_id).await
}

/// Delete FTP connection
pub async fn delete_ftp_connection(
    pool: &SqlitePool,
    id: &str,
    user_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM ftp_connections WHERE id = ? AND user_id = ?")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Test FTP connection
pub async fn test_ftp_connection(
    connection: &FtpConnection,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let password = general_purpose::STANDARD.decode(&connection.password_encrypted)?;
    let password = String::from_utf8(password)?;

    let mut ftp_stream =
        FtpStream::connect(format!("{}:{}", connection.host, connection.port))?;
    ftp_stream.login(&connection.username, &password)?;

    if connection.passive_mode {
        ftp_stream.set_mode(Mode::Passive);
    } else {
        ftp_stream.set_mode(Mode::Active);
    }

    // Try to change to remote path
    ftp_stream.cwd(&connection.remote_path)?;

    let _ = ftp_stream.quit();
    Ok(true)
}

/// List files on FTP server
pub async fn list_remote_files(
    connection: &FtpConnection,
    path: Option<&str>,
) -> Result<Vec<FtpFileEntry>, Box<dyn std::error::Error + Send + Sync>> {
    let password = general_purpose::STANDARD.decode(&connection.password_encrypted)?;
    let password = String::from_utf8(password)?;

    let mut ftp_stream =
        FtpStream::connect(format!("{}:{}", connection.host, connection.port))?;
    ftp_stream.login(&connection.username, &password)?;

    if connection.passive_mode {
        ftp_stream.set_mode(Mode::Passive);
    }

    let target_path = path.unwrap_or(&connection.remote_path);
    ftp_stream.cwd(target_path)?;

    let file_list = ftp_stream.nlst(None)?;
    let mut entries = Vec::new();

    for filename in file_list {
        // Try to get file size to determine if it's a file or directory
        let is_dir = ftp_stream.cwd(&filename).is_ok();
        if is_dir {
            ftp_stream.cwd("..")?;
        }

        let size = if !is_dir {
            ftp_stream.size(&filename).unwrap_or(0) as u64
        } else {
            0
        };

        entries.push(FtpFileEntry {
            name: filename,
            is_directory: is_dir,
            size,
            modified: None,
        });
    }

    let _ = ftp_stream.quit();
    Ok(entries)
}

/// Sync files with FTP server
pub async fn sync_ftp(
    pool: &SqlitePool,
    connection: &FtpConnection,
) -> Result<SyncResult, Box<dyn std::error::Error + Send + Sync>> {
    let start = std::time::Instant::now();
    let started_at = chrono::Utc::now().to_rfc3339();

    let password = general_purpose::STANDARD.decode(&connection.password_encrypted)?;
    let password = String::from_utf8(password)?;

    let mut ftp_stream =
        FtpStream::connect(format!("{}:{}", connection.host, connection.port))?;
    ftp_stream.login(&connection.username, &password)?;

    if connection.passive_mode {
        ftp_stream.set_mode(Mode::Passive);
    } else {
        ftp_stream.set_mode(Mode::Active);
    }

    ftp_stream.cwd(&connection.remote_path)?;

    let mut result = SyncResult {
        connection_id: connection.id.clone(),
        uploaded: 0,
        downloaded: 0,
        errors: 0,
        error_messages: Vec::new(),
        started_at,
        completed_at: String::new(),
        duration_ms: 0,
    };

    let local_path = PathBuf::from("./data").join(&connection.local_path);
    fs::create_dir_all(&local_path).await.ok();

    match connection.sync_direction.as_str() {
        "upload" => {
            sync_upload(&mut ftp_stream, &local_path, &mut result).await;
        }
        "download" => {
            sync_download(&mut ftp_stream, &local_path, &mut result).await;
        }
        "bidirectional" => {
            sync_upload(&mut ftp_stream, &local_path, &mut result).await;
            sync_download(&mut ftp_stream, &local_path, &mut result).await;
        }
        _ => {
            result.error_messages.push(format!(
                "Unknown sync direction: {}",
                connection.sync_direction
            ));
            result.errors += 1;
        }
    }

    let _ = ftp_stream.quit();

    result.completed_at = chrono::Utc::now().to_rfc3339();
    result.duration_ms = start.elapsed().as_millis() as u64;

    // Update last sync status
    let status = if result.errors == 0 { "success" } else { "partial" };
    let error = if result.error_messages.is_empty() {
        None
    } else {
        Some(result.error_messages.join("; "))
    };

    sqlx::query(
        "UPDATE ftp_connections SET last_sync_at = datetime('now'), last_sync_status = ?, last_sync_error = ? WHERE id = ?",
    )
    .bind(status)
    .bind(error)
    .bind(&connection.id)
    .execute(pool)
    .await?;

    Ok(result)
}

async fn sync_upload(ftp: &mut FtpStream, local_path: &PathBuf, result: &mut SyncResult) {
    if let Ok(entries) = std::fs::read_dir(local_path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    let filename = entry.file_name().to_string_lossy().to_string();
                    match std::fs::read(entry.path()) {
                        Ok(file_data) => {
                            let mut cursor = Cursor::new(file_data);
                            match ftp.put_file(&filename, &mut cursor) {
                                Ok(_) => result.uploaded += 1,
                                Err(e) => {
                                    result.errors += 1;
                                    result.error_messages.push(format!("Upload {}: {}", filename, e));
                                }
                            }
                        }
                        Err(e) => {
                            result.errors += 1;
                            result.error_messages.push(format!("Read {}: {}", filename, e));
                        }
                    }
                }
            }
        }
    }
}

async fn sync_download(ftp: &mut FtpStream, local_path: &PathBuf, result: &mut SyncResult) {
    match ftp.nlst(None) {
        Ok(files) => {
            for filename in files {
                match ftp.retr_as_buffer(&filename) {
                    Ok(data) => {
                        let file_path = local_path.join(&filename);
                        match std::fs::write(&file_path, &data.into_inner()) {
                            Ok(_) => result.downloaded += 1,
                            Err(e) => {
                                result.errors += 1;
                                result.error_messages.push(format!("Write {}: {}", filename, e));
                            }
                        }
                    }
                    Err(e) => {
                        result.errors += 1;
                        result.error_messages.push(format!("Download {}: {}", filename, e));
                    }
                }
            }
        }
        Err(e) => {
            result.errors += 1;
            result.error_messages.push(format!("List files: {}", e));
        }
    }
}
