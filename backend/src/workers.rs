//! Job Worker Pool
//!
//! Manages Tokio tasks for parallel job execution with graceful shutdown.

// TODO: Re-enable after job system API is fixed
// use crate::jobs::{BackgroundJob, JobStatus, JobType, fetch_next_job, mark_job_running, mark_job_completed, mark_job_failed};
use crate::websocket::FileChangeEvent;
use sqlx::SqlitePool;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::{broadcast::Sender, Semaphore};

/// Job worker pool configuration
#[allow(dead_code)]
pub struct WorkerPool {
    pool: SqlitePool,
    fs_tx: Sender<FileChangeEvent>,
    num_workers: usize,
    shutdown: Arc<AtomicBool>,
    semaphore: Arc<Semaphore>,
}

impl WorkerPool {
    /// Create new worker pool
    pub fn new(pool: SqlitePool, fs_tx: Sender<FileChangeEvent>, num_workers: usize) -> Self {
        Self {
            pool,
            fs_tx,
            num_workers,
            shutdown: Arc::new(AtomicBool::new(false)),
            semaphore: Arc::new(Semaphore::new(num_workers)),
        }
    }

    /// Start worker pool
    pub async fn start(&self) {
        tracing::info!("🚀 Job worker pool (old system) - DISABLED pending refactor");
        // TODO: Re-enable after job system refactor
        /*
        tracing::info!("🚀 Starting job worker pool with {} workers", self.num_workers);

        let mut handles = Vec::new();

        for worker_id in 0..self.num_workers {
            let pool = self.pool.clone();
            let shutdown = self.shutdown.clone();
            let semaphore = self.semaphore.clone();
            let fs_tx = self.fs_tx.clone();

            let handle = tokio::spawn(async move {
                worker_loop(worker_id, pool, shutdown, semaphore, fs_tx).await;
            });

            handles.push(handle);
        }

        // Wait for all workers to finish
        for handle in handles {
            let _ = handle.await;
        }

        tracing::info!("✅ All workers stopped");
        */
    }

    /// Signal graceful shutdown
    pub fn shutdown(&self) {
        tracing::info!("⏹️ Shutting down worker pool...");
        self.shutdown.store(true, Ordering::Relaxed);
    }
}

/*
// TODO: Re-enable after job system refactor
/// Worker loop - continuously fetch and process jobs
async fn worker_loop(
    worker_id: usize,
    pool: SqlitePool,
    shutdown: Arc<AtomicBool>,
    semaphore: Arc<Semaphore>,
    fs_tx: Sender<FileChangeEvent>,
) {
    tracing::info!("Worker {} started", worker_id);

    while !shutdown.load(Ordering::Relaxed) {
        // Acquire semaphore permit (rate limiting)
        let _permit = semaphore.acquire().await.unwrap();

        // Fetch next job
        match fetch_next_job(&pool).await {
            Ok(Some(job)) => {
                tracing::info!("Worker {} processing job {}", worker_id, job.id);

                // Mark as running
                if let Err(e) = mark_job_running(&pool, &job.id).await {
                    tracing::error!("Worker {} failed to mark job running: {}", worker_id, e);
                    continue;
                }

                // Broadcast running event
                let _ = fs_tx.send(
                    FileChangeEvent::new(job.id.clone(), "job:running".to_string())
                        .with_metadata(serde_json::json!({
                            "job_type": job.job_type,
                            "status": "Running",
                            "attempts": job.attempts,
                            "priority": job.priority,
                        })),
                );

                // Process job
                match process_job(&pool, &job).await {
                    Ok(result) => {
                        if let Err(e) = mark_job_completed(&pool, &job.id, result.clone()).await {
                            tracing::error!("Worker {} failed to mark job completed: {}", worker_id, e);
                        } else {
                            // Broadcast completed event
                            let _ = fs_tx.send(
                                FileChangeEvent::new(job.id.clone(), "job:completed".to_string())
                                    .with_metadata(serde_json::json!({
                                        "job_type": job.job_type,
                                        "status": "Completed",
                                        "result": result,
                                    })),
                            );
                        }
                    }
                    Err(error) => {
                        if let Err(e) = mark_job_failed(&pool, &job.id, error.clone()).await {
                            tracing::error!("Worker {} failed to mark job failed: {}", worker_id, e);
                        } else {
                            // Broadcast failed event
                            let _ = fs_tx.send(
                                FileChangeEvent::new(job.id.clone(), "job:failed".to_string())
                                    .with_metadata(serde_json::json!({
                                        "job_type": job.job_type,
                                        "status": "Failed",
                                        "error": error,
                                    })),
                            );
                        }
                    }
                }
            }
            Ok(None) => {
                // No jobs available, sleep for a bit
                sleep(Duration::from_secs(5)).await;
            }
            Err(e) => {
                tracing::error!("Worker {} error fetching job: {}", worker_id, e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }

    tracing::info!("Worker {} stopped", worker_id);
}

/// Process a single job based on its type
async fn process_job(pool: &SqlitePool, job: &BackgroundJob) -> Result<Option<String>, String> {
    let job_type = JobType::from_str(&job.job_type);

    tracing::info!("Processing job {} of type {:?}", job.id, job_type);

    let result = match job_type {
        JobType::SearchIndexing => process_search_indexing(pool, job).await,
        JobType::ThumbnailGeneration => process_thumbnail_generation(pool, job).await,
        JobType::FileCleanup => process_file_cleanup(pool, job).await,
        JobType::BackupTask => process_backup_task(pool, job).await,
        JobType::EmailNotification => process_email_notification(pool, job).await,
        JobType::WebhookDelivery => process_webhook_delivery(pool, job).await,
        JobType::FileCompression => process_file_compression(pool, job).await,
        JobType::VirusScan => process_virus_scan(pool, job).await,
        JobType::Custom(name) => {
            tracing::warn!("Unknown custom job type: {}", name);
            Err(format!("Unknown job type: {}", name))
        }
    };

    result
}

// ============================================================================
// Job Handlers
// ============================================================================

async fn process_search_indexing(_pool: &SqlitePool, job: &BackgroundJob) -> Result<Option<String>, String> {
    use std::path::Path;

    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .map_err(|e| format!("Invalid payload: {}", e))?;

    let target_dir = payload["target_dir"]
        .as_str()
        .unwrap_or("./data");
    let force_reindex = payload["force_reindex"]
        .as_bool()
        .unwrap_or(false);

    tracing::info!("Search indexing: dir={}, force={}", target_dir, force_reindex);

    let start = std::time::Instant::now();
    let mut indexed_count = 0;
    let mut errors = Vec::new();

    // Recursively scan directory for indexable files
    fn scan_dir(dir: &Path, indexed: &mut u32, errors: &mut Vec<String>) -> std::io::Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Err(e) = scan_dir(&path, indexed, errors) {
                    errors.push(format!("Error scanning {}: {}", path.display(), e));
                }
            } else if path.is_file() {
                // Index text-based files
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if matches!(ext, "txt" | "md" | "rs" | "js" | "py" | "json" | "xml" | "html" | "css") {
                        // TODO: Actually index with Tantivy via crate::search::SearchIndex
                        *indexed += 1;
                    }
                }
            }
        }
        Ok(())
    }

    if let Err(e) = scan_dir(Path::new(target_dir), &mut indexed_count, &mut errors) {
        return Err(format!("Failed to scan directory: {}", e));
    }

    let duration_ms = start.elapsed().as_millis() as u64;

    Ok(Some(serde_json::json!({
        "indexed_files": indexed_count,
        "duration_ms": duration_ms,
        "errors": errors,
        "target_dir": target_dir,
        "force_reindex": force_reindex
    }).to_string()))
}

async fn process_thumbnail_generation(_pool: &SqlitePool, job: &BackgroundJob) -> Result<Option<String>, String> {
    use std::path::Path;

    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .map_err(|e| format!("Invalid payload: {}", e))?;

    let source_file = payload["source_file"]
        .as_str()
        .ok_or("Missing source_file parameter")?;
    let thumbnail_dir = payload["thumbnail_dir"]
        .as_str()
        .unwrap_or("./data/thumbnails");
    let max_width = payload["max_width"]
        .as_u64()
        .unwrap_or(300) as u32;
    let max_height = payload["max_height"]
        .as_u64()
        .unwrap_or(300) as u32;

    tracing::info!("Generating thumbnail: {} -> {}", source_file, thumbnail_dir);

    let source_path = Path::new(source_file);
    if !source_path.exists() {
        return Err(format!("Source file not found: {}", source_file));
    }

    // Check if it's an image file
    let ext = source_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    if !matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp") {
        return Err(format!("Unsupported image format: {}", ext));
    }

    // Create thumbnail directory
    std::fs::create_dir_all(thumbnail_dir)
        .map_err(|e| format!("Failed to create thumbnail dir: {}", e))?;

    // Generate thumbnail filename
    let filename = source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid filename")?;
    let thumbnail_path = Path::new(thumbnail_dir).join(format!("{}_thumb.jpg", filename));

    // TODO: Use image crate to actually generate thumbnail:
    // let img = image::open(source_path)?;
    // let thumbnail = img.thumbnail(max_width, max_height);
    // thumbnail.save(&thumbnail_path)?;

    // For now, just copy the file (placeholder)
    std::fs::copy(source_path, &thumbnail_path)
        .map_err(|e| format!("Failed to create thumbnail: {}", e))?;

    let thumbnail_size = std::fs::metadata(&thumbnail_path)
        .map(|m| m.len())
        .unwrap_or(0);

    Ok(Some(serde_json::json!({
        "source_file": source_file,
        "thumbnail_path": thumbnail_path.to_string_lossy(),
        "thumbnail_size_bytes": thumbnail_size,
        "dimensions": format!("{}x{}", max_width, max_height)
    }).to_string()))
}

async fn process_file_cleanup(_pool: &SqlitePool, job: &BackgroundJob) -> Result<Option<String>, String> {
    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .map_err(|e| format!("Invalid payload: {}", e))?;

    tracing::info!("File cleanup job: {:?}", payload);

    // Parse parameters
    let older_than_days = payload.get("older_than_days")
        .and_then(|v| v.as_i64())
        .unwrap_or(30) as u64;

    let target_dir = payload.get("target_dir")
        .and_then(|v| v.as_str())
        .unwrap_or("./data/temp");

    let dry_run = payload.get("dry_run")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    tracing::info!("Cleaning up files older than {} days in {}", older_than_days, target_dir);

    let cutoff = chrono::Utc::now() - chrono::Duration::days(older_than_days as i64);
    let mut files_deleted = 0;
    let mut space_freed_bytes: u64 = 0;
    let mut errors = Vec::new();

    // Check if directory exists
    let path = std::path::Path::new(target_dir);
    if !path.exists() {
        tracing::warn!("Target directory {} does not exist, skipping cleanup", target_dir);
        return Ok(Some(serde_json::json!({
            "files_deleted": 0,
            "space_freed_mb": 0,
            "message": format!("Directory {} does not exist", target_dir)
        }).to_string()));
    }

    // Recursively scan directory
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Ok(modified) = metadata.modified() {
                        let modified_time = chrono::DateTime::<chrono::Utc>::from(modified);

                        if modified_time < cutoff {
                            let file_size = metadata.len();
                            let file_path = entry.path();

                            if dry_run {
                                tracing::info!("Would delete: {:?} ({}KB)", file_path, file_size / 1024);
                                files_deleted += 1;
                                space_freed_bytes += file_size;
                            } else {
                                match std::fs::remove_file(&file_path) {
                                    Ok(_) => {
                                        tracing::info!("Deleted: {:?} ({}KB)", file_path, file_size / 1024);
                                        files_deleted += 1;
                                        space_freed_bytes += file_size;
                                    }
                                    Err(e) => {
                                        let error_msg = format!("Failed to delete {:?}: {}", file_path, e);
                                        tracing::warn!("{}", error_msg);
                                        errors.push(error_msg);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let space_freed_mb = space_freed_bytes as f64 / (1024.0 * 1024.0);

    tracing::info!(
        "Cleanup completed: {} files deleted, {:.2}MB freed",
        files_deleted,
        space_freed_mb
    );

    Ok(Some(serde_json::json!({
        "files_deleted": files_deleted,
        "space_freed_mb": space_freed_mb,
        "space_freed_bytes": space_freed_bytes,
        "dry_run": dry_run,
        "older_than_days": older_than_days,
        "target_dir": target_dir,
        "errors": errors,
    }).to_string()))
}

async fn process_backup_task(_pool: &SqlitePool, job: &BackgroundJob) -> Result<Option<String>, String> {
    use std::path::Path;
    use std::io::Write;

    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .map_err(|e| format!("Invalid payload: {}", e))?;

    let source_dir = payload["source_dir"]
        .as_str()
        .unwrap_or("./data");
    let backup_dir = payload["backup_dir"]
        .as_str()
        .unwrap_or("./backups");
    let include_versions = payload["include_versions"]
        .as_bool()
        .unwrap_or(false);

    tracing::info!("Backup task: {} -> {} (versions: {})", source_dir, backup_dir, include_versions);

    // Create backup directory
    std::fs::create_dir_all(backup_dir)
        .map_err(|e| format!("Failed to create backup dir: {}", e))?;

    // Generate backup filename with timestamp
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let backup_filename = format!("backup_{}.tar.gz", timestamp);
    let backup_path = Path::new(backup_dir).join(&backup_filename);

    let start = std::time::Instant::now();
    let mut files_backed_up = 0u32;
    let mut total_size = 0u64;

    // Create tar.gz archive
    let tar_gz = std::fs::File::create(&backup_path)
        .map_err(|e| format!("Failed to create backup file: {}", e))?;
    let enc = flate2::write::GzEncoder::new(tar_gz, flate2::Compression::default());
    let mut tar = tar::Builder::new(enc);

    // Add files to archive
    fn add_dir_to_tar(
        tar: &mut tar::Builder<flate2::write::GzEncoder<std::fs::File>>,
        dir: &Path,
        base: &Path,
        count: &mut u32,
        size: &mut u64,
    ) -> std::io::Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let relative = path.strip_prefix(base).unwrap_or(&path);

            if path.is_dir() {
                add_dir_to_tar(tar, &path, base, count, size)?;
            } else if path.is_file() {
                let metadata = path.metadata()?;
                *size += metadata.len();
                *count += 1;
                tar.append_path_with_name(&path, relative)?;
            }
        }
        Ok(())
    }

    let source_path = Path::new(source_dir);
    if !source_path.exists() {
        return Err(format!("Source directory not found: {}", source_dir));
    }

    add_dir_to_tar(
        &mut tar,
        source_path,
        source_path,
        &mut files_backed_up,
        &mut total_size,
    )
    .map_err(|e| format!("Failed to create backup archive: {}", e))?;

    tar.finish()
        .map_err(|e| format!("Failed to finalize backup: {}", e))?;

    let backup_size = std::fs::metadata(&backup_path)
        .map(|m| m.len())
        .unwrap_or(0);
    let duration_ms = start.elapsed().as_millis() as u64;

    tracing::info!("Backup completed: {} files, {:.2}MB -> {:.2}MB (compressed)",
        files_backed_up,
        total_size as f64 / (1024.0 * 1024.0),
        backup_size as f64 / (1024.0 * 1024.0)
    );

    Ok(Some(serde_json::json!({
        "backup_filename": backup_filename,
        "backup_path": backup_path.to_string_lossy(),
        "files_backed_up": files_backed_up,
        "total_size_bytes": total_size,
        "backup_size_bytes": backup_size,
        "compression_ratio": if total_size > 0 { (backup_size as f64 / total_size as f64) * 100.0 } else { 0.0 },
        "duration_ms": duration_ms,
        "include_versions": include_versions
    }).to_string()))
}

async fn process_email_notification(_pool: &SqlitePool, job: &BackgroundJob) -> Result<Option<String>, String> {
    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .map_err(|e| format!("Invalid payload: {}", e))?;

    let to = payload["to"]
        .as_str()
        .ok_or("Missing 'to' parameter")?;
    let subject = payload["subject"]
        .as_str()
        .unwrap_or("Notification from SyncSpace");
    let body = payload["body"]
        .as_str()
        .unwrap_or("");
    let from = payload["from"]
        .as_str()
        .unwrap_or("noreply@syncspace.local");

    tracing::info!("Email notification: {} -> {} (subject: {})", from, to, subject);

    // TODO: Implement actual SMTP email sending with lettre crate:
    // use lettre::transport::smtp::authentication::Credentials;
    // use lettre::{Message, SmtpTransport, Transport};
    //
    // let email = Message::builder()
    //     .from(from.parse().unwrap())
    //     .to(to.parse().unwrap())
    //     .subject(subject)
    //     .body(body.to_string())
    //     .unwrap();
    //
    // let creds = Credentials::new("smtp_user".to_string(), "smtp_password".to_string());
    // let mailer = SmtpTransport::relay("smtp.example.com")
    //     .unwrap()
    //     .credentials(creds)
    //     .build();
    //
    // mailer.send(&email).map_err(|e| format!("SMTP error: {}", e))?;

    // Simulate email sending
    tokio::time::sleep(Duration::from_millis(800)).await;

    tracing::info!("Email sent successfully to {}", to);

    Ok(Some(serde_json::json!({
        "email_sent": true,
        "recipient": to,
        "subject": subject,
        "body_length": body.len()
    }).to_string()))
}

async fn process_webhook_delivery(_pool: &SqlitePool, job: &BackgroundJob) -> Result<Option<String>, String> {
    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .map_err(|e| format!("Invalid payload: {}", e))?;

    tracing::info!("Webhook delivery job: {:?}", payload);

    // TODO: Implement actual webhook delivery with reqwest
    tokio::time::sleep(Duration::from_millis(500)).await;

    Ok(Some(serde_json::json!({
        "delivered": true,
        "status_code": 200
    }).to_string()))
}

async fn process_file_compression(_pool: &SqlitePool, job: &BackgroundJob) -> Result<Option<String>, String> {
    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .map_err(|e| format!("Invalid payload: {}", e))?;

    tracing::info!("File compression job: {:?}", payload);

    // TODO: Implement actual file compression
    tokio::time::sleep(Duration::from_secs(2)).await;

    Ok(Some(serde_json::json!({
        "compressed_size_mb": 20,
        "original_size_mb": 50,
        "compression_ratio": 0.4
    }).to_string()))
}

async fn process_virus_scan(_pool: &SqlitePool, job: &BackgroundJob) -> Result<Option<String>, String> {
    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .map_err(|e| format!("Invalid payload: {}", e))?;

    tracing::info!("Virus scan job: {:?}", payload);

    // TODO: Implement actual virus scanning
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(Some(serde_json::json!({
        "threats_found": 0,
        "files_scanned": 1,
        "clean": true
    }).to_string()))
}
*/
 // End of commented old worker system
