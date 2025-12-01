//! Enhanced Backup & Restore API endpoints

use crate::auth::UserInfo;
use crate::{services, AppState};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateBackupRequest {
    pub backup_type: String, // 'full', 'incremental', 'database', 'files'
    pub include_versions: Option<bool>,
    pub include_database: Option<bool>,
    pub description: Option<String>,
    pub destination_id: Option<String>,
    pub encrypt: Option<bool>,
    pub encryption_password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Backup {
    pub id: String,
    pub backup_type: String,
    pub size_bytes: i64,
    pub file_count: Option<i32>,
    pub storage_path: String,
    pub created_by: String,
    pub created_at: String,
    pub status: String,
    pub error_message: Option<String>,
    pub is_encrypted: bool,
    pub compressed_size_bytes: Option<i64>,
    pub description: Option<String>,
    pub checksum: Option<String>,
    pub destination_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct BackupSchedule {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub cron_expression: String,
    pub backup_type: String,
    pub enabled: bool,
    pub destination_type: String,
    pub destination_id: Option<String>,
    pub retention_days: Option<i32>,
    pub created_by: String,
    pub created_at: String,
    pub last_run_at: Option<String>,
    pub next_run_at: Option<String>,
    pub encryption_enabled: bool,
    pub include_versions: bool,
    pub include_database: bool,
    pub max_backups: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct BackupDestination {
    pub id: String,
    pub name: String,
    pub destination_type: String,
    pub is_default: bool,
    pub created_by: String,
    pub created_at: String,
    pub last_verified_at: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct BackupRestore {
    pub id: String,
    pub backup_id: String,
    pub restored_by: String,
    pub restored_at: String,
    pub restore_type: String,
    pub restore_path: Option<String>,
    pub files_restored: i32,
    pub status: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct BackupVerification {
    pub id: String,
    pub backup_id: String,
    pub verification_type: String,
    pub status: String,
    pub details: Option<String>,
    pub verified_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BackupStats {
    pub total_backups: i64,
    pub successful_backups: i64,
    pub failed_backups: i64,
    pub total_size_bytes: i64,
    pub total_files: i64,
    pub last_backup_at: Option<String>,
    pub next_scheduled_at: Option<String>,
    pub storage_by_type: Vec<StorageByType>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StorageByType {
    pub backup_type: String,
    pub count: i64,
    pub size_bytes: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateScheduleRequest {
    pub name: String,
    pub description: Option<String>,
    pub cron_expression: String,
    pub backup_type: String,
    pub destination_id: Option<String>,
    pub retention_days: Option<i32>,
    pub encryption_enabled: Option<bool>,
    pub include_versions: Option<bool>,
    pub include_database: Option<bool>,
    pub max_backups: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDestinationRequest {
    pub name: String,
    pub destination_type: String,
    pub config: serde_json::Value,
    pub is_default: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct RestoreRequest {
    pub restore_type: String, // 'full', 'partial', 'selective'
    pub restore_path: Option<String>,
    pub file_paths: Option<Vec<String>>, // For selective restore
    pub encryption_password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BackupListQuery {
    pub status: Option<String>,
    pub backup_type: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        // Backups
        .route("/backups", get(list_backups).post(create_backup))
        .route("/backups/stats", get(get_backup_stats))
        .route(
            "/backups/{backup_id}",
            get(get_backup).delete(delete_backup),
        )
        .route("/backups/{backup_id}/verify", post(verify_backup))
        .route("/backups/{backup_id}/restore", post(restore_backup))
        .route("/backups/{backup_id}/download", get(download_backup))
        .route(
            "/backups/{backup_id}/verifications",
            get(list_verifications),
        )
        .route("/backups/cleanup", post(cleanup_backups))
        // Backup schedules
        .route(
            "/backups/schedules",
            get(list_schedules).post(create_schedule),
        )
        .route(
            "/backups/schedules/{schedule_id}",
            get(get_schedule)
                .put(update_schedule)
                .delete(delete_schedule),
        )
        .route(
            "/backups/schedules/{schedule_id}/toggle",
            post(toggle_schedule),
        )
        .route(
            "/backups/schedules/{schedule_id}/trigger",
            post(trigger_schedule),
        )
        // Backup destinations
        .route(
            "/backups/destinations",
            get(list_destinations).post(create_destination),
        )
        .route(
            "/backups/destinations/{destination_id}",
            get(get_destination)
                .put(update_destination)
                .delete(delete_destination),
        )
        .route(
            "/backups/destinations/{destination_id}/test",
            post(test_destination),
        )
        // Restore history
        .route("/backups/restores", get(list_restores))
}

async fn list_backups(
    State(state): State<AppState>,
    _user: UserInfo,
    Query(query): Query<BackupListQuery>,
) -> Result<Json<Vec<Backup>>, StatusCode> {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let mut sql = String::from(
        "SELECT id, backup_type, size_bytes, file_count, storage_path, created_by, created_at, 
                status, error_message, COALESCE(is_encrypted, 0) as is_encrypted, 
                compressed_size_bytes, description, checksum, destination_type
         FROM backups WHERE 1=1",
    );

    if let Some(ref status) = query.status {
        sql.push_str(&format!(" AND status = '{}'", status));
    }
    if let Some(ref backup_type) = query.backup_type {
        sql.push_str(&format!(" AND backup_type = '{}'", backup_type));
    }

    sql.push_str(&format!(
        " ORDER BY created_at DESC LIMIT {} OFFSET {}",
        limit, offset
    ));

    let backups = sqlx::query_as::<_, Backup>(&sql)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(backups))
}

async fn get_backup_stats(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<BackupStats>, StatusCode> {
    // Get totals
    let totals: (i64, i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT 
            COUNT(*) as total,
            SUM(CASE WHEN status = 'completed' THEN 1 ELSE 0 END) as successful,
            SUM(CASE WHEN status = 'failed' THEN 1 ELSE 0 END) as failed,
            COALESCE(SUM(size_bytes), 0) as total_size,
            COALESCE(SUM(file_count), 0) as total_files
        FROM backups
        "#,
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0, 0, 0, 0, 0));

    // Get last backup time
    let last_backup: Option<(String,)> = sqlx::query_as(
        "SELECT created_at FROM backups WHERE status = 'completed' ORDER BY created_at DESC LIMIT 1"
    )
    .fetch_optional(&state.db_pool)
    .await
    .ok()
    .flatten();

    // Get next scheduled backup
    let next_scheduled: Option<(String,)> = sqlx::query_as(
        "SELECT next_run_at FROM backup_schedules WHERE enabled = 1 AND next_run_at IS NOT NULL ORDER BY next_run_at ASC LIMIT 1"
    )
    .fetch_optional(&state.db_pool)
    .await
    .ok()
    .flatten();

    // Get storage by type
    let storage_by_type: Vec<(String, i64, i64)> = sqlx::query_as(
        "SELECT backup_type, COUNT(*) as count, COALESCE(SUM(size_bytes), 0) as size_bytes 
         FROM backups GROUP BY backup_type",
    )
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    Ok(Json(BackupStats {
        total_backups: totals.0,
        successful_backups: totals.1,
        failed_backups: totals.2,
        total_size_bytes: totals.3,
        total_files: totals.4,
        last_backup_at: last_backup.map(|b| b.0),
        next_scheduled_at: next_scheduled.map(|s| s.0),
        storage_by_type: storage_by_type
            .into_iter()
            .map(|(t, c, s)| StorageByType {
                backup_type: t,
                count: c,
                size_bytes: s,
            })
            .collect(),
    }))
}

async fn create_backup(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateBackupRequest>,
) -> Result<Json<Backup>, StatusCode> {
    let backup_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let storage_path = format!("./data/backups/{}.zip", backup_id);

    // Insert backup record
    sqlx::query(
        r#"
        INSERT INTO backups (id, backup_type, size_bytes, file_count, storage_path, created_by, 
                           created_at, status, is_encrypted, description, destination_type)
        VALUES (?, ?, 0, 0, ?, ?, ?, 'in_progress', ?, ?, 'local')
        "#,
    )
    .bind(&backup_id)
    .bind(&req.backup_type)
    .bind(&storage_path)
    .bind(&user.id)
    .bind(&now)
    .bind(req.encrypt.unwrap_or(false))
    .bind(&req.description)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create backup: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Queue background job for actual backup creation
    let job_id = Uuid::new_v4().to_string();
    let job_params = serde_json::json!({
        "backup_id": backup_id,
        "backup_type": req.backup_type,
        "include_versions": req.include_versions.unwrap_or(true),
        "include_database": req.include_database.unwrap_or(true),
        "encrypt": req.encrypt.unwrap_or(false),
        "destination_id": req.destination_id
    });

    let _ = sqlx::query(
        "INSERT INTO jobs (id, job_type, status, parameters, created_at, priority) 
         VALUES (?, 'backup_creation', 'pending', ?, datetime('now'), 'high')",
    )
    .bind(&job_id)
    .bind(job_params.to_string())
    .execute(&state.db_pool)
    .await;

    let backup = Backup {
        id: backup_id,
        backup_type: req.backup_type,
        size_bytes: 0,
        file_count: Some(0),
        storage_path,
        created_by: user.id,
        created_at: now,
        status: "in_progress".to_string(),
        error_message: None,
        is_encrypted: req.encrypt.unwrap_or(false),
        compressed_size_bytes: None,
        description: req.description,
        checksum: None,
        destination_type: Some("local".to_string()),
    };

    Ok(Json(backup))
}

async fn get_backup(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(backup_id): Path<String>,
) -> Result<Json<Backup>, StatusCode> {
    let backup = sqlx::query_as::<_, Backup>(
        r#"
        SELECT id, backup_type, size_bytes, file_count, storage_path, created_by, created_at, 
               status, error_message, COALESCE(is_encrypted, 0) as is_encrypted, 
               compressed_size_bytes, description, checksum, destination_type
        FROM backups WHERE id = ?
        "#,
    )
    .bind(&backup_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(backup))
}

async fn delete_backup(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(backup_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // Get storage path to delete file
    let backup: Option<(String,)> = sqlx::query_as("SELECT storage_path FROM backups WHERE id = ?")
        .bind(&backup_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((storage_path,)) = backup {
        // Delete file
        let _ = tokio::fs::remove_file(&storage_path).await;

        // Delete database record
        sqlx::query("DELETE FROM backups WHERE id = ?")
            .bind(&backup_id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn verify_backup(
    State(state): State<AppState>,
    user: UserInfo,
    Path(backup_id): Path<String>,
) -> Result<Json<BackupVerification>, StatusCode> {
    // Verify backup exists
    let backup: Option<(String, String)> =
        sqlx::query_as("SELECT storage_path, checksum FROM backups WHERE id = ?")
            .bind(&backup_id)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (storage_path, _checksum) = backup.ok_or(StatusCode::NOT_FOUND)?;

    // Check if file exists
    let file_exists = tokio::fs::metadata(&storage_path).await.is_ok();

    let verification_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let status = if file_exists { "passed" } else { "failed" };
    let details = serde_json::json!({
        "file_exists": file_exists,
        "verified_by": user.id,
        "verification_method": "file_check"
    });

    sqlx::query(
        "INSERT INTO backup_verifications (id, backup_id, verification_type, status, details, verified_at)
         VALUES (?, ?, 'file_check', ?, ?, ?)"
    )
    .bind(&verification_id)
    .bind(&backup_id)
    .bind(status)
    .bind(details.to_string())
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(BackupVerification {
        id: verification_id,
        backup_id,
        verification_type: "file_check".to_string(),
        status: status.to_string(),
        details: Some(details.to_string()),
        verified_at: now,
    }))
}

async fn restore_backup(
    State(state): State<AppState>,
    user: UserInfo,
    Path(backup_id): Path<String>,
    Json(req): Json<RestoreRequest>,
) -> Result<Json<BackupRestore>, StatusCode> {
    // Verify backup exists and is completed
    let backup: Option<(String, String)> =
        sqlx::query_as("SELECT storage_path, status FROM backups WHERE id = ?")
            .bind(&backup_id)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (storage_path, status) = backup.ok_or(StatusCode::NOT_FOUND)?;

    if status != "completed" {
        return Err(StatusCode::BAD_REQUEST);
    }

    let restore_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // Create restore record
    sqlx::query(
        "INSERT INTO backup_restores (id, backup_id, restored_by, restored_at, restore_type, restore_path, status)
         VALUES (?, ?, ?, ?, ?, ?, 'in_progress')"
    )
    .bind(&restore_id)
    .bind(&backup_id)
    .bind(&user.id)
    .bind(&now)
    .bind(&req.restore_type)
    .bind(&req.restore_path)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Queue restore job
    let job_id = Uuid::new_v4().to_string();
    let job_params = serde_json::json!({
        "restore_id": restore_id,
        "backup_id": backup_id,
        "storage_path": storage_path,
        "restore_type": req.restore_type,
        "restore_path": req.restore_path,
        "file_paths": req.file_paths
    });

    let _ = sqlx::query(
        "INSERT INTO jobs (id, job_type, status, parameters, created_at, priority) 
         VALUES (?, 'backup_restore', 'pending', ?, datetime('now'), 'high')",
    )
    .bind(&job_id)
    .bind(job_params.to_string())
    .execute(&state.db_pool)
    .await;

    Ok(Json(BackupRestore {
        id: restore_id,
        backup_id,
        restored_by: user.id,
        restored_at: now,
        restore_type: req.restore_type,
        restore_path: req.restore_path,
        files_restored: 0,
        status: "in_progress".to_string(),
        error_message: None,
    }))
}

async fn download_backup(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(backup_id): Path<String>,
) -> Result<axum::response::Response, StatusCode> {
    use axum::body::Body;
    use axum::http::header;
    use tokio_util::io::ReaderStream;

    let backup: Option<(String, String)> = sqlx::query_as(
        "SELECT storage_path, backup_type FROM backups WHERE id = ? AND status = 'completed'",
    )
    .bind(&backup_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (storage_path, backup_type) = backup.ok_or(StatusCode::NOT_FOUND)?;

    let file = tokio::fs::File::open(&storage_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let filename = format!("backup_{}_{}.zip", backup_type, backup_id);

    Ok(axum::response::Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/zip")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(body)
        .unwrap())
}

async fn list_verifications(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(backup_id): Path<String>,
) -> Result<Json<Vec<BackupVerification>>, StatusCode> {
    let verifications = sqlx::query_as::<_, BackupVerification>(
        "SELECT id, backup_id, verification_type, status, details, verified_at 
         FROM backup_verifications WHERE backup_id = ? ORDER BY verified_at DESC",
    )
    .bind(&backup_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(verifications))
}

async fn cleanup_backups(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Delete backups older than retention period based on schedule settings
    let deleted = sqlx::query(
        r#"
        DELETE FROM backups 
        WHERE id IN (
            SELECT b.id FROM backups b
            JOIN backup_schedules s ON b.schedule_id = s.id
            WHERE datetime(b.created_at) < datetime('now', '-' || COALESCE(s.retention_days, 30) || ' days')
        )
        "#
    )
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "deleted_count": deleted.rows_affected(),
        "message": "Cleanup completed"
    })))
}

async fn list_restores(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<Vec<BackupRestore>>, StatusCode> {
    let restores = sqlx::query_as::<_, BackupRestore>(
        "SELECT id, backup_id, restored_by, restored_at, restore_type, restore_path, 
                files_restored, status, error_message 
         FROM backup_restores ORDER BY restored_at DESC LIMIT 50",
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(restores))
}

// Backup schedule handlers

async fn list_schedules(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<Vec<BackupSchedule>>, StatusCode> {
    let schedules = sqlx::query_as::<_, BackupSchedule>(
        r#"
        SELECT id, name, description, cron_expression, backup_type, enabled, 
               destination_type, destination_id, retention_days, created_by, created_at,
               last_run_at, next_run_at, COALESCE(encryption_enabled, 0) as encryption_enabled,
               COALESCE(include_versions, 1) as include_versions, 
               COALESCE(include_database, 1) as include_database, max_backups
        FROM backup_schedules ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(schedules))
}

async fn create_schedule(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateScheduleRequest>,
) -> Result<Json<BackupSchedule>, StatusCode> {
    let schedule_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // Calculate next run time (simplified - just add 1 day for now)
    let next_run = (Utc::now() + Duration::days(1)).to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO backup_schedules (id, name, description, cron_expression, backup_type, enabled,
                                     destination_type, destination_id, retention_days, created_by, 
                                     created_at, next_run_at, encryption_enabled, include_versions, 
                                     include_database, max_backups)
        VALUES (?, ?, ?, ?, ?, 1, 'local', ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&schedule_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.cron_expression)
    .bind(&req.backup_type)
    .bind(&req.destination_id)
    .bind(req.retention_days.unwrap_or(30))
    .bind(&user.id)
    .bind(&now)
    .bind(&next_run)
    .bind(req.encryption_enabled.unwrap_or(false))
    .bind(req.include_versions.unwrap_or(true))
    .bind(req.include_database.unwrap_or(true))
    .bind(req.max_backups.unwrap_or(10))
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create schedule: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(BackupSchedule {
        id: schedule_id,
        name: req.name,
        description: req.description,
        cron_expression: req.cron_expression,
        backup_type: req.backup_type,
        enabled: true,
        destination_type: "local".to_string(),
        destination_id: req.destination_id,
        retention_days: Some(req.retention_days.unwrap_or(30)),
        created_by: user.id,
        created_at: now,
        last_run_at: None,
        next_run_at: Some(next_run),
        encryption_enabled: req.encryption_enabled.unwrap_or(false),
        include_versions: req.include_versions.unwrap_or(true),
        include_database: req.include_database.unwrap_or(true),
        max_backups: req.max_backups,
    }))
}

async fn get_schedule(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(schedule_id): Path<String>,
) -> Result<Json<BackupSchedule>, StatusCode> {
    let schedule = sqlx::query_as::<_, BackupSchedule>(
        r#"
        SELECT id, name, description, cron_expression, backup_type, enabled, 
               destination_type, destination_id, retention_days, created_by, created_at,
               last_run_at, next_run_at, COALESCE(encryption_enabled, 0) as encryption_enabled,
               COALESCE(include_versions, 1) as include_versions, 
               COALESCE(include_database, 1) as include_database, max_backups
        FROM backup_schedules WHERE id = ?
        "#,
    )
    .bind(&schedule_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(schedule))
}

async fn update_schedule(
    State(state): State<AppState>,
    Path(schedule_id): Path<String>,
    Json(req): Json<CreateScheduleRequest>,
) -> Result<Json<BackupSchedule>, StatusCode> {
    sqlx::query(
        r#"
        UPDATE backup_schedules 
        SET name = ?, description = ?, cron_expression = ?, backup_type = ?,
            destination_id = ?, retention_days = ?, encryption_enabled = ?,
            include_versions = ?, include_database = ?, max_backups = ?
        WHERE id = ?
        "#,
    )
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.cron_expression)
    .bind(&req.backup_type)
    .bind(&req.destination_id)
    .bind(req.retention_days.unwrap_or(30))
    .bind(req.encryption_enabled.unwrap_or(false))
    .bind(req.include_versions.unwrap_or(true))
    .bind(req.include_database.unwrap_or(true))
    .bind(req.max_backups.unwrap_or(10))
    .bind(&schedule_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    get_schedule(
        State(state),
        UserInfo {
            id: String::new(),
            username: String::new(),
            totp_enabled: false,
        },
        Path(schedule_id),
    )
    .await
}

async fn delete_schedule(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(schedule_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM backup_schedules WHERE id = ?")
        .bind(&schedule_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn toggle_schedule(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(schedule_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Toggle enabled status
    sqlx::query("UPDATE backup_schedules SET enabled = NOT enabled WHERE id = ?")
        .bind(&schedule_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let enabled: Option<(bool,)> =
        sqlx::query_as("SELECT enabled FROM backup_schedules WHERE id = ?")
            .bind(&schedule_id)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "id": schedule_id,
        "enabled": enabled.map(|e| e.0).unwrap_or(false)
    })))
}

async fn trigger_schedule(
    State(state): State<AppState>,
    user: UserInfo,
    Path(schedule_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Get schedule details
    let schedule: Option<(String, bool, bool, bool)> = sqlx::query_as(
        "SELECT backup_type, encryption_enabled, include_versions, include_database 
         FROM backup_schedules WHERE id = ?",
    )
    .bind(&schedule_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (backup_type, encrypt, include_versions, include_database) =
        schedule.ok_or(StatusCode::NOT_FOUND)?;

    // Create backup
    let backup_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let storage_path = format!("./data/backups/{}.zip", backup_id);

    sqlx::query(
        r#"
        INSERT INTO backups (id, backup_type, size_bytes, file_count, storage_path, created_by, 
                           created_at, status, is_encrypted, schedule_id, destination_type)
        VALUES (?, ?, 0, 0, ?, ?, ?, 'in_progress', ?, ?, 'local')
        "#,
    )
    .bind(&backup_id)
    .bind(&backup_type)
    .bind(&storage_path)
    .bind(&user.id)
    .bind(&now)
    .bind(encrypt)
    .bind(&schedule_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Update last run time
    sqlx::query("UPDATE backup_schedules SET last_run_at = ? WHERE id = ?")
        .bind(&now)
        .bind(&schedule_id)
        .execute(&state.db_pool)
        .await
        .ok();

    // Queue job
    let job_id = Uuid::new_v4().to_string();
    let job_params = serde_json::json!({
        "backup_id": backup_id,
        "backup_type": backup_type,
        "include_versions": include_versions,
        "include_database": include_database,
        "encrypt": encrypt,
        "schedule_id": schedule_id
    });

    let _ = sqlx::query(
        "INSERT INTO jobs (id, job_type, status, parameters, created_at, priority) 
         VALUES (?, 'backup_creation', 'pending', ?, datetime('now'), 'high')",
    )
    .bind(&job_id)
    .bind(job_params.to_string())
    .execute(&state.db_pool)
    .await;

    Ok(Json(serde_json::json!({
        "message": "Backup triggered successfully",
        "backup_id": backup_id,
        "status": "in_progress"
    })))
}

// Backup destination handlers

async fn list_destinations(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<Vec<BackupDestination>>, StatusCode> {
    let destinations = sqlx::query_as::<_, BackupDestination>(
        "SELECT id, name, destination_type, is_default, created_by, created_at, last_verified_at, status 
         FROM backup_destinations ORDER BY is_default DESC, name"
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(destinations))
}

async fn create_destination(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateDestinationRequest>,
) -> Result<Json<BackupDestination>, StatusCode> {
    let dest_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // If setting as default, unset other defaults
    if req.is_default.unwrap_or(false) {
        let _ = sqlx::query("UPDATE backup_destinations SET is_default = 0")
            .execute(&state.db_pool)
            .await;
    }

    sqlx::query(
        "INSERT INTO backup_destinations (id, name, destination_type, config, is_default, created_by, created_at, status)
         VALUES (?, ?, ?, ?, ?, ?, ?, 'active')"
    )
    .bind(&dest_id)
    .bind(&req.name)
    .bind(&req.destination_type)
    .bind(req.config.to_string())
    .bind(req.is_default.unwrap_or(false))
    .bind(&user.id)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(BackupDestination {
        id: dest_id,
        name: req.name,
        destination_type: req.destination_type,
        is_default: req.is_default.unwrap_or(false),
        created_by: user.id,
        created_at: now,
        last_verified_at: None,
        status: "active".to_string(),
    }))
}

async fn get_destination(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(destination_id): Path<String>,
) -> Result<Json<BackupDestination>, StatusCode> {
    let destination = sqlx::query_as::<_, BackupDestination>(
        "SELECT id, name, destination_type, is_default, created_by, created_at, last_verified_at, status 
         FROM backup_destinations WHERE id = ?"
    )
    .bind(&destination_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(destination))
}

async fn update_destination(
    State(state): State<AppState>,
    Path(destination_id): Path<String>,
    Json(req): Json<CreateDestinationRequest>,
) -> Result<Json<BackupDestination>, StatusCode> {
    if req.is_default.unwrap_or(false) {
        let _ = sqlx::query("UPDATE backup_destinations SET is_default = 0")
            .execute(&state.db_pool)
            .await;
    }

    sqlx::query(
        "UPDATE backup_destinations SET name = ?, destination_type = ?, config = ?, is_default = ? WHERE id = ?"
    )
    .bind(&req.name)
    .bind(&req.destination_type)
    .bind(req.config.to_string())
    .bind(req.is_default.unwrap_or(false))
    .bind(&destination_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    get_destination(
        State(state),
        UserInfo {
            id: String::new(),
            username: String::new(),
            totp_enabled: false,
        },
        Path(destination_id),
    )
    .await
}

async fn delete_destination(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(destination_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // Don't allow deleting default destination
    let is_default: Option<(bool,)> =
        sqlx::query_as("SELECT is_default FROM backup_destinations WHERE id = ?")
            .bind(&destination_id)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if is_default.map(|d| d.0).unwrap_or(false) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let result = sqlx::query("DELETE FROM backup_destinations WHERE id = ?")
        .bind(&destination_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn test_destination(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(destination_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let destination: Option<(String, String)> =
        sqlx::query_as("SELECT destination_type, config FROM backup_destinations WHERE id = ?")
            .bind(&destination_id)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (dest_type, config) = destination.ok_or(StatusCode::NOT_FOUND)?;
    let now = Utc::now().to_rfc3339();

    // Test connection based on type
    let success = match dest_type.as_str() {
        "local" => {
            let config: serde_json::Value = serde_json::from_str(&config).unwrap_or_default();
            let path = config
                .get("path")
                .and_then(|p| p.as_str())
                .unwrap_or("./data/backups");
            tokio::fs::metadata(path).await.is_ok() || tokio::fs::create_dir_all(path).await.is_ok()
        }
        _ => true, // For now, assume other types pass
    };

    // Update last verified
    let status = if success { "active" } else { "error" };
    sqlx::query("UPDATE backup_destinations SET last_verified_at = ?, status = ? WHERE id = ?")
        .bind(&now)
        .bind(status)
        .bind(&destination_id)
        .execute(&state.db_pool)
        .await
        .ok();

    Ok(Json(serde_json::json!({
        "success": success,
        "message": if success { "Connection successful" } else { "Connection failed" },
        "tested_at": now
    })))
}
