/// File locking and real-time collaboration system
/// Supports pessimistic and optimistic locking with conflict resolution
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FileLock {
    pub id: String,
    pub file_id: String,
    pub user_id: String,
    pub lock_type: String, // "read" or "write"
    pub expires_at: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FilePresence {
    pub id: String,
    pub file_id: String,
    pub user_id: String,
    pub user_name: String,
    pub cursor_position: Option<i32>,
    pub last_seen: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConflictResolution {
    pub id: String,
    pub file_id: String,
    pub version_a_id: String,
    pub version_b_id: String,
    pub resolved_version_id: Option<String>,
    pub resolution_strategy: String, // "manual", "theirs", "ours", "merge"
    pub resolved_by: Option<String>,
    pub resolved_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AcquireLockRequest {
    pub file_id: String,
    pub lock_type: String, // "read" or "write"
    pub duration_seconds: Option<i64>, // Optional lock duration, default 300s
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePresenceRequest {
    pub file_id: String,
    pub cursor_position: Option<i32>,
}

/// Acquire a file lock (pessimistic locking)
pub async fn acquire_lock(
    pool: &SqlitePool,
    user_id: &str,
    user_name: &str,
    req: AcquireLockRequest,
) -> Result<FileLock, Box<dyn std::error::Error + Send + Sync>> {
    // Check for existing write locks
    let existing_write_locks: Vec<FileLock> = sqlx::query_as(
        "SELECT * FROM file_locks WHERE file_id = ? AND lock_type = 'write' AND expires_at > datetime('now')"
    )
    .bind(&req.file_id)
    .fetch_all(pool)
    .await?;
    
    if !existing_write_locks.is_empty() && req.lock_type == "write" {
        return Err("File is already locked for writing by another user".into());
    }
    
    // If requesting write lock, check for any existing read locks
    if req.lock_type == "write" {
        let existing_read_locks: Vec<FileLock> = sqlx::query_as(
            "SELECT * FROM file_locks WHERE file_id = ? AND lock_type = 'read' AND expires_at > datetime('now') AND user_id != ?"
        )
        .bind(&req.file_id)
        .bind(user_id)
        .fetch_all(pool)
        .await?;
        
        if !existing_read_locks.is_empty() {
            return Err("File is locked for reading by other users".into());
        }
    }
    
    let id = Uuid::new_v4().to_string();
    let duration = req.duration_seconds.unwrap_or(300); // Default 5 minutes
    let expires_at = Utc::now() + Duration::seconds(duration);
    
    sqlx::query(
        "INSERT INTO file_locks (id, file_id, user_id, lock_type, expires_at, created_at)
         VALUES (?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&id)
    .bind(&req.file_id)
    .bind(user_id)
    .bind(&req.lock_type)
    .bind(expires_at.to_rfc3339())
    .execute(pool)
    .await?;
    
    let lock = sqlx::query_as::<_, FileLock>(
        "SELECT * FROM file_locks WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(pool)
    .await?;
    
    Ok(lock)
}

/// Release a file lock
pub async fn release_lock(
    pool: &SqlitePool,
    lock_id: &str,
    user_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM file_locks WHERE id = ? AND user_id = ?"
    )
    .bind(lock_id)
    .bind(user_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Release all locks for a user
pub async fn release_all_user_locks(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM file_locks WHERE user_id = ?"
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Get active locks for a file
pub async fn get_file_locks(
    pool: &SqlitePool,
    file_id: &str,
) -> Result<Vec<FileLock>, sqlx::Error> {
    // Clean up expired locks first
    sqlx::query("DELETE FROM file_locks WHERE expires_at <= datetime('now')")
        .execute(pool)
        .await?;
    
    sqlx::query_as::<_, FileLock>(
        "SELECT * FROM file_locks WHERE file_id = ? AND expires_at > datetime('now') ORDER BY created_at DESC"
    )
    .bind(file_id)
    .fetch_all(pool)
    .await
}

/// Check if file is locked
pub async fn is_file_locked(
    pool: &SqlitePool,
    file_id: &str,
    user_id: &str,
    lock_type: &str,
) -> Result<bool, sqlx::Error> {
    let count: (i64,) = if lock_type == "write" {
        // For write lock, check any locks by others
        sqlx::query_as(
            "SELECT COUNT(*) FROM file_locks WHERE file_id = ? AND user_id != ? AND expires_at > datetime('now')"
        )
        .bind(file_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?
    } else {
        // For read lock, check write locks by others
        sqlx::query_as(
            "SELECT COUNT(*) FROM file_locks WHERE file_id = ? AND lock_type = 'write' AND user_id != ? AND expires_at > datetime('now')"
        )
        .bind(file_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?
    };
    
    Ok(count.0 > 0)
}

/// Update user presence on a file (for real-time collaboration)
pub async fn update_presence(
    pool: &SqlitePool,
    user_id: &str,
    user_name: &str,
    req: UpdatePresenceRequest,
) -> Result<FilePresence, sqlx::Error> {
    // Check if presence exists
    let existing: Option<FilePresence> = sqlx::query_as(
        "SELECT * FROM file_presence WHERE file_id = ? AND user_id = ?"
    )
    .bind(&req.file_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    
    if let Some(presence) = existing {
        // Update existing presence
        sqlx::query(
            "UPDATE file_presence SET cursor_position = ?, last_seen = datetime('now') WHERE id = ?"
        )
        .bind(req.cursor_position)
        .bind(&presence.id)
        .execute(pool)
        .await?;
        
        sqlx::query_as::<_, FilePresence>(
            "SELECT * FROM file_presence WHERE id = ?"
        )
        .bind(&presence.id)
        .fetch_one(pool)
        .await
    } else {
        // Create new presence
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO file_presence (id, file_id, user_id, user_name, cursor_position, last_seen)
             VALUES (?, ?, ?, ?, ?, datetime('now'))"
        )
        .bind(&id)
        .bind(&req.file_id)
        .bind(user_id)
        .bind(user_name)
        .bind(req.cursor_position)
        .execute(pool)
        .await?;
        
        sqlx::query_as::<_, FilePresence>(
            "SELECT * FROM file_presence WHERE id = ?"
        )
        .bind(&id)
        .fetch_one(pool)
        .await
    }
}

/// Get active users on a file
pub async fn get_file_presence(
    pool: &SqlitePool,
    file_id: &str,
) -> Result<Vec<FilePresence>, sqlx::Error> {
    // Clean up stale presence (older than 5 minutes)
    sqlx::query(
        "DELETE FROM file_presence WHERE last_seen < datetime('now', '-5 minutes')"
    )
    .execute(pool)
    .await?;
    
    sqlx::query_as::<_, FilePresence>(
        "SELECT * FROM file_presence WHERE file_id = ? ORDER BY last_seen DESC"
    )
    .bind(file_id)
    .fetch_all(pool)
    .await
}

/// Remove user presence
pub async fn remove_presence(
    pool: &SqlitePool,
    user_id: &str,
    file_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM file_presence WHERE user_id = ? AND file_id = ?"
    )
    .bind(user_id)
    .bind(file_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Create conflict resolution record
pub async fn create_conflict(
    pool: &SqlitePool,
    file_id: &str,
    version_a_id: &str,
    version_b_id: &str,
) -> Result<ConflictResolution, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO conflict_resolutions (id, file_id, version_a_id, version_b_id, resolution_strategy, created_at)
         VALUES (?, ?, ?, ?, 'manual', datetime('now'))"
    )
    .bind(&id)
    .bind(file_id)
    .bind(version_a_id)
    .bind(version_b_id)
    .execute(pool)
    .await?;
    
    sqlx::query_as::<_, ConflictResolution>(
        "SELECT * FROM conflict_resolutions WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(pool)
    .await
}

/// Resolve a conflict
pub async fn resolve_conflict(
    pool: &SqlitePool,
    conflict_id: &str,
    resolved_version_id: &str,
    resolution_strategy: &str,
    resolved_by: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE conflict_resolutions 
         SET resolved_version_id = ?, resolution_strategy = ?, resolved_by = ?, resolved_at = datetime('now')
         WHERE id = ?"
    )
    .bind(resolved_version_id)
    .bind(resolution_strategy)
    .bind(resolved_by)
    .bind(conflict_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Get unresolved conflicts for a file
pub async fn get_file_conflicts(
    pool: &SqlitePool,
    file_id: &str,
) -> Result<Vec<ConflictResolution>, sqlx::Error> {
    sqlx::query_as::<_, ConflictResolution>(
        "SELECT * FROM conflict_resolutions WHERE file_id = ? AND resolved_at IS NULL ORDER BY created_at DESC"
    )
    .bind(file_id)
    .fetch_all(pool)
    .await
}

/// Renew a lock (extend expiration)
pub async fn renew_lock(
    pool: &SqlitePool,
    lock_id: &str,
    user_id: &str,
    extension_seconds: i64,
) -> Result<FileLock, sqlx::Error> {
    let new_expires = Utc::now() + Duration::seconds(extension_seconds);
    
    sqlx::query(
        "UPDATE file_locks SET expires_at = ? WHERE id = ? AND user_id = ?"
    )
    .bind(new_expires.to_rfc3339())
    .bind(lock_id)
    .bind(user_id)
    .execute(pool)
    .await?;
    
    sqlx::query_as::<_, FileLock>(
        "SELECT * FROM file_locks WHERE id = ?"
    )
    .bind(lock_id)
    .fetch_one(pool)
    .await
}

/// Clean up expired locks (maintenance task)
pub async fn cleanup_expired_locks(pool: &SqlitePool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM file_locks WHERE expires_at <= datetime('now')"
    )
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected())
}

/// Clean up stale presence (maintenance task)
pub async fn cleanup_stale_presence(pool: &SqlitePool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM file_presence WHERE last_seen < datetime('now', '-10 minutes')"
    )
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected())
}
