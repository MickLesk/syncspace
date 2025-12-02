//! Background Job Worker
//! Continuously processes queued jobs with priority handling

use chrono::Utc;
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, sqlx::FromRow)]
struct PendingJob {
    id: String,
    job_type: String,
    payload: String,
    attempts: i32,
    max_attempts: i32,
}

/// Start the background job worker
pub fn spawn_worker(db_pool: SqlitePool) {
    tokio::spawn(async move {
        println!("🚀 Background job worker started");
        
        loop {
            if let Err(e) = process_pending_jobs(&db_pool).await {
                eprintln!("Job worker error: {}", e);
            }
            
            sleep(Duration::from_secs(5)).await;
        }
    });
}

async fn process_pending_jobs(db_pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch pending jobs ordered by priority and scheduled time
    let jobs: Vec<PendingJob> = sqlx::query_as(
        r#"
        SELECT id, job_type, payload, attempts, max_attempts
        FROM background_jobs
        WHERE status = 'pending'
        AND scheduled_at <= ?
        ORDER BY priority ASC, scheduled_at ASC
        LIMIT 5
        "#,
    )
    .bind(Utc::now().to_rfc3339())
    .fetch_all(db_pool)
    .await?;

    for job in jobs {
        println!("Processing job {} ({})", job.id, job.job_type);
        
        // Mark as running
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            r#"
            UPDATE background_jobs
            SET status = 'running', 
                started_at = ?,
                updated_at = ?,
                attempts = attempts + 1
            WHERE id = ?
            "#,
        )
        .bind(&now)
        .bind(&now)
        .bind(&job.id)
        .execute(db_pool)
        .await?;

        // Process job based on type
        let result = match job.job_type.as_str() {
            "bulk_delete" => process_bulk_delete(db_pool, &job).await,
            "bulk_move" => process_bulk_move(db_pool, &job).await,
            "bulk_copy" => process_bulk_copy(db_pool, &job).await,
            "bulk_compress" => process_bulk_compress(db_pool, &job).await,
            _ => Err(format!("Unknown job type: {}", job.job_type)),
        };

        // Update job with result
        let now = Utc::now().to_rfc3339();
        let job_completed_or_failed = match &result {
            Ok(ref result_data) => {
                sqlx::query(
                    r#"
                    UPDATE background_jobs
                    SET status = 'completed',
                        result = ?,
                        completed_at = ?,
                        updated_at = ?
                    WHERE id = ?
                    "#,
                )
                .bind(result_data)
                .bind(&now)
                .bind(&now)
                .bind(&job.id)
                .execute(db_pool)
                .await?;

                println!("✅ Job {} completed", job.id);
                true
            }
            Err(ref error) => {
                let should_retry = job.attempts < job.max_attempts;
                let status = if should_retry { "pending" } else { "failed" };

                sqlx::query(
                    r#"
                    UPDATE background_jobs
                    SET status = ?,
                        error = ?,
                        completed_at = ?,
                        updated_at = ?
                    WHERE id = ?
                    "#,
                )
                .bind(status)
                .bind(error)
                .bind(if should_retry { None } else { Some(&now) })
                .bind(&now)
                .bind(&job.id)
                .execute(db_pool)
                .await?;

                if should_retry {
                    println!("⚠️ Job {} failed, will retry (attempt {}/{})", 
                        job.id, job.attempts, job.max_attempts);
                    false
                } else {
                    eprintln!("❌ Job {} failed permanently: {}", job.id, error);
                    true
                }
            }
        };

        // Move completed/failed jobs to history
        if job_completed_or_failed {
            archive_job(db_pool, &job.id).await?;
        }
    }

    Ok(())
}

async fn process_bulk_delete(
    db_pool: &SqlitePool,
    job: &PendingJob,
) -> Result<String, String> {
    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .map_err(|e| format!("Invalid payload: {}", e))?;

    let file_paths = payload
        .get("file_paths")
        .and_then(|v| v.as_array())
        .ok_or("Missing file_paths in payload")?;

    let mut deleted = 0;
    let mut failed = 0;

    for (idx, path) in file_paths.iter().enumerate() {
        let file_path = path.as_str().ok_or("Invalid file path")?;
        
        // Delete from database
        match sqlx::query("DELETE FROM files WHERE file_path = ?")
            .bind(file_path)
            .execute(db_pool)
            .await
        {
            Ok(_) => {
                deleted += 1;
                // Try to delete actual file (best effort)
                let _ = std::fs::remove_file(format!("./data/{}", file_path));
            }
            Err(_) => failed += 1,
        }

        // Update progress
        let progress = ((idx + 1) * 100 / file_paths.len()) as i32;
        let _ = sqlx::query(
            "UPDATE background_jobs SET result = ? WHERE id = ?"
        )
        .bind(serde_json::json!({"progress": progress}).to_string())
        .bind(&job.id)
        .execute(db_pool)
        .await;
    }

    Ok(serde_json::json!({
        "deleted": deleted,
        "failed": failed,
        "total": file_paths.len(),
    }).to_string())
}

async fn process_bulk_move(
    db_pool: &SqlitePool,
    job: &PendingJob,
) -> Result<String, String> {
    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .map_err(|e| format!("Invalid payload: {}", e))?;

    let file_paths = payload.get("file_paths")
        .and_then(|v| v.as_array())
        .ok_or("Missing file_paths")?;
    
    let destination = payload.get("destination")
        .and_then(|v| v.as_str())
        .ok_or("Missing destination")?;

    let mut moved = 0;
    let mut failed = 0;

    for (idx, path) in file_paths.iter().enumerate() {
        let file_path = path.as_str().ok_or("Invalid file path")?;
        let filename = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        let new_path = format!("{}/{}", destination.trim_end_matches('/'), filename);

        // Update in database
        match sqlx::query(
            "UPDATE files SET file_path = ?, updated_at = ? WHERE file_path = ?"
        )
        .bind(&new_path)
        .bind(Utc::now().to_rfc3339())
        .bind(file_path)
        .execute(db_pool)
        .await
        {
            Ok(_) => {
                moved += 1;
                // Try to move actual file
                let old_full = format!("./data/{}", file_path);
                let new_full = format!("./data/{}", new_path);
                let _ = std::fs::rename(old_full, new_full);
            }
            Err(_) => failed += 1,
        }

        let progress = ((idx + 1) * 100 / file_paths.len()) as i32;
        let _ = sqlx::query(
            "UPDATE background_jobs SET result = ? WHERE id = ?"
        )
        .bind(serde_json::json!({"progress": progress}).to_string())
        .bind(&job.id)
        .execute(db_pool)
        .await;
    }

    Ok(serde_json::json!({
        "moved": moved,
        "failed": failed,
        "total": file_paths.len(),
    }).to_string())
}

async fn process_bulk_copy(
    db_pool: &SqlitePool,
    job: &PendingJob,
) -> Result<String, String> {
    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .map_err(|e| format!("Invalid payload: {}", e))?;

    let file_paths = payload.get("file_paths")
        .and_then(|v| v.as_array())
        .ok_or("Missing file_paths")?;
    
    let destination = payload.get("destination")
        .and_then(|v| v.as_str())
        .ok_or("Missing destination")?;

    let mut copied = 0;
    let mut failed = 0;

    for (idx, path) in file_paths.iter().enumerate() {
        let file_path = path.as_str().ok_or("Invalid file path")?;
        let filename = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        let new_path = format!("{}/{}", destination.trim_end_matches('/'), filename);

        // Copy file record in database
        if let Ok(original) = sqlx::query_as::<_, (String, i64, String, String)>(
            "SELECT filename, size_bytes, mime_type, owner_id FROM files WHERE file_path = ?"
        )
        .bind(file_path)
        .fetch_one(db_pool)
        .await
        {
            let now = Utc::now().to_rfc3339();
            match sqlx::query(
                r#"
                INSERT INTO files (id, filename, file_path, size_bytes, mime_type, owner_id, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                "#
            )
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(&original.0)
            .bind(&new_path)
            .bind(original.1)
            .bind(&original.2)
            .bind(&original.3)
            .bind(&now)
            .bind(&now)
            .execute(db_pool)
            .await
            {
                Ok(_) => {
                    copied += 1;
                    // Try to copy actual file
                    let old_full = format!("./data/{}", file_path);
                    let new_full = format!("./data/{}", new_path);
                    let _ = std::fs::copy(old_full, new_full);
                }
                Err(_) => failed += 1,
            }
        } else {
            failed += 1;
        }

        let progress = ((idx + 1) * 100 / file_paths.len()) as i32;
        let _ = sqlx::query(
            "UPDATE background_jobs SET result = ? WHERE id = ?"
        )
        .bind(serde_json::json!({"progress": progress}).to_string())
        .bind(&job.id)
        .execute(db_pool)
        .await;
    }

    Ok(serde_json::json!({
        "copied": copied,
        "failed": failed,
        "total": file_paths.len(),
    }).to_string())
}

async fn process_bulk_compress(
    db_pool: &SqlitePool,
    job: &PendingJob,
) -> Result<String, String> {
    use std::io::{Read, Write};
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .map_err(|e| format!("Invalid payload: {}", e))?;

    let file_paths: Vec<String> = payload.get("file_paths")
        .and_then(|v| v.as_array())
        .ok_or("Missing file_paths")?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    let output_path = payload.get("output_path")
        .and_then(|v| v.as_str())
        .unwrap_or("bulk_archive.zip");

    // Create the archive in the data directory
    let archive_path = format!("./data/{}", output_path);
    
    // Ensure parent directory exists
    if let Some(parent) = std::path::Path::new(&archive_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let file = std::fs::File::create(&archive_path)
        .map_err(|e| format!("Failed to create archive: {}", e))?;
    
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .compression_level(Some(6));

    let mut compressed = 0;
    let mut failed = 0;
    let total = file_paths.len();

    for (idx, file_path) in file_paths.iter().enumerate() {
        let full_path = format!("./data/{}", file_path);
        
        match std::fs::File::open(&full_path) {
            Ok(mut source_file) => {
                // Use only the filename in the archive (not full path)
                let file_name = std::path::Path::new(file_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(file_path);

                if let Err(e) = zip.start_file(file_name, options) {
                    eprintln!("Failed to add {} to archive: {}", file_path, e);
                    failed += 1;
                    continue;
                }

                let mut buffer = Vec::new();
                if let Err(e) = source_file.read_to_end(&mut buffer) {
                    eprintln!("Failed to read {}: {}", file_path, e);
                    failed += 1;
                    continue;
                }

                if let Err(e) = zip.write_all(&buffer) {
                    eprintln!("Failed to write {} to archive: {}", file_path, e);
                    failed += 1;
                    continue;
                }

                compressed += 1;
            }
            Err(e) => {
                eprintln!("Failed to open {}: {}", file_path, e);
                failed += 1;
            }
        }

        // Update progress
        let progress = ((idx + 1) * 100 / total) as i32;
        let _ = sqlx::query("UPDATE background_jobs SET result = ? WHERE id = ?")
            .bind(serde_json::json!({"progress": progress}).to_string())
            .bind(&job.id)
            .execute(db_pool)
            .await;
    }

    let _ = zip.finish().map_err(|e| format!("Failed to finish archive: {}", e))?;

    // Get final archive size
    let archive_size = std::fs::metadata(&archive_path)
        .map(|m| m.len())
        .unwrap_or(0);

    Ok(serde_json::json!({
        "compressed": compressed,
        "failed": failed,
        "total": total,
        "archive_path": output_path,
        "archive_size": archive_size,
    }).to_string())
}

async fn archive_job(db_pool: &SqlitePool, job_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch job details with proper types
    let job = sqlx::query_as::<_, (String, String, String, String, Option<String>, Option<String>)>(
        "SELECT id, job_type, status, created_at, started_at, completed_at FROM background_jobs WHERE id = ?"
    )
    .bind(job_id)
    .fetch_one(db_pool)
    .await?;

    // Calculate duration
    let duration_ms = if let (Some(started), Some(completed)) = (&job.4, &job.5) {
        chrono::DateTime::parse_from_rfc3339(completed)
            .ok()
            .and_then(|c| chrono::DateTime::parse_from_rfc3339(started)
                .ok()
                .map(|s| (c.timestamp_millis() - s.timestamp_millis())))
            .unwrap_or(0)
    } else {
        0
    };

    // Insert into history
    sqlx::query(
        r#"
        INSERT INTO job_history (id, job_id, job_type, status, duration_ms, completed_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(&job.0)
    .bind(&job.1)
    .bind(&job.2)
    .bind(duration_ms)
    .bind(job.5.as_ref().unwrap_or(&Utc::now().to_rfc3339()))
    .execute(db_pool)
    .await?;

    Ok(())
}
