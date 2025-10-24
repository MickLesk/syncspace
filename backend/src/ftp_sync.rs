/// FTP/FTPS synchronization
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

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
    let password_encrypted = base64::encode(&req.password);
    
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

/// Sync with FTP server (placeholder - requires ftp crate)
pub async fn sync_ftp(
    _connection: &FtpConnection,
) -> Result<SyncResult, Box<dyn std::error::Error + Send + Sync>> {
    // Placeholder implementation
    // In production: use `suppaftp` or `async-ftp` crate
    /*
    Example with suppaftp:
    
    let mut ftp_stream = if connection.use_ftps {
        FtpStream::connect_secure((connection.host.as_str(), connection.port as u16))?
    } else {
        FtpStream::connect((connection.host.as_str(), connection.port as u16))?
    };
    
    ftp_stream.login(&connection.username, &password)?;
    
    if connection.passive_mode {
        ftp_stream.pasv()?;
    }
    
    ftp_stream.cwd(&connection.remote_path)?;
    
    match connection.sync_direction.as_str() {
        "upload" => {
            // Upload files from local to remote
            let files = std::fs::read_dir(&connection.local_path)?;
            for file in files {
                let file = file?;
                let filename = file.file_name().to_string_lossy().to_string();
                let mut file_data = std::fs::read(file.path())?;
                ftp_stream.put(&filename, &mut file_data.as_slice())?;
            }
        },
        "download" => {
            // Download files from remote to local
            let files = ftp_stream.nlst(None)?;
            for filename in files {
                let data = ftp_stream.retr_as_buffer(&filename)?;
                std::fs::write(format!("{}/{}", connection.local_path, filename), data)?;
            }
        },
        "bidirectional" => {
            // Two-way sync with conflict resolution
        },
        _ => {}
    }
    
    ftp_stream.quit()?;
    */
    
    Ok(SyncResult {
        uploaded: 0,
        downloaded: 0,
        errors: 0,
    })
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
