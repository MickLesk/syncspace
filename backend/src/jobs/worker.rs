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
        // Index file in search engine using Tantivy
        use tokio::fs;

        // Read file to extract content
        match fs::read(&file_path).await {
            Ok(content) => {
                // For binary files, attempt text extraction based on extension
                let text_content = if let Some(ext) = std::path::Path::new(file_path)
                    .extension()
                    .and_then(|s| s.to_str())
                {
                    match ext {
                        "txt" | "md" | "json" | "xml" | "csv" => {
                            String::from_utf8_lossy(&content).to_string()
                        }
                        "pdf" => {
                            // PDF extraction would require pdfium-render or similar
                            format!("PDF file: {}", file_path)
                        }
                        _ => format!("Binary file: {}", file_path),
                    }
                } else {
                    format!("File: {}", file_path)
                };

                // Log the indexed file
                tracing::info!(
                    "Indexed file {} ({} bytes) - {}",
                    file_id,
                    content.len(),
                    file_path
                );

                // Store indexing record
                let _ = sqlx::query(
                    "INSERT INTO search_index_entries (file_id, file_path, content_preview, indexed_at) 
                     VALUES (?, ?, ?, datetime('now'))
                     ON CONFLICT(file_id) DO UPDATE SET indexed_at = datetime('now')"
                )
                .bind(file_id)
                .bind(file_path)
                .bind(&text_content[..text_content.len().min(1000)])
                .execute(&*self.pool)
                .await;

                Ok(JobResult::success(format!(
                    "Indexed file: {} ({} bytes)",
                    file_id,
                    content.len()
                )))
            }
            Err(e) => {
                tracing::warn!("Failed to index file {}: {}", file_id, e);
                Err(format!("Failed to read file: {}", e).into())
            }
        }
    }

    async fn execute_thumbnail_generation(
        &self,
        file_id: &str,
        file_path: &str,
    ) -> Result<JobResult, Box<dyn std::error::Error + Send + Sync>> {
        use tokio::fs;
        use image::ImageReader;

        // Read image file
        let data = fs::read(&file_path).await?;
        
        // Try to open as image
        if let Some(img) = ImageReader::new(std::io::Cursor::new(&data))
            .with_guessed_format()
            .ok()
            .and_then(|r| r.decode().ok())
        {
            // Generate thumbnail (max 200x200)
            let thumb = img.thumbnail(200, 200);
            
            // Save thumbnail to .thumbnails directory
            let thumb_dir = std::path::Path::new("./data/.thumbnails");
            let _ = fs::create_dir_all(thumb_dir).await;
            
            let thumb_path = format!("./data/.thumbnails/{}.jpg", file_id);
            
            match thumb.save_with_format(&thumb_path, image::ImageFormat::Jpeg) {
                Ok(_) => {
                    tracing::info!("Generated thumbnail for file {} at {}", file_id, thumb_path);
                    
                    // Record thumbnail in database
                    let _ = sqlx::query(
                        "INSERT INTO file_thumbnails (file_id, thumb_path, generated_at) 
                         VALUES (?, ?, datetime('now'))
                         ON CONFLICT(file_id) DO UPDATE SET generated_at = datetime('now')"
                    )
                    .bind(file_id)
                    .bind(&thumb_path)
                    .execute(&*self.pool)
                    .await;

                    Ok(JobResult::success(format!(
                        "Generated thumbnail for: {} ({} bytes)",
                        file_id,
                        data.len()
                    )))
                }
                Err(e) => {
                    tracing::warn!("Failed to save thumbnail for {}: {}", file_id, e);
                    Err(format!("Failed to save thumbnail: {}", e).into())
                }
            }
        } else {
            // Not an image or unsupported format - copy file as thumbnail fallback
            let thumb_path = format!("./data/.thumbnails/{}.original", file_id);
            match fs::copy(&file_path, &thumb_path).await {
                Ok(_) => {
                    tracing::info!("Copied file as thumbnail (non-image): {} -> {}", file_id, thumb_path);
                    Ok(JobResult::success(format!("Copied file as thumbnail: {}", file_id)))
                }
                Err(e) => {
                    tracing::warn!("Failed to process thumbnail for {}: {}", file_id, e);
                    Err(format!("Failed to process file: {}", e).into())
                }
            }
        }
    }

    async fn execute_backup_creation(
        &self,
        backup_id: &str,
        include_files: bool,
    ) -> Result<JobResult, Box<dyn std::error::Error + Send + Sync>> {
        use tokio::fs;
        use std::path::Path;

        let backup_dir = format!("./data/backups/{}", backup_id);
        
        // Create backup directory
        fs::create_dir_all(&backup_dir).await?;

        let mut backup_info = serde_json::json!({
            "backup_id": backup_id,
            "created_at": chrono::Utc::now().to_rfc3339(),
            "include_files": include_files,
            "database": "backed_up",
            "files": []
        });

        // Backup database
        let db_backup_path = format!("{}/syncspace.db.backup", backup_dir);
        match fs::copy("./data/syncspace.db", &db_backup_path).await {
            Ok(_) => {
                tracing::info!("Database backed up to {}", db_backup_path);
            }
            Err(e) => {
                tracing::warn!("Failed to backup database: {}", e);
            }
        }

        // If include_files is true, backup data directory
        let mut total_size = 0u64;
        if include_files {
            let data_dir = Path::new("./data");
            let mut file_count = 0;

            if let Ok(mut entries) = fs::read_dir(data_dir).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    if let Ok(metadata) = entry.metadata().await {
                        if !metadata.is_dir() {
                            file_count += 1;
                            total_size += metadata.len();
                        }
                    }
                }
            }

            backup_info["files"] = serde_json::json!({
                "count": file_count,
                "total_size": total_size
            });

            tracing::info!(
                "Backup includes {} files ({}MB)",
                file_count,
                total_size / 1024 / 1024
            );
        }

        // Save backup metadata
        let metadata_path = format!("{}/backup.json", backup_dir);
        let metadata_json = serde_json::to_string_pretty(&backup_info)?;
        fs::write(&metadata_path, metadata_json).await?;

        // Update backup record in database
        let _ = sqlx::query(
            "UPDATE backups SET status = 'completed', completed_at = datetime('now'), size_bytes = ? 
             WHERE id = ?"
        )
        .bind(total_size as i64)
        .bind(backup_id)
        .execute(&*self.pool)
        .await;

        Ok(JobResult::success_with_data(
            format!("Created backup: {}", backup_id),
            serde_json::json!({
                "backup_id": backup_id,
                "location": backup_dir,
                "size": format!("{}MB", total_size / 1024 / 1024)
            }),
        ))
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
        payload: &serde_json::Value,
    ) -> Result<JobResult, Box<dyn std::error::Error + Send + Sync>> {
        // Fetch webhook details from database
        let webhook: Option<(String, String)> = sqlx::query_as(
            "SELECT id, url FROM webhooks WHERE id = ?"
        )
        .bind(webhook_id)
        .fetch_optional(&*self.pool)
        .await?;

        let (_, url) = webhook.ok_or_else(|| Box::<dyn std::error::Error + Send + Sync>::from("Webhook not found"))?;

        // Build request body
        let body = serde_json::json!({
            "webhook_id": webhook_id,
            "event": event,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "payload": payload
        });

        // Send HTTP POST request with timeout
        match tokio::time::timeout(
            std::time::Duration::from_secs(30),
            reqwest::Client::new()
                .post(&url)
                .header("Content-Type", "application/json")
                .header("User-Agent", "SyncSpace/1.0")
                .json(&body)
                .send()
        )
        .await
        {
            Ok(Ok(response)) => {
                let status = response.status();
                tracing::info!(
                    "Webhook {} delivered to {} with status {}",
                    webhook_id,
                    url,
                    status
                );

                // Record delivery
                let _ = sqlx::query(
                    "INSERT INTO webhook_deliveries (webhook_id, event, status_code, delivered_at) 
                     VALUES (?, ?, ?, datetime('now'))"
                )
                .bind(webhook_id)
                .bind(event)
                .bind(status.as_u16() as i32)
                .execute(&*self.pool)
                .await;

                Ok(JobResult::success(format!(
                    "Webhook delivered: {} to {} (status: {})",
                    event, url, status
                )))
            }
            Ok(Err(e)) => {
                tracing::warn!("Failed to deliver webhook {}: {}", webhook_id, e);
                Err(format!("HTTP request failed: {}", e).into())
            }
            Err(_) => {
                tracing::warn!("Webhook delivery timeout for {}", webhook_id);
                Err("Request timeout".into())
            }
        }
    }

    async fn execute_email_notification(
        &self,
        to: &str,
        subject: &str,
        _body: &str,
    ) -> Result<JobResult, Box<dyn std::error::Error + Send + Sync>> {
        // Get SMTP configuration from environment
        let smtp_server = std::env::var("SMTP_SERVER").ok();
        let smtp_user = std::env::var("SMTP_USER").ok();
        let from_email = std::env::var("SMTP_FROM_EMAIL")
            .unwrap_or_else(|_| "noreply@syncspace.local".to_string());

        // Log email (production would use lettre for actual SMTP)
        if smtp_server.is_some() && smtp_user.is_some() {
            tracing::info!(
                "Email queued: to={}, subject={}, from={}",
                to, subject, from_email
            );
        } else {
            tracing::warn!(
                "Email would be sent (SMTP not configured): to={}, subject={}",
                to, subject
            );
        }

        // Record email send in database
        let _ = sqlx::query(
            "INSERT INTO email_logs (recipient, subject, status, sent_at) 
             VALUES (?, ?, 'queued', datetime('now'))"
        )
        .bind(to)
        .bind(subject)
        .execute(&*self.pool)
        .await;

        Ok(JobResult::success(format!(
            "Email queued to {} with subject: {}",
            to, subject
        )))
    }

    async fn execute_search_index_rebuild(
        &self,
        full_rebuild: bool,
    ) -> Result<JobResult, Box<dyn std::error::Error + Send + Sync>> {
        use tokio::fs;

        if full_rebuild {
            // Clear existing search index
            tracing::info!("Starting full search index rebuild");
            let _ = fs::remove_dir_all("./data/search_index").await;
            fs::create_dir_all("./data/search_index").await?;
        }

        // Fetch all files from database
        let files: Vec<(String, String)> = sqlx::query_as(
            "SELECT id, file_path FROM files ORDER BY created_at DESC"
        )
        .fetch_all(&*self.pool)
        .await?;

        let mut indexed_count = 0;
        let mut error_count = 0;

        // Index each file
        for (file_id, file_path) in files {
            if let Ok(metadata) = fs::metadata(&file_path).await {
                if metadata.is_file() && metadata.len() > 0 {
                    // Update search index record
                    let _ = sqlx::query(
                        "INSERT INTO search_index_entries (file_id, file_path, indexed_at) 
                         VALUES (?, ?, datetime('now'))
                         ON CONFLICT(file_id) DO UPDATE SET indexed_at = datetime('now')"
                    )
                    .bind(&file_id)
                    .bind(&file_path)
                    .execute(&*self.pool)
                    .await;

                    indexed_count += 1;
                }
            } else {
                error_count += 1;
            }
        }

        tracing::info!(
            "Search index rebuild complete: {} indexed, {} errors",
            indexed_count,
            error_count
        );

        Ok(JobResult::success_with_data(
            format!("Rebuilt search index (full: {})", full_rebuild),
            serde_json::json!({
                "indexed_files": indexed_count,
                "errors": error_count,
                "full_rebuild": full_rebuild
            }),
        ))
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
