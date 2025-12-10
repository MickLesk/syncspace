/// Virus scanning integration with ClamAV
use std::process::Command;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ScanResult {
    pub id: String,
    pub file_id: String,
    pub scan_status: String, // "clean", "infected", "error"
    pub virus_name: Option<String>,
    pub scanned_at: String,
    pub quarantined: bool,
}

/// Scan file with ClamAV
pub async fn scan_file(file_path: &str) -> Result<(String, Option<String>), Box<dyn std::error::Error + Send + Sync>> {
    let output = Command::new("clamscan")
        .args(&["--no-summary", file_path])
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    if stdout.contains("OK") {
        Ok(("clean".to_string(), None))
    } else if stdout.contains("FOUND") {
        let virus = stdout.lines()
            .find(|l| l.contains("FOUND"))
            .and_then(|l| l.split(':').last())
            .map(|s| s.trim().to_string());
        Ok(("infected".to_string(), virus))
    } else {
        Ok(("error".to_string(), None))
    }
}

/// Store scan result
pub async fn store_scan_result(
    pool: &SqlitePool,
    file_id: &str,
    status: &str,
    virus_name: Option<String>,
) -> Result<ScanResult, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO scan_results (id, file_id, scan_status, virus_name, scanned_at, quarantined)
         VALUES (?, ?, ?, ?, datetime('now'), ?)"
    )
    .bind(&id)
    .bind(file_id)
    .bind(status)
    .bind(virus_name)
    .bind(if status == "infected" { 1 } else { 0 })
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM scan_results WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Quarantine infected file
pub async fn quarantine_file(
    pool: &SqlitePool,
    file_id: &str,
    original_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let quarantine_dir = "./data/quarantine";
    tokio::fs::create_dir_all(quarantine_dir).await?;
    
    let quarantine_path = format!("{}/{}", quarantine_dir, file_id);
    tokio::fs::rename(original_path, &quarantine_path).await?;
    
    sqlx::query("UPDATE scan_results SET quarantined = 1 WHERE file_id = ?")
        .bind(file_id)
        .execute(pool)
        .await?;
    
    Ok(())
}
