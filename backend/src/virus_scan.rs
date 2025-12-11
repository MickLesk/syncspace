/// Virus scanning integration with ClamAV
/// 
/// Provides malware detection:
/// - On-upload scanning
/// - Manual file scanning
/// - Quarantine management
/// - Scan result history

use std::path::{Path, PathBuf};
use std::process::Command;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use tokio::fs;

const QUARANTINE_DIR: &str = "./data/.quarantine";

/// Scan result stored in database
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ScanResult {
    pub id: String,
    pub file_id: String,
    pub file_path: String,
    pub scan_status: String, // "clean", "infected", "error", "pending", "skipped"
    pub virus_name: Option<String>,
    pub scanner_version: Option<String>,
    pub scanned_at: String,
    pub scan_duration_ms: i64,
    pub quarantined: bool,
    pub quarantine_path: Option<String>,
}

/// Scan status enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ScanStatus {
    Clean,
    Infected,
    Error,
    Pending,
    Skipped,
}

impl ScanStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScanStatus::Clean => "clean",
            ScanStatus::Infected => "infected",
            ScanStatus::Error => "error",
            ScanStatus::Pending => "pending",
            ScanStatus::Skipped => "skipped",
        }
    }
}

/// Scanner configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerConfig {
    pub enabled: bool,
    pub scan_on_upload: bool,
    pub max_file_size_mb: u64,
    pub quarantine_infected: bool,
    pub excluded_extensions: Vec<String>,
    pub excluded_paths: Vec<String>,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            enabled: false, // Disabled by default (requires ClamAV)
            scan_on_upload: true,
            max_file_size_mb: 100,
            quarantine_infected: true,
            excluded_extensions: vec![],
            excluded_paths: vec![".thumbnails".to_string(), ".previews".to_string()],
        }
    }
}

// ==================== SCANNING ====================

/// Check if ClamAV is available
pub fn is_clamav_available() -> bool {
    Command::new("clamscan")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Get ClamAV version
pub fn get_scanner_version() -> Option<String> {
    Command::new("clamscan")
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok()
                    .map(|s| s.lines().next().unwrap_or("").trim().to_string())
            } else {
                None
            }
        })
}

/// Scan a single file with ClamAV
pub async fn scan_file(file_path: &Path) -> Result<(ScanStatus, Option<String>), ScanError> {
    if !is_clamav_available() {
        return Err(ScanError::ScannerNotAvailable);
    }
    
    if !file_path.exists() {
        return Err(ScanError::FileNotFound);
    }
    
    let output = Command::new("clamscan")
        .args(&[
            "--no-summary",
            "--infected",
            &file_path.to_string_lossy(),
        ])
        .output()
        .map_err(|e| ScanError::ScanFailed(e.to_string()))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // ClamAV exit codes:
    // 0 = No virus found
    // 1 = Virus found
    // 2 = Error occurred
    
    match output.status.code() {
        Some(0) => Ok((ScanStatus::Clean, None)),
        Some(1) => {
            // Extract virus name from output
            let virus_name = stdout.lines()
                .find(|l| l.contains("FOUND"))
                .and_then(|l| {
                    // Format: "filename: VirusName FOUND"
                    let parts: Vec<&str> = l.split(':').collect();
                    if parts.len() >= 2 {
                        Some(parts[1].trim().trim_end_matches(" FOUND").to_string())
                    } else {
                        None
                    }
                });
            
            Ok((ScanStatus::Infected, virus_name))
        }
        Some(2) => {
            let error_msg = if stderr.is_empty() { stdout.to_string() } else { stderr.to_string() };
            Err(ScanError::ScanFailed(error_msg))
        }
        _ => {
            let error_msg = if stderr.is_empty() { stdout.to_string() } else { stderr.to_string() };
            Err(ScanError::ScanFailed(error_msg))
        }
    }
}

/// Scan file and store result
pub async fn scan_and_store(
    pool: &SqlitePool,
    file_id: &str,
    file_path: &Path,
    config: &ScannerConfig,
) -> Result<ScanResult, ScanError> {
    let start = std::time::Instant::now();
    
    // Check if file should be skipped
    if should_skip_file(file_path, config) {
        return store_scan_result(
            pool,
            file_id,
            file_path,
            ScanStatus::Skipped,
            None,
            0,
        ).await;
    }
    
    // Check file size
    let metadata = fs::metadata(file_path).await
        .map_err(|e| ScanError::IoError(e.to_string()))?;
    
    if metadata.len() > config.max_file_size_mb * 1024 * 1024 {
        return store_scan_result(
            pool,
            file_id,
            file_path,
            ScanStatus::Skipped,
            Some("File too large".to_string()),
            0,
        ).await;
    }
    
    // Perform scan
    let (status, virus_name) = scan_file(file_path).await?;
    let duration_ms = start.elapsed().as_millis() as i64;
    
    // Store result
    let mut result = store_scan_result(
        pool,
        file_id,
        file_path,
        status,
        virus_name.clone(),
        duration_ms,
    ).await?;
    
    // Quarantine if infected
    if status == ScanStatus::Infected && config.quarantine_infected
        && let Ok(quarantine_path) = quarantine_file(pool, file_id, file_path).await {
            result.quarantined = true;
            result.quarantine_path = Some(quarantine_path.to_string_lossy().to_string());
        }
    
    Ok(result)
}

/// Check if file should be skipped based on config
fn should_skip_file(file_path: &Path, config: &ScannerConfig) -> bool {
    // Check extension
    if let Some(ext) = file_path.extension().and_then(|e| e.to_str())
        && config.excluded_extensions.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
            return true;
        }
    
    // Check path
    let path_str = file_path.to_string_lossy();
    for excluded in &config.excluded_paths {
        if path_str.contains(excluded) {
            return true;
        }
    }
    
    false
}

// ==================== QUARANTINE ====================

/// Initialize quarantine directory
pub async fn init_quarantine_dir() -> Result<(), std::io::Error> {
    fs::create_dir_all(QUARANTINE_DIR).await
}

/// Move infected file to quarantine
pub async fn quarantine_file(
    pool: &SqlitePool,
    file_id: &str,
    original_path: &Path,
) -> Result<PathBuf, ScanError> {
    init_quarantine_dir().await
        .map_err(|e| ScanError::IoError(e.to_string()))?;
    
    let quarantine_name = format!("{}_{}", file_id, chrono::Utc::now().timestamp());
    let quarantine_path = PathBuf::from(QUARANTINE_DIR).join(&quarantine_name);
    
    // Move file to quarantine
    fs::rename(original_path, &quarantine_path).await
        .map_err(|e| ScanError::IoError(format!("Failed to quarantine: {}", e)))?;
    
    // Update database
    sqlx::query(
        "UPDATE scan_results 
         SET quarantined = 1, quarantine_path = ? 
         WHERE file_id = ? AND quarantined = 0"
    )
    .bind(quarantine_path.to_string_lossy().to_string())
    .bind(file_id)
    .execute(pool)
    .await
    .map_err(|e| ScanError::DatabaseError(e.to_string()))?;
    
    // Mark file as deleted in files table
    sqlx::query("UPDATE files SET is_deleted = 1 WHERE id = ?")
        .bind(file_id)
        .execute(pool)
        .await
        .ok();
    
    tracing::warn!("File {} quarantined to {}", file_id, quarantine_path.display());
    
    Ok(quarantine_path)
}

/// Restore file from quarantine
pub async fn restore_from_quarantine(
    pool: &SqlitePool,
    file_id: &str,
) -> Result<PathBuf, ScanError> {
    // Get quarantine info
    let result: Option<ScanResult> = sqlx::query_as(
        "SELECT * FROM scan_results WHERE file_id = ? AND quarantined = 1"
    )
    .bind(file_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ScanError::DatabaseError(e.to_string()))?;
    
    let result = result.ok_or(ScanError::NotQuarantined)?;
    let quarantine_path = result.quarantine_path.ok_or(ScanError::NotQuarantined)?;
    
    // Restore file
    let original_path = PathBuf::from(&result.file_path);
    
    fs::rename(&quarantine_path, &original_path).await
        .map_err(|e| ScanError::IoError(format!("Failed to restore: {}", e)))?;
    
    // Update database
    sqlx::query(
        "UPDATE scan_results 
         SET quarantined = 0, quarantine_path = NULL 
         WHERE file_id = ?"
    )
    .bind(file_id)
    .execute(pool)
    .await
    .map_err(|e| ScanError::DatabaseError(e.to_string()))?;
    
    // Restore file in files table
    sqlx::query("UPDATE files SET is_deleted = 0 WHERE id = ?")
        .bind(file_id)
        .execute(pool)
        .await
        .ok();
    
    tracing::info!("File {} restored from quarantine", file_id);
    
    Ok(original_path)
}

/// Delete quarantined file permanently
pub async fn delete_quarantined(
    pool: &SqlitePool,
    file_id: &str,
) -> Result<(), ScanError> {
    let result: Option<ScanResult> = sqlx::query_as(
        "SELECT * FROM scan_results WHERE file_id = ? AND quarantined = 1"
    )
    .bind(file_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ScanError::DatabaseError(e.to_string()))?;
    
    let result = result.ok_or(ScanError::NotQuarantined)?;
    
    if let Some(path) = result.quarantine_path {
        fs::remove_file(&path).await
            .map_err(|e| ScanError::IoError(e.to_string()))?;
    }
    
    // Remove from database
    sqlx::query("DELETE FROM scan_results WHERE file_id = ? AND quarantined = 1")
        .bind(file_id)
        .execute(pool)
        .await
        .map_err(|e| ScanError::DatabaseError(e.to_string()))?;
    
    // Delete file record
    sqlx::query("DELETE FROM files WHERE id = ?")
        .bind(file_id)
        .execute(pool)
        .await
        .ok();
    
    Ok(())
}

/// List quarantined files
pub async fn list_quarantined(pool: &SqlitePool) -> Result<Vec<ScanResult>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM scan_results WHERE quarantined = 1 ORDER BY scanned_at DESC"
    )
    .fetch_all(pool)
    .await
}

// ==================== DATABASE ====================

/// Store scan result in database
async fn store_scan_result(
    pool: &SqlitePool,
    file_id: &str,
    file_path: &Path,
    status: ScanStatus,
    virus_name: Option<String>,
    duration_ms: i64,
) -> Result<ScanResult, ScanError> {
    let id = Uuid::new_v4().to_string();
    let scanner_version = get_scanner_version();
    
    sqlx::query(
        "INSERT INTO scan_results 
         (id, file_id, file_path, scan_status, virus_name, scanner_version, 
          scanned_at, scan_duration_ms, quarantined, quarantine_path)
         VALUES (?, ?, ?, ?, ?, ?, datetime('now'), ?, 0, NULL)"
    )
    .bind(&id)
    .bind(file_id)
    .bind(file_path.to_string_lossy().to_string())
    .bind(status.as_str())
    .bind(&virus_name)
    .bind(&scanner_version)
    .bind(duration_ms)
    .execute(pool)
    .await
    .map_err(|e| ScanError::DatabaseError(e.to_string()))?;
    
    sqlx::query_as("SELECT * FROM scan_results WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(|e| ScanError::DatabaseError(e.to_string()))
}

/// Get scan history for a file
pub async fn get_scan_history(
    pool: &SqlitePool,
    file_id: &str,
) -> Result<Vec<ScanResult>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM scan_results WHERE file_id = ? ORDER BY scanned_at DESC"
    )
    .bind(file_id)
    .fetch_all(pool)
    .await
}

/// Get latest scan result
pub async fn get_latest_scan(
    pool: &SqlitePool,
    file_id: &str,
) -> Result<Option<ScanResult>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM scan_results WHERE file_id = ? ORDER BY scanned_at DESC LIMIT 1"
    )
    .bind(file_id)
    .fetch_optional(pool)
    .await
}

/// Get scan statistics
pub async fn get_scan_stats(pool: &SqlitePool) -> Result<ScanStats, sqlx::Error> {
    let row: (i64, i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT 
            COUNT(*) as total,
            SUM(CASE WHEN scan_status = 'clean' THEN 1 ELSE 0 END) as clean,
            SUM(CASE WHEN scan_status = 'infected' THEN 1 ELSE 0 END) as infected,
            SUM(CASE WHEN scan_status = 'error' THEN 1 ELSE 0 END) as errors,
            SUM(CASE WHEN quarantined = 1 THEN 1 ELSE 0 END) as quarantined
        FROM scan_results
        "#
    )
    .fetch_one(pool)
    .await?;
    
    Ok(ScanStats {
        total_scans: row.0 as u64,
        clean: row.1 as u64,
        infected: row.2 as u64,
        errors: row.3 as u64,
        quarantined: row.4 as u64,
        scanner_available: is_clamav_available(),
        scanner_version: get_scanner_version(),
    })
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanStats {
    pub total_scans: u64,
    pub clean: u64,
    pub infected: u64,
    pub errors: u64,
    pub quarantined: u64,
    pub scanner_available: bool,
    pub scanner_version: Option<String>,
}

// ==================== ERRORS ====================

#[derive(Debug, Clone)]
pub enum ScanError {
    ScannerNotAvailable,
    ScanFailed(String),
    FileNotFound,
    IoError(String),
    DatabaseError(String),
    NotQuarantined,
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::ScannerNotAvailable => write!(f, "ClamAV scanner not available"),
            ScanError::ScanFailed(e) => write!(f, "Scan failed: {}", e),
            ScanError::FileNotFound => write!(f, "File not found"),
            ScanError::IoError(e) => write!(f, "IO error: {}", e),
            ScanError::DatabaseError(e) => write!(f, "Database error: {}", e),
            ScanError::NotQuarantined => write!(f, "File is not quarantined"),
        }
    }
}

impl std::error::Error for ScanError {}

// ==================== API COMPATIBILITY FUNCTIONS ====================

/// Simpler scan result for API responses
#[derive(Debug, Clone, Serialize)]
pub struct SimpleScanResult {
    pub is_infected: bool,
    pub threat_name: Option<String>,
}

/// Scan a file and return simple result (for API)
/// This is an alias that wraps scan_file for simpler API usage
pub async fn scan_file_simple(file_path: &Path) -> Result<SimpleScanResult, ScanError> {
    let (status, threat) = scan_file(file_path).await?;
    Ok(SimpleScanResult {
        is_infected: status == ScanStatus::Infected,
        threat_name: threat,
    })
}

/// Quarantine an infected file (simple version for API)
pub async fn quarantine_file_simple(file_path: &Path) -> Result<PathBuf, ScanError> {
    if !file_path.exists() {
        return Err(ScanError::FileNotFound);
    }

    // Create quarantine directory
    fs::create_dir_all(QUARANTINE_DIR).await
        .map_err(|e| ScanError::IoError(e.to_string()))?;

    let quarantine_id = Uuid::new_v4().to_string();
    let quarantine_path = PathBuf::from(QUARANTINE_DIR).join(&quarantine_id);
    let meta_path = quarantine_path.with_extension("meta.json");

    // Move file to quarantine
    fs::rename(file_path, &quarantine_path).await
        .map_err(|e| ScanError::IoError(e.to_string()))?;

    // Save metadata
    let metadata = serde_json::json!({
        "original_path": file_path.to_string_lossy(),
        "quarantined_at": chrono::Utc::now().to_rfc3339(),
        "threat_name": "Unknown",
        "file_size": fs::metadata(&quarantine_path).await.map(|m| m.len()).unwrap_or(0)
    });

    fs::write(&meta_path, serde_json::to_string_pretty(&metadata).unwrap()).await
        .map_err(|e| ScanError::IoError(e.to_string()))?;

    Ok(quarantine_path)
}

/// Restore a file from quarantine (simple version)
pub async fn restore_quarantine_simple(quarantine_path: &Path) -> Result<PathBuf, ScanError> {
    if !quarantine_path.exists() {
        return Err(ScanError::NotQuarantined);
    }

    let meta_path = quarantine_path.with_extension("meta.json");
    let meta_content = fs::read_to_string(&meta_path).await
        .map_err(|e| ScanError::IoError(e.to_string()))?;

    let metadata: serde_json::Value = serde_json::from_str(&meta_content)
        .map_err(|e| ScanError::IoError(e.to_string()))?;

    let original_path = metadata["original_path"]
        .as_str()
        .ok_or(ScanError::IoError("Missing original path in metadata".to_string()))?;

    let original = PathBuf::from(original_path);

    // Create parent directories if needed
    if let Some(parent) = original.parent() {
        fs::create_dir_all(parent).await.ok();
    }

    // Move back to original location
    fs::rename(quarantine_path, &original).await
        .map_err(|e| ScanError::IoError(e.to_string()))?;

    // Remove metadata file
    fs::remove_file(&meta_path).await.ok();

    Ok(original)
}

/// Get scan statistics for API
#[derive(Debug, Clone, Serialize)]
pub struct ApiScanStats {
    pub total_scanned: u64,
    pub threats_found: u64,
    pub files_quarantined: u64,
    pub last_scan_at: Option<String>,
    pub scanner_version: String,
    pub definitions_date: Option<String>,
}

/// Get scan statistics (for API - simple version without database)
pub async fn get_scan_stats_simple() -> Result<ApiScanStats, ScanError> {
    Ok(ApiScanStats {
        total_scanned: 0,
        threats_found: 0,
        files_quarantined: count_quarantined_files().await.unwrap_or(0),
        last_scan_at: None,
        scanner_version: get_scanner_version().unwrap_or_else(|| "ClamAV (not installed)".to_string()),
        definitions_date: None,
    })
}

/// Count quarantined files
async fn count_quarantined_files() -> Result<u64, ScanError> {
    let quarantine_dir = PathBuf::from(QUARANTINE_DIR);
    if !quarantine_dir.exists() {
        return Ok(0);
    }

    let mut count = 0u64;
    let mut entries = fs::read_dir(&quarantine_dir).await
        .map_err(|e| ScanError::IoError(e.to_string()))?;

    while let Ok(Some(entry)) = entries.next_entry().await {
        if entry.path().extension().and_then(|e| e.to_str()) != Some("json") {
            count += 1;
        }
    }

    Ok(count)
}

/// Check if scanner is available (for API)
pub async fn check_scanner_available() -> bool {
    is_clamav_available()
}
