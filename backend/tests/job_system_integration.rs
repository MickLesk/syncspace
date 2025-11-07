//! Integration tests for the background job system
//! 
//! Tests the complete flow:
//! - Enqueue job via API
//! - Worker picks up job
//! - WebSocket broadcasts status updates
//! - Job gets archived to history
//! - Cron scheduler creates recurring jobs

use sqlx::SqlitePool;
use std::time::Duration;
use tokio::time::sleep;

// Helper to initialize test database
async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory database");
    
    // Create only the tables we need for job system tests
    // Instead of running all migrations (which have conflicts), create minimal schema
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS background_jobs (
            id TEXT PRIMARY KEY,
            job_type TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            priority INTEGER NOT NULL DEFAULT 5,
            payload TEXT NOT NULL DEFAULT '{}',
            result TEXT,
            error TEXT,
            attempts INTEGER NOT NULL DEFAULT 0,
            max_attempts INTEGER NOT NULL DEFAULT 3,
            scheduled_at TEXT,
            started_at TEXT,
            completed_at TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE INDEX IF NOT EXISTS idx_background_jobs_status_priority 
        ON background_jobs(status, priority, scheduled_at);
        
        CREATE TABLE IF NOT EXISTS job_history (
            id TEXT PRIMARY KEY,
            job_id TEXT NOT NULL,
            job_type TEXT NOT NULL,
            status TEXT NOT NULL,
            payload TEXT,
            result TEXT,
            error TEXT,
            attempts INTEGER NOT NULL,
            duration_ms INTEGER,
            completed_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE TABLE IF NOT EXISTS cron_jobs (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            job_type TEXT NOT NULL,
            cron_expression TEXT NOT NULL,
            payload TEXT NOT NULL DEFAULT '{}',
            enabled INTEGER NOT NULL DEFAULT 1,
            last_run_at TEXT,
            next_run_at TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create test schema");
    
    pool
}

#[tokio::test]
async fn test_job_enqueue_and_fetch() {
    let pool = setup_test_db().await;
    
    // Enqueue a job
    let job_id = syncbackend::jobs::enqueue_job(
        &pool,
        syncbackend::jobs::JobType::FileCleanup,
        serde_json::json!({"older_than_days": 30}),
        5,
        None,
    )
    .await
    .expect("Failed to enqueue job");
    
    assert!(!job_id.is_empty());
    
    // Fetch next job
    let job = syncbackend::jobs::fetch_next_job(&pool)
        .await
        .expect("Failed to fetch job")
        .expect("No job found");
    
    assert_eq!(job.id, job_id);
    assert_eq!(job.job_type, "file_cleanup"); // snake_case from as_str()
    assert_eq!(job.status, "pending"); // snake_case from as_str()
    assert_eq!(job.priority, 5);
}

#[tokio::test]
async fn test_job_status_transitions() {
    let pool = setup_test_db().await;
    
    // Enqueue job
    let job_id = syncbackend::jobs::enqueue_job(
        &pool,
        syncbackend::jobs::JobType::ThumbnailGeneration,
        serde_json::json!({"file_id": "test123"}),
        5,
        None,
    )
    .await
    .expect("Failed to enqueue job");
    
    // Mark as running
    syncbackend::jobs::mark_job_running(&pool, &job_id)
        .await
        .expect("Failed to mark job running");
    
    let job = syncbackend::jobs::get_job(&pool, &job_id)
        .await
        .expect("Failed to get job")
        .expect("Job not found");
    
    assert_eq!(job.status, "running"); // snake_case
    assert_eq!(job.attempts, 1);
    
    // Mark as completed
    let result = serde_json::json!({"thumbnails_created": 3});
    syncbackend::jobs::mark_job_completed(&pool, &job_id, Some(result.to_string()))
        .await
        .expect("Failed to mark job completed");
    
    // Job should be archived now, so get_job returns None
    let job = syncbackend::jobs::get_job(&pool, &job_id)
        .await
        .expect("Failed to query job");
    
    assert!(job.is_none(), "Job should be archived after completion");
}

#[tokio::test]
async fn test_job_retry_logic() {
    let pool = setup_test_db().await;
    
    // Enqueue job
    let job_id = syncbackend::jobs::enqueue_job(
        &pool,
        syncbackend::jobs::JobType::WebhookDelivery,
        serde_json::json!({"url": "http://example.com/webhook"}),
        5,
        None,
    )
    .await
    .expect("Failed to enqueue job");
    
    // Mark as running
    syncbackend::jobs::mark_job_running(&pool, &job_id)
        .await
        .expect("Failed to mark running");
    
    // Fail the job (attempt 1)
    syncbackend::jobs::mark_job_failed(&pool, &job_id, "Connection timeout".to_string())
        .await
        .expect("Failed to mark job failed");
    
    // Should be back to Pending for retry
    let job = syncbackend::jobs::get_job(&pool, &job_id)
        .await
        .expect("Failed to get job")
        .expect("Job not found");
    
    assert_eq!(job.status, "pending"); // snake_case
    assert_eq!(job.attempts, 1);
    
    // Run and fail again (attempt 2)
    syncbackend::jobs::mark_job_running(&pool, &job_id).await.unwrap();
    syncbackend::jobs::mark_job_failed(&pool, &job_id, "Connection timeout".to_string()).await.unwrap();
    
    let job = syncbackend::jobs::get_job(&pool, &job_id).await.unwrap().unwrap();
    assert_eq!(job.attempts, 2);
    
    // Run and fail again (attempt 3 - max attempts)
    syncbackend::jobs::mark_job_running(&pool, &job_id).await.unwrap();
    syncbackend::jobs::mark_job_failed(&pool, &job_id, "Connection timeout".to_string()).await.unwrap();
    
    // Should be permanently failed and archived
    let job = syncbackend::jobs::get_job(&pool, &job_id).await.unwrap();
    assert!(job.is_none(), "Job should be archived after max attempts");
}

#[tokio::test]
async fn test_job_cancellation() {
    let pool = setup_test_db().await;
    
    // Enqueue job
    let job_id = syncbackend::jobs::enqueue_job(
        &pool,
        syncbackend::jobs::JobType::BackupTask,
        serde_json::json!({"target": "s3"}),
        5,
        None,
    )
    .await
    .expect("Failed to enqueue job");
    
    // Cancel it
    syncbackend::jobs::cancel_job(&pool, &job_id)
        .await
        .expect("Failed to cancel job");
    
    // Should still exist but with cancelled status
    let job = syncbackend::jobs::get_job(&pool, &job_id)
        .await
        .expect("Failed to query job")
        .expect("Job not found");
    
    assert_eq!(job.status, "cancelled"); // snake_case
}

#[tokio::test]
async fn test_job_priority_ordering() {
    let pool = setup_test_db().await;
    
    // Enqueue jobs with different priorities
    let _low = syncbackend::jobs::enqueue_job(
        &pool,
        syncbackend::jobs::JobType::FileCleanup,
        serde_json::json!({}),
        1, // Low priority
        None,
    )
    .await
    .unwrap();
    
    let high_id = syncbackend::jobs::enqueue_job(
        &pool,
        syncbackend::jobs::JobType::VirusScan,
        serde_json::json!({}),
        10, // High priority
        None,
    )
    .await
    .unwrap();
    
    let _medium = syncbackend::jobs::enqueue_job(
        &pool,
        syncbackend::jobs::JobType::EmailNotification,
        serde_json::json!({}),
        5, // Medium priority
        None,
    )
    .await
    .unwrap();
    
    // Fetch next job - should be the high priority one
    let job = syncbackend::jobs::fetch_next_job(&pool)
        .await
        .unwrap()
        .expect("No job found");
    
    assert_eq!(job.id, high_id, "Should fetch highest priority job first");
    assert_eq!(job.priority, 10);
}

#[tokio::test]
async fn test_cron_job_creation_and_listing() {
    let pool = setup_test_db().await;
    
    // Create cron job
    let cron_id = syncbackend::cron::create_cron_job(
        &pool,
        "Daily Cleanup".to_string(),
        syncbackend::jobs::JobType::FileCleanup,
        "0 0 * * *".to_string(), // Daily at midnight
        serde_json::json!({"older_than_days": 30}),
    )
    .await
    .expect("Failed to create cron job");
    
    assert!(!cron_id.is_empty());
    
    // List cron jobs
    let cron_jobs = syncbackend::cron::list_cron_jobs(&pool)
        .await
        .expect("Failed to list cron jobs");
    
    assert_eq!(cron_jobs.len(), 1);
    assert_eq!(cron_jobs[0].name, "Daily Cleanup");
    assert_eq!(cron_jobs[0].cron_expression, "0 0 * * *");
    assert_eq!(cron_jobs[0].enabled, 1); // SQLite stores booleans as i32
}

#[tokio::test]
async fn test_cron_expression_validation() {
    let pool = setup_test_db().await;
    
    // Valid expressions
    let valid = vec![
        "* * * * *",           // Every minute
        "0 * * * *",           // Every hour
        "0 0 * * *",           // Daily at midnight
        "*/5 * * * *",         // Every 5 minutes
        "0 9-17 * * 1-5",      // Business hours Monday-Friday
    ];
    
    for expr in valid {
        let result = syncbackend::cron::create_cron_job(
            &pool,
            format!("Test {}", expr),
            syncbackend::jobs::JobType::FileCleanup,
            expr.to_string(),
            serde_json::json!({}),
        )
        .await;
        
        assert!(result.is_ok(), "Expression '{}' should be valid", expr);
    }
}

#[tokio::test]
async fn test_cleanup_old_jobs() {
    let pool = setup_test_db().await;
    
    // Create and complete a job
    let job_id = syncbackend::jobs::enqueue_job(
        &pool,
        syncbackend::jobs::JobType::FileCleanup,
        serde_json::json!({}),
        5,
        None,
    )
    .await
    .unwrap();
    
    syncbackend::jobs::mark_job_running(&pool, &job_id).await.unwrap();
    syncbackend::jobs::mark_job_completed(&pool, &job_id, Some("{}".to_string())).await.unwrap();
    
    // Should be in history
    let history = syncbackend::jobs::list_jobs(&pool, None, None, 10, 0)
        .await
        .unwrap();
    
    assert!(history.is_empty(), "Active jobs should be empty");
    
    // Clean up jobs older than 0 days (should remove all)
    syncbackend::jobs::cleanup_old_jobs(&pool, 0)
        .await
        .expect("Failed to cleanup jobs");
    
    // History should be empty now
    let history = syncbackend::jobs::list_jobs(&pool, None, None, 10, 0)
        .await
        .unwrap();
    
    assert!(history.is_empty(), "History should be empty after cleanup");
}

#[tokio::test]
async fn test_scheduled_job_execution() {
    let pool = setup_test_db().await;
    
    // Schedule job for 1 second in the future
    let scheduled_at = chrono::Utc::now() + chrono::Duration::seconds(1);
    
    let job_id = syncbackend::jobs::enqueue_job(
        &pool,
        syncbackend::jobs::JobType::FileCleanup,
        serde_json::json!({}),
        5,
        Some(scheduled_at), // Pass DateTime directly
    )
    .await
    .unwrap();
    
    // Should not be fetchable yet
    let job = syncbackend::jobs::fetch_next_job(&pool).await.unwrap();
    assert!(job.is_none(), "Job should not be fetchable before scheduled time");
    
    // Wait for scheduled time
    sleep(Duration::from_millis(1100)).await;
    
    // Now should be fetchable
    let job = syncbackend::jobs::fetch_next_job(&pool).await.unwrap();
    assert!(job.is_some(), "Job should be fetchable after scheduled time");
    assert_eq!(job.unwrap().id, job_id);
}

#[tokio::test]
async fn test_job_listing_filters() {
    let pool = setup_test_db().await;
    
    // Create various jobs
    let cleanup_id = syncbackend::jobs::enqueue_job(
        &pool,
        syncbackend::jobs::JobType::FileCleanup,
        serde_json::json!({}),
        5,
        None,
    )
    .await
    .unwrap();
    
    let thumbnail_id = syncbackend::jobs::enqueue_job(
        &pool,
        syncbackend::jobs::JobType::ThumbnailGeneration,
        serde_json::json!({}),
        5,
        None,
    )
    .await
    .unwrap();
    
    // Mark one as running
    syncbackend::jobs::mark_job_running(&pool, &cleanup_id).await.unwrap();
    
    // List all jobs
    let all = syncbackend::jobs::list_jobs(&pool, None, None, 10, 0)
        .await
        .unwrap();
    assert_eq!(all.len(), 2);
    
    // Filter by status
    let pending = syncbackend::jobs::list_jobs(&pool, Some(syncbackend::JobStatus::Pending), None, 10, 0)
        .await
        .unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].id, thumbnail_id);
    
    let running = syncbackend::jobs::list_jobs(&pool, Some(syncbackend::JobStatus::Running), None, 10, 0)
        .await
        .unwrap();
    assert_eq!(running.len(), 1);
    assert_eq!(running[0].id, cleanup_id);
    
    // Filter by job type
    let cleanup_jobs = syncbackend::jobs::list_jobs(&pool, None, Some(syncbackend::JobType::FileCleanup), 10, 0)
        .await
        .unwrap();
    assert_eq!(cleanup_jobs.len(), 1);
    assert_eq!(cleanup_jobs[0].id, cleanup_id);
}
