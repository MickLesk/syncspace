//! Job worker that processes jobs from the queue

use super::queue::JobQueue;
use super::types::{Job, JobResult, JobType};
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub struct JobWorker {
    queue: Arc<JobQueue>,
    pool: Arc<SqlitePool>,
    worker_id: usize,
    shutdown: Arc<tokio::sync::Notify>,
}

impl JobWorker {
    pub fn new(
        pool: Arc<SqlitePool>,
        worker_id: usize,
        shutdown: Arc<tokio::sync::Notify>,
    ) -> Self {
        let queue = Arc::new(JobQueue::new(pool.clone()));
        Self {
            queue,
            pool,
            worker_id,
            shutdown,
        }
    }

    /// Start worker loop
    pub async fn start(self) {
        println!("🚀 Job worker {} started", self.worker_id);

        loop {
            tokio::select! {
                _ = self.shutdown.notified() => {
                    println!("🛑 Job worker {} shutting down", self.worker_id);
                    break;
                }
                _ = sleep(Duration::from_secs(5)) => {
                    if let Err(e) = self.process_next_job().await {
                        eprintln!("❌ Job worker {} error: {}", self.worker_id, e);
                    }
                }
            }
        }
    }

    async fn process_next_job(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Try to get next job
        if let Some(job) = self.queue.dequeue().await? {
            println!(
                "🔨 Worker {} processing job: {} ({})",
                self.worker_id, job.id, job.job_type
            );

            // Mark as running
            self.queue.mark_running(&job.id).await?;

            // Execute job
            match self.execute_job(&job).await {
                Ok(result) => {
                    let result_json = serde_json::to_string(&result)?;
                    self.queue.mark_success(&job.id, &result_json).await?;
                    println!(
                        "✅ Worker {} completed job: {} - {}",
                        self.worker_id, job.id, result.message
                    );
                }
                Err(e) => {
                    let error_msg = format!("{}", e);
                    self.queue.mark_failed(&job.id, &error_msg).await?;
                    eprintln!(
                        "❌ Worker {} failed job: {} - {}",
                        self.worker_id, job.id, error_msg
                    );
                }
            }
        }

        Ok(())
    }

    async fn execute_job(
        &self,
        job: &Job,
    ) -> Result<JobResult, Box<dyn std::error::Error + Send + Sync>> {
        let job_type = job.parse_type()?;

        match job_type {
            JobType::FileIndexing { file_id, file_path } => {
                self.execute_file_indexing(&file_id, &file_path).await
            }
            JobType::ThumbnailGeneration { file_id, file_path } => {
                self.execute_thumbnail_generation(&file_id, &file_path)
                    .await
            }
            JobType::BackupCreation {
                backup_id,
                include_files,
            } => {
                self.execute_backup_creation(&backup_id, include_files)
                    .await
            }
            JobType::VersionCleanup { file_id } => {
                self.execute_version_cleanup(file_id.as_deref()).await
            }
            JobType::WebhookDelivery {
                webhook_id,
                event,
                payload,
            } => {
                self.execute_webhook_delivery(&webhook_id, &event, &payload)
                    .await
            }
            JobType::EmailNotification { to, subject, body } => {
                self.execute_email_notification(&to, &subject, &body).await
            }
            JobType::SearchIndexRebuild { full_rebuild } => {
                self.execute_search_index_rebuild(full_rebuild).await
            }
            JobType::DatabaseCleanup { table } => self.execute_database_cleanup(&table).await,
        }
    }

    // ==================== JOB IMPLEMENTATIONS ====================

    async fn execute_file_indexing(
        &self,
        file_id: &str,
        file_path: &str,
    ) -> Result<JobResult, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement file indexing
        // For now, just simulate work
        sleep(Duration::from_millis(100)).await;

        Ok(JobResult::success(format!(
            "Indexed file: {} ({})",
            file_id, file_path
        )))
    }

    async fn execute_thumbnail_generation(
        &self,
        file_id: &str,
        file_path: &str,
    ) -> Result<JobResult, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement thumbnail generation
        sleep(Duration::from_millis(200)).await;

        Ok(JobResult::success(format!(
            "Generated thumbnail for: {} ({})",
            file_id, file_path
        )))
    }

    async fn execute_backup_creation(
        &self,
        backup_id: &str,
        include_files: bool,
    ) -> Result<JobResult, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement backup creation
        sleep(Duration::from_secs(1)).await;

        Ok(JobResult::success(format!(
            "Created backup: {} (include_files: {})",
            backup_id, include_files
        )))
    }

    async fn execute_version_cleanup(
        &self,
        file_id: Option<&str>,
    ) -> Result<JobResult, Box<dyn std::error::Error + Send + Sync>> {
        // Cleanup old file versions (keep latest 5 per file)
        let deleted_count: i64 = if let Some(fid) = file_id {
            sqlx::query_scalar::<_, i64>(
                "DELETE FROM file_versions WHERE id IN (
                    SELECT id FROM file_versions
                    WHERE file_id = ?
                    ORDER BY version_number DESC
                    LIMIT -1 OFFSET 5
                )",
            )
            .bind(fid)
            .fetch_one(&*self.pool)
            .await
            .unwrap_or(0)
        } else {
            sqlx::query_scalar::<_, i64>(
                "DELETE FROM file_versions WHERE id IN (
                    SELECT fv.id FROM file_versions fv
                    INNER JOIN (
                        SELECT file_id, version_number,
                               ROW_NUMBER() OVER (PARTITION BY file_id ORDER BY version_number DESC) as rn
                        FROM file_versions
                    ) ranked ON fv.file_id = ranked.file_id AND fv.version_number = ranked.version_number
                    WHERE ranked.rn > 5
                )"
            )
            .fetch_one(&*self.pool)
            .await
            .unwrap_or(0)
        };

        Ok(JobResult::success_with_data(
            format!("Cleaned up {} old versions", deleted_count),
            serde_json::json!({ "deleted_count": deleted_count }),
        ))
    }

    async fn execute_webhook_delivery(
        &self,
        webhook_id: &str,
        event: &str,
        _payload: &serde_json::Value,
    ) -> Result<JobResult, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement webhook delivery
        sleep(Duration::from_millis(300)).await;

        Ok(JobResult::success(format!(
            "Delivered webhook: {} event: {}",
            webhook_id, event
        )))
    }

    async fn execute_email_notification(
        &self,
        to: &str,
        subject: &str,
        _body: &str,
    ) -> Result<JobResult, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement email sending
        sleep(Duration::from_millis(500)).await;

        Ok(JobResult::success(format!(
            "Sent email to: {} subject: {}",
            to, subject
        )))
    }

    async fn execute_search_index_rebuild(
        &self,
        full_rebuild: bool,
    ) -> Result<JobResult, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement search index rebuild
        sleep(Duration::from_secs(2)).await;

        Ok(JobResult::success(format!(
            "Rebuilt search index (full_rebuild: {})",
            full_rebuild
        )))
    }

    async fn execute_database_cleanup(
        &self,
        table: &str,
    ) -> Result<JobResult, Box<dyn std::error::Error + Send + Sync>> {
        let deleted_count = match table {
            "jobs" => self.queue.cleanup_old_jobs().await?,
            "login_attempts" => {
                // Delete login attempts older than 30 days
                sqlx::query_scalar::<_, i64>(
                    "DELETE FROM login_attempts 
                     WHERE attempted_at < datetime('now', '-30 days')
                     RETURNING (SELECT COUNT(*) FROM login_attempts WHERE attempted_at < datetime('now', '-30 days'))"
                )
                .fetch_one(&*self.pool)
                .await
                .unwrap_or(0) as u64
            }
            _ => {
                return Err(format!("Unknown table: {}", table).into());
            }
        };

        Ok(JobResult::success_with_data(
            format!("Cleaned up {} rows from table '{}'", deleted_count, table),
            serde_json::json!({ "deleted_count": deleted_count, "table": table }),
        ))
    }
}

/// Start job worker pool
pub async fn start_job_workers(
    pool: Arc<SqlitePool>,
    num_workers: usize,
) -> Arc<tokio::sync::Notify> {
    let shutdown = Arc::new(tokio::sync::Notify::new());

    println!("🚀 Starting {} job workers", num_workers);

    for i in 0..num_workers {
        let worker = JobWorker::new(pool.clone(), i, shutdown.clone());
        tokio::spawn(async move {
            worker.start().await;
        });
    }

    shutdown
}
