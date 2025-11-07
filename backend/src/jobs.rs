//! Background Job System
//! 
//! Provides asynchronous task processing with retry logic, priority queuing,
//! and cron scheduling for recurring tasks.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

// ============================================================================
// Job Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum JobType {
    SearchIndexing,
    ThumbnailGeneration,
    FileCleanup,
    BackupTask,
    EmailNotification,
    WebhookDelivery,
    FileCompression,
    VirusScan,
    Custom(String),
}

impl JobType {
    pub fn as_str(&self) -> &str {
        match self {
            JobType::SearchIndexing => "search_indexing",
            JobType::ThumbnailGeneration => "thumbnail_generation",
            JobType::FileCleanup => "file_cleanup",
            JobType::BackupTask => "backup_task",
            JobType::EmailNotification => "email_notification",
            JobType::WebhookDelivery => "webhook_delivery",
            JobType::FileCompression => "file_compression",
            JobType::VirusScan => "virus_scan",
            JobType::Custom(s) => s,
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "search_indexing" => JobType::SearchIndexing,
            "thumbnail_generation" => JobType::ThumbnailGeneration,
            "file_cleanup" => JobType::FileCleanup,
            "backup_task" => JobType::BackupTask,
            "email_notification" => JobType::EmailNotification,
            "webhook_delivery" => JobType::WebhookDelivery,
            "file_compression" => JobType::FileCompression,
            "virus_scan" => JobType::VirusScan,
            s => JobType::Custom(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl JobStatus {
    pub fn as_str(&self) -> &str {
        match self {
            JobStatus::Pending => "pending",
            JobStatus::Running => "running",
            JobStatus::Completed => "completed",
            JobStatus::Failed => "failed",
            JobStatus::Cancelled => "cancelled",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "pending" => JobStatus::Pending,
            "running" => JobStatus::Running,
            "completed" => JobStatus::Completed,
            "failed" => JobStatus::Failed,
            "cancelled" => JobStatus::Cancelled,
            _ => JobStatus::Pending,
        }
    }
}

// ============================================================================
// Database Models
// ============================================================================

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct BackgroundJob {
    pub id: String,
    pub job_type: String,
    pub status: String,
    pub priority: i32,
    pub payload: String,
    pub result: Option<String>,
    pub error: Option<String>,
    pub attempts: i32,
    pub max_attempts: i32,
    pub scheduled_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct JobHistory {
    pub id: String,
    pub job_id: String,
    pub job_type: String,
    pub status: String,
    pub payload: String,
    pub result: Option<String>,
    pub error: Option<String>,
    pub attempts: i32,
    pub duration_ms: Option<i32>,
    pub completed_at: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CronJob {
    pub id: String,
    pub name: String,
    pub job_type: String,
    pub cron_expression: String,
    pub payload: String,
    pub enabled: i32,
    pub last_run_at: Option<String>,
    pub next_run_at: String,
    pub created_at: String,
    pub updated_at: String,
}

// ============================================================================
// Job Queue Operations
// ============================================================================

/// Enqueue a new background job
pub async fn enqueue_job(
    pool: &SqlitePool,
    job_type: JobType,
    payload: serde_json::Value,
    priority: i32,
    scheduled_at: Option<DateTime<Utc>>,
) -> Result<String, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let job_type_str = job_type.as_str();
    let payload_str = serde_json::to_string(&payload).unwrap_or_default();
    let scheduled = scheduled_at.unwrap_or_else(Utc::now).to_rfc3339();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"
        INSERT INTO background_jobs (id, job_type, status, priority, payload, scheduled_at, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&id)
    .bind(job_type_str)
    .bind(JobStatus::Pending.as_str())
    .bind(priority)
    .bind(&payload_str)
    .bind(&scheduled)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;
    
    tracing::info!("Enqueued job {} of type {}", id, job_type_str);
    Ok(id)
}

/// Fetch next available job for processing
pub async fn fetch_next_job(pool: &SqlitePool) -> Result<Option<BackgroundJob>, sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    
    // Get next pending job with highest priority that is scheduled to run
    let job = sqlx::query_as::<_, BackgroundJob>(
        r#"
        SELECT * FROM background_jobs
        WHERE status = ? AND scheduled_at <= ?
        ORDER BY priority DESC, scheduled_at ASC
        LIMIT 1
        "#
    )
    .bind(JobStatus::Pending.as_str())
    .bind(&now)
    .fetch_optional(pool)
    .await?;
    
    Ok(job)
}

/// Mark job as running
pub async fn mark_job_running(pool: &SqlitePool, job_id: &str) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"
        UPDATE background_jobs
        SET status = ?, started_at = ?, updated_at = ?, attempts = attempts + 1
        WHERE id = ?
        "#
    )
    .bind(JobStatus::Running.as_str())
    .bind(&now)
    .bind(&now)
    .bind(job_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Mark job as completed
pub async fn mark_job_completed(
    pool: &SqlitePool,
    job_id: &str,
    result: Option<String>,
) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"
        UPDATE background_jobs
        SET status = ?, completed_at = ?, updated_at = ?, result = ?
        WHERE id = ?
        "#
    )
    .bind(JobStatus::Completed.as_str())
    .bind(&now)
    .bind(&now)
    .bind(result)
    .bind(job_id)
    .execute(pool)
    .await?;
    
    // Archive to history
    archive_job(pool, job_id).await?;
    
    tracing::info!("Job {} completed successfully", job_id);
    Ok(())
}

/// Mark job as failed
pub async fn mark_job_failed(
    pool: &SqlitePool,
    job_id: &str,
    error: String,
) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    
    // Get current job to check attempts
    let job = sqlx::query_as::<_, BackgroundJob>(
        "SELECT * FROM background_jobs WHERE id = ?"
    )
    .bind(job_id)
    .fetch_one(pool)
    .await?;
    
    if job.attempts >= job.max_attempts {
        // Permanently failed
        sqlx::query(
            r#"
            UPDATE background_jobs
            SET status = ?, completed_at = ?, updated_at = ?, error = ?
            WHERE id = ?
            "#
        )
        .bind(JobStatus::Failed.as_str())
        .bind(&now)
        .bind(&now)
        .bind(&error)
        .bind(job_id)
        .execute(pool)
        .await?;
        
        // Archive to history
        archive_job(pool, job_id).await?;
        
        tracing::error!("Job {} permanently failed after {} attempts: {}", job_id, job.attempts, error);
    } else {
        // Retry with exponential backoff
        let backoff_secs = 2_i64.pow(job.attempts as u32) * 60; // 1min, 2min, 4min, 8min...
        let next_run = Utc::now() + chrono::Duration::seconds(backoff_secs);
        
        sqlx::query(
            r#"
            UPDATE background_jobs
            SET status = ?, scheduled_at = ?, updated_at = ?, error = ?
            WHERE id = ?
            "#
        )
        .bind(JobStatus::Pending.as_str())
        .bind(next_run.to_rfc3339())
        .bind(&now)
        .bind(&error)
        .bind(job_id)
        .execute(pool)
        .await?;
        
        tracing::warn!("Job {} failed (attempt {}), retrying in {}s", job_id, job.attempts, backoff_secs);
    }
    
    Ok(())
}

/// Archive completed/failed job to history
async fn archive_job(pool: &SqlitePool, job_id: &str) -> Result<(), sqlx::Error> {
    // Calculate duration if job has started
    let job = sqlx::query_as::<_, BackgroundJob>(
        "SELECT * FROM background_jobs WHERE id = ?"
    )
    .bind(job_id)
    .fetch_one(pool)
    .await?;
    
    let duration_ms = if let (Some(started), Some(completed)) = (&job.started_at, &job.completed_at) {
        if let (Ok(start), Ok(end)) = (
            DateTime::parse_from_rfc3339(started),
            DateTime::parse_from_rfc3339(completed)
        ) {
            Some((end.timestamp_millis() - start.timestamp_millis()) as i32)
        } else {
            None
        }
    } else {
        None
    };
    
    let history_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"
        INSERT INTO job_history (id, job_id, job_type, status, payload, result, error, attempts, duration_ms, completed_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&history_id)
    .bind(&job.id)
    .bind(&job.job_type)
    .bind(&job.status)
    .bind(&job.payload)
    .bind(&job.result)
    .bind(&job.error)
    .bind(job.attempts)
    .bind(duration_ms)
    .bind(&now)
    .execute(pool)
    .await?;
    
    // Delete from active queue
    sqlx::query("DELETE FROM background_jobs WHERE id = ?")
        .bind(job_id)
        .execute(pool)
        .await?;
    
    Ok(())
}

/// Cancel a pending job
pub async fn cancel_job(pool: &SqlitePool, job_id: &str) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"
        UPDATE background_jobs
        SET status = ?, updated_at = ?
        WHERE id = ? AND status = ?
        "#
    )
    .bind(JobStatus::Cancelled.as_str())
    .bind(&now)
    .bind(job_id)
    .bind(JobStatus::Pending.as_str())
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Get job by ID
pub async fn get_job(pool: &SqlitePool, job_id: &str) -> Result<Option<BackgroundJob>, sqlx::Error> {
    sqlx::query_as::<_, BackgroundJob>("SELECT * FROM background_jobs WHERE id = ?")
        .bind(job_id)
        .fetch_optional(pool)
        .await
}

/// List jobs with filters
pub async fn list_jobs(
    pool: &SqlitePool,
    status: Option<JobStatus>,
    job_type: Option<JobType>,
    limit: i32,
    offset: i32,
) -> Result<Vec<BackgroundJob>, sqlx::Error> {
    let mut query = String::from("SELECT * FROM background_jobs WHERE 1=1");
    
    if let Some(s) = status {
        query.push_str(&format!(" AND status = '{}'", s.as_str()));
    }
    
    if let Some(jt) = job_type {
        query.push_str(&format!(" AND job_type = '{}'", jt.as_str()));
    }
    
    query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");
    
    sqlx::query_as::<_, BackgroundJob>(&query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

/// Clean old completed jobs (keep only last N days)
pub async fn cleanup_old_jobs(pool: &SqlitePool, keep_days: i64) -> Result<u64, sqlx::Error> {
    let cutoff = Utc::now() - chrono::Duration::days(keep_days);
    let cutoff_str = cutoff.to_rfc3339();
    
    let result = sqlx::query(
        r#"
        DELETE FROM background_jobs
        WHERE status IN (?, ?) AND completed_at < ?
        "#
    )
    .bind(JobStatus::Completed.as_str())
    .bind(JobStatus::Failed.as_str())
    .bind(&cutoff_str)
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected())
}
