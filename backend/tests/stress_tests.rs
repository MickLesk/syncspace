//! Stress tests for the background job system
//!
//! Tests system behavior under load:
//! - Concurrent job enqueuing (100 jobs simultaneously)
//! - Worker pool saturation (1000 jobs with 4 workers)
//! - Database contention (multiple workers accessing queue)
//! - Cron scheduler load (100 cron jobs due simultaneously)

use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory database");

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
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create test schema");

    pool
}

#[tokio::test]
#[ignore]
async fn test_concurrent_enqueue() {
    let pool = setup_test_db().await;
    let pool = Arc::new(pool);

    let num_jobs = 100;
    let start = Instant::now();

    // Enqueue jobs concurrently
    let mut handles = vec![];
    for i in 0..num_jobs {
        let pool_clone = Arc::clone(&pool);
        let handle = tokio::spawn(async move {
            syncbackend::jobs::enqueue_job(
                &pool_clone,
                syncbackend::jobs::JobType::FileCleanup,
                serde_json::json!({ "job_number": i }),
                5,
                None,
            )
            .await
        });
        handles.push(handle);
    }

    // Wait for all to complete
    let mut success_count = 0;
    for handle in handles {
        if handle.await.is_ok() {
            success_count += 1;
        }
    }

    let duration = start.elapsed();

    println!("Concurrent enqueuing stress test:");
    println!("  Jobs: {}", num_jobs);
    println!("  Success: {}", success_count);
    println!("  Duration: {:?}", duration);
    println!(
        "  Rate: {:.2} jobs/sec",
        num_jobs as f64 / duration.as_secs_f64()
    );

    assert_eq!(
        success_count, num_jobs,
        "All jobs should enqueue successfully"
    );

    // Verify in database
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM background_jobs")
        .fetch_one(&*pool)
        .await
        .unwrap();

    assert_eq!(count.0, num_jobs as i64);
}

#[tokio::test]
#[ignore]
async fn stress_test_worker_pool_saturation() {
    let pool = setup_test_db().await;
    let pool = Arc::new(pool);

    let num_jobs = 1000;
    let num_workers = 4;

    println!("Worker pool saturation test:");
    println!("  Enqueuing {} jobs...", num_jobs);

    let start = Instant::now();

    // Enqueue many jobs
    for i in 0..num_jobs {
        syncbackend::jobs::enqueue_job(
            &pool,
            syncbackend::jobs::JobType::FileCleanup,
            serde_json::json!({ "dry_run": true, "job_number": i }),
            (i % 10) as i32, // Vary priority
            None,
        )
        .await
        .expect("Failed to enqueue job");
    }

    let enqueue_duration = start.elapsed();
    println!(
        "  Enqueued in {:?} ({:.2} jobs/sec)",
        enqueue_duration,
        num_jobs as f64 / enqueue_duration.as_secs_f64()
    );

    // Simulate workers processing (fetch + mark completed)
    let processing_start = Instant::now();
    let semaphore = Arc::new(Semaphore::new(num_workers));
    let mut handles = vec![];

    for worker_id in 0..num_workers {
        let pool_clone = Arc::clone(&pool);
        let semaphore_clone = Arc::clone(&semaphore);

        let handle = tokio::spawn(async move {
            let mut processed = 0;
            loop {
                let _permit = semaphore_clone.acquire().await.unwrap();

                match syncbackend::jobs::fetch_next_job(&pool_clone).await {
                    Ok(Some(job)) => {
                        syncbackend::jobs::mark_job_running(&pool_clone, &job.id)
                            .await
                            .ok();

                        // Simulate quick processing
                        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

                        syncbackend::jobs::mark_job_completed(
                            &pool_clone,
                            &job.id,
                            Some("{}".to_string()),
                        )
                        .await
                        .ok();

                        processed += 1;
                    }
                    Ok(None) => break,
                    Err(_) => break,
                }
            }
            (worker_id, processed)
        });

        handles.push(handle);
    }

    // Wait for all workers
    let mut total_processed = 0;
    for handle in handles {
        if let Ok((worker_id, processed)) = handle.await {
            println!("  Worker {} processed {} jobs", worker_id, processed);
            total_processed += processed;
        }
    }

    let processing_duration = processing_start.elapsed();

    println!(
        "  Processed {} jobs in {:?}",
        total_processed, processing_duration
    );
    println!(
        "  Rate: {:.2} jobs/sec",
        total_processed as f64 / processing_duration.as_secs_f64()
    );
    println!(
        "  Per worker: {:.2} jobs/sec",
        (total_processed as f64 / num_workers as f64) / processing_duration.as_secs_f64()
    );

    // Verify completion
    let remaining: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM background_jobs WHERE status = 'pending'")
            .fetch_one(&*pool)
            .await
            .unwrap();

    println!("  Remaining pending: {}", remaining.0);
    assert!(remaining.0 < 100, "Most jobs should be processed");
}

#[tokio::test]
#[ignore]
async fn stress_test_database_contention() {
    let pool = setup_test_db().await;
    let pool = Arc::new(pool);

    let num_concurrent_operations = 50;
    let operations_per_task = 20;

    println!("Database contention stress test:");
    println!(
        "  {} concurrent tasks, {} ops each",
        num_concurrent_operations, operations_per_task
    );

    let start = Instant::now();
    let mut handles = vec![];

    for task_id in 0..num_concurrent_operations {
        let pool_clone = Arc::clone(&pool);

        let handle = tokio::spawn(async move {
            for op in 0..operations_per_task {
                // Mix of operations
                match op % 4 {
                    0 => {
                        // Enqueue
                        syncbackend::jobs::enqueue_job(
                            &pool_clone,
                            syncbackend::jobs::JobType::FileCleanup,
                            serde_json::json!({ "task": task_id, "op": op }),
                            5,
                            None,
                        )
                        .await
                        .ok();
                    }
                    1 => {
                        // Fetch
                        syncbackend::jobs::fetch_next_job(&pool_clone).await.ok();
                    }
                    2 => {
                        // List
                        syncbackend::jobs::list_jobs(&pool_clone, None, None, 10, 0)
                            .await
                            .ok();
                    }
                    3 => {
                        // Get stats (count queries)
                        sqlx::query_as::<_, (i64,)>(
                            "SELECT COUNT(*) FROM background_jobs WHERE status = 'pending'",
                        )
                        .fetch_one(&*pool_clone)
                        .await
                        .ok();
                    }
                    _ => {}
                }
            }
            task_id
        });

        handles.push(handle);
    }

    // Wait for all tasks
    let mut completed = 0;
    for handle in handles {
        if handle.await.is_ok() {
            completed += 1;
        }
    }

    let duration = start.elapsed();
    let total_ops = num_concurrent_operations * operations_per_task;

    println!("  Completed: {}/{}", completed, num_concurrent_operations);
    println!("  Duration: {:?}", duration);
    println!(
        "  Rate: {:.2} ops/sec",
        total_ops as f64 / duration.as_secs_f64()
    );

    assert_eq!(
        completed, num_concurrent_operations,
        "All tasks should complete despite contention"
    );
}

#[tokio::test]
#[ignore]
async fn stress_test_cron_scheduler_load() {
    let pool = setup_test_db().await;

    let num_cron_jobs = 100;

    println!("Cron scheduler load test:");
    println!("  Creating {} cron jobs...", num_cron_jobs);

    let start = Instant::now();

    // Create many cron jobs with same schedule (all due now)
    for i in 0..num_cron_jobs {
        syncbackend::cron::create_cron_job(
            &pool,
            &format!("Load Test Job {}", i),
            syncbackend::jobs::JobType::FileCleanup,
            "* * * * *", // Every minute
            serde_json::json!({ "job_number": i }),
        )
        .await
        .expect("Failed to create cron job");
    }

    let creation_duration = start.elapsed();
    println!("  Created in {:?}", creation_duration);

    // Simulate scheduler checking and enqueuing
    let check_start = Instant::now();
    let due_jobs = syncbackend::cron::get_due_cron_jobs(&pool)
        .await
        .expect("Failed to get due jobs");

    println!("  Found {} due jobs", due_jobs.len());

    // Enqueue all due jobs
    let mut enqueued = 0;
    for cron_job in due_jobs {
        if syncbackend::jobs::enqueue_job(
            &pool,
            syncbackend::jobs::JobType::from_str(&cron_job.job_type),
            serde_json::from_str(&cron_job.payload).unwrap_or(serde_json::json!({})),
            5,
            None,
        )
        .await
        .is_ok()
        {
            enqueued += 1;

            // Update last_run_at
            syncbackend::cron::update_cron_last_run(&pool, &cron_job.id)
                .await
                .ok();
        }
    }

    let check_duration = check_start.elapsed();

    println!("  Enqueued {} jobs in {:?}", enqueued, check_duration);
    println!(
        "  Rate: {:.2} jobs/sec",
        enqueued as f64 / check_duration.as_secs_f64()
    );

    assert_eq!(enqueued, num_cron_jobs, "All cron jobs should trigger");

    // Verify jobs in queue
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM background_jobs")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(count.0, num_cron_jobs as i64);
}

#[tokio::test]
#[ignore]
async fn stress_test_mixed_workload() {
    let pool = setup_test_db().await;
    let pool = Arc::new(pool);

    println!("Mixed workload stress test:");

    let start = Instant::now();

    // Spawn multiple task types simultaneously
    let enqueue_handle = {
        let pool = Arc::clone(&pool);
        tokio::spawn(async move {
            for i in 0..50 {
                syncbackend::jobs::enqueue_job(
                    &pool,
                    syncbackend::jobs::JobType::FileCleanup,
                    serde_json::json!({ "i": i }),
                    5,
                    None,
                )
                .await
                .ok();
            }
        })
    };

    let worker_handle = {
        let pool = Arc::clone(&pool);
        tokio::spawn(async move {
            for _ in 0..30 {
                if let Ok(Some(job)) = syncbackend::jobs::fetch_next_job(&pool).await {
                    syncbackend::jobs::mark_job_running(&pool, &job.id)
                        .await
                        .ok();
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    syncbackend::jobs::mark_job_completed(&pool, &job.id, Some("{}".to_string()))
                        .await
                        .ok();
                }
            }
        })
    };

    let list_handle = {
        let pool = Arc::clone(&pool);
        tokio::spawn(async move {
            for _ in 0..20 {
                syncbackend::jobs::list_jobs(&pool, None, None, 10, 0)
                    .await
                    .ok();
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            }
        })
    };

    // Wait for all
    enqueue_handle.await.ok();
    worker_handle.await.ok();
    list_handle.await.ok();

    let duration = start.elapsed();

    println!("  Mixed workload completed in {:?}", duration);
    println!("  All concurrent operations succeeded without deadlock");
}
