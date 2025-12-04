//! Auto-Cleanup Service for deleted files
//!
//! Automatically permanently deletes soft-deleted files after a retention period (default: 30 days)
//! This service prevents deleted files from consuming storage indefinitely.

use chrono::{Duration, Utc};
use sqlx::SqlitePool;
use std::path::Path;
use tracing::{error, info, warn};

const RETENTION_DAYS: i64 = 30;
const DATA_DIR: &str = "./data";

/// Configuration for cleanup operations
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CleanupConfig {
    /// Number of days to retain deleted files before permanent deletion
    pub retention_days: i64,
    /// Enable automatic cleanup on startup
    pub auto_cleanup_on_startup: bool,
    /// Enable periodic cleanup (every N hours)
    pub enable_periodic_cleanup: bool,
    /// Interval in hours for periodic cleanup
    pub cleanup_interval_hours: u64,
}

impl Default for CleanupConfig {
    fn default() -> Self {
        Self {
            retention_days: RETENTION_DAYS,
            auto_cleanup_on_startup: false,
            enable_periodic_cleanup: true,
            cleanup_interval_hours: 24,
        }
    }
}

/// Cleanup statistics
#[derive(Debug, Clone, serde::Serialize)]
pub struct CleanupStats {
    /// Number of files permanently deleted
    pub files_deleted: i32,
    /// Total storage freed in bytes
    pub storage_freed_bytes: i64,
    /// Number of failed deletions
    pub failed_deletions: i32,
    /// Duration of cleanup in milliseconds
    pub duration_ms: u128,
    /// Timestamp of last cleanup
    pub last_cleanup_at: String,
}

/// Perform automatic cleanup of expired deleted files
///
/// Finds all files with:
/// - `is_deleted = 1` (soft-deleted)
/// - `deleted_at <= now - retention_days`
///
/// And permanently deletes them from both:
/// - Physical storage
/// - Database records
pub async fn cleanup_expired_deleted_files(
    pool: &SqlitePool,
    config: &CleanupConfig,
) -> Result<CleanupStats, Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();

    // Calculate cutoff date
    let cutoff_date = Utc::now() - Duration::days(config.retention_days);
    let cutoff_iso = cutoff_date.to_rfc3339();

    info!(
        "🗑️ Starting auto-cleanup of deleted files older than {} days (before {})",
        config.retention_days, cutoff_iso
    );

    // 1. Find all soft-deleted files that have expired
    let expired_files: Vec<(String, String)> = sqlx::query_as(
        r#"
        SELECT id, path FROM files
        WHERE is_deleted = 1
        AND deleted_at IS NOT NULL
        AND deleted_at <= ?1
        ORDER BY deleted_at ASC
        LIMIT 10000
        "#,
    )
    .bind(&cutoff_iso)
    .fetch_all(pool)
    .await?;

    let total_files = expired_files.len();
    info!("📋 Found {} expired deleted files to cleanup", total_files);

    let mut stats = CleanupStats {
        files_deleted: 0,
        storage_freed_bytes: 0,
        failed_deletions: 0,
        duration_ms: 0,
        last_cleanup_at: Utc::now().to_rfc3339(),
    };

    // 2. Get total storage before deletion
    let _storage_before: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(size_bytes), 0) FROM files WHERE is_deleted = 1 AND deleted_at <= ?1",
    )
    .bind(&cutoff_iso)
    .fetch_one(pool)
    .await?;

    // 3. Delete physical files and database records
    for (file_id, file_path) in expired_files.iter() {
        match delete_file_permanently(pool, file_id, file_path).await {
            Ok(size_freed) => {
                stats.files_deleted += 1;
                stats.storage_freed_bytes += size_freed;
            }
            Err(e) => {
                stats.failed_deletions += 1;
                error!("❌ Failed to delete file {}: {}", file_path, e);
            }
        }
    }

    stats.duration_ms = start_time.elapsed().as_millis();

    info!(
        "✅ Cleanup completed: {} files deleted, {} bytes freed, {} failed ({}ms)",
        stats.files_deleted, stats.storage_freed_bytes, stats.failed_deletions, stats.duration_ms
    );

    // Log cleanup event
    let _log_result = log_cleanup_event(pool, &stats).await;

    Ok(stats)
}

/// Permanently delete a single file
async fn delete_file_permanently(
    pool: &SqlitePool,
    file_id: &str,
    file_path: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    // Get file size for stats
    let size: (i64,) = sqlx::query_as("SELECT size_bytes FROM files WHERE id = ?1")
        .bind(file_id)
        .fetch_optional(pool)
        .await?
        .unwrap_or((0i64,));

    let size_bytes = size.0;

    // Delete physical file
    let full_path = format!("{}/{}", DATA_DIR, file_path);
    if Path::new(&full_path).exists() {
        match std::fs::remove_file(&full_path) {
            Ok(_) => {
                // Physical file deleted successfully
            }
            Err(e) => {
                warn!("⚠️ Failed to delete physical file {}: {}", full_path, e);
                // Continue with database deletion even if physical file is already gone
            }
        }
    }

    // Delete from database (cascade deletes related records)
    sqlx::query("DELETE FROM files WHERE id = ?1")
        .bind(file_id)
        .execute(pool)
        .await?;

    // Delete file versions
    sqlx::query("DELETE FROM file_versions WHERE file_id = ?1")
        .bind(file_id)
        .execute(pool)
        .await?;

    // Delete file tags
    sqlx::query("DELETE FROM file_tags WHERE file_id = ?1")
        .bind(file_id)
        .execute(pool)
        .await?;

    Ok(size_bytes)
}

/// Log cleanup event for audit purposes
async fn log_cleanup_event(
    pool: &SqlitePool,
    stats: &CleanupStats,
) -> Result<(), Box<dyn std::error::Error>> {
    let event_id = uuid::Uuid::new_v4().to_string();

    // Insert into activity log or dedicated cleanup log
    sqlx::query(
        r#"
        INSERT INTO activity (id, user_id, action, file_path, metadata, created_at)
        VALUES (?1, NULL, ?2, ?3, ?4, ?5)
        "#,
    )
    .bind(&event_id)
    .bind("auto_cleanup")
    .bind("system")
    .bind(
        serde_json::json!({
            "files_deleted": stats.files_deleted,
            "storage_freed_bytes": stats.storage_freed_bytes,
            "failed_deletions": stats.failed_deletions,
            "duration_ms": stats.duration_ms,
        })
        .to_string(),
    )
    .bind(Utc::now().to_rfc3339())
    .execute(pool)
    .await?;

    Ok(())
}

/// Get cleanup statistics (last cleanup info)
pub async fn get_cleanup_stats(pool: &SqlitePool) -> Result<Option<CleanupStats>, sqlx::Error> {
    let result: Option<(String,)> = sqlx::query_as(
        r#"
        SELECT metadata FROM activity
        WHERE action = 'auto_cleanup'
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await?;

    if let Some((metadata,)) = result {
        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&metadata) {
            let stats = CleanupStats {
                files_deleted: data["files_deleted"].as_i64().unwrap_or(0) as i32,
                storage_freed_bytes: data["storage_freed_bytes"].as_i64().unwrap_or(0),
                failed_deletions: data["failed_deletions"].as_i64().unwrap_or(0) as i32,
                duration_ms: data["duration_ms"].as_u64().unwrap_or(0) as u128,
                last_cleanup_at: Utc::now().to_rfc3339(),
            };
            return Ok(Some(stats));
        }
    }

    Ok(None)
}

/// Get count of files eligible for cleanup
pub async fn get_eligible_for_cleanup_count(
    pool: &SqlitePool,
    retention_days: i64,
) -> Result<i32, sqlx::Error> {
    let cutoff_date = Utc::now() - Duration::days(retention_days);
    let cutoff_iso = cutoff_date.to_rfc3339();

    let count: (i32,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM files
        WHERE is_deleted = 1
        AND deleted_at IS NOT NULL
        AND deleted_at <= ?1
        "#,
    )
    .bind(&cutoff_iso)
    .fetch_one(pool)
    .await?;

    Ok(count.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanup_config_default() {
        let config = CleanupConfig::default();
        assert_eq!(config.retention_days, RETENTION_DAYS);
        assert!(!config.auto_cleanup_on_startup);
        assert!(config.enable_periodic_cleanup);
    }
}
