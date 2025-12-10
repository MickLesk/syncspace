//! Virus Scan API endpoints
//! Provides file scanning, quarantine management, and scan statistics

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{auth::UserInfo, virus_scan, AppState};

#[derive(Debug, Deserialize)]
pub struct ScanQuery {
    /// Deep scan (slower but more thorough)
    #[serde(default)]
    pub deep: bool,
}

#[derive(Debug, Serialize)]
pub struct ScanResponse {
    pub file_path: String,
    pub status: String,
    pub threat_name: Option<String>,
    pub scan_duration_ms: u64,
    pub action_taken: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct QuarantineEntry {
    pub id: String,
    pub original_path: String,
    pub threat_name: String,
    pub quarantined_at: String,
    pub file_size: u64,
    pub can_restore: bool,
}

#[derive(Debug, Serialize)]
pub struct ScanStats {
    pub total_scanned: u64,
    pub threats_found: u64,
    pub files_quarantined: u64,
    pub last_scan_at: Option<String>,
    pub scanner_version: String,
    pub definitions_date: Option<String>,
}

/// Scan a single file
async fn scan_file(
    State(state): State<AppState>,
    Path(file_path): Path<String>,
    Query(query): Query<ScanQuery>,
    _user: UserInfo,
) -> Result<Json<ScanResponse>, StatusCode> {
    let full_path = std::path::Path::new("./data").join(&file_path);

    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let start = std::time::Instant::now();
    
    let scan_result = virus_scan::scan_file(&full_path)
        .await
        .map_err(|e| {
            tracing::error!("Scan failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let duration = start.elapsed().as_millis() as u64;

    // If threat found and auto-quarantine enabled, quarantine the file
    let action_taken = if scan_result.is_infected {
        if let Ok(_) = virus_scan::quarantine_file(&full_path).await {
            Some("quarantined".to_string())
        } else {
            Some("flagged".to_string())
        }
    } else {
        None
    };

    Ok(Json(ScanResponse {
        file_path,
        status: if scan_result.is_infected { "infected" } else { "clean" }.to_string(),
        threat_name: scan_result.threat_name,
        scan_duration_ms: duration,
        action_taken,
    }))
}

/// Scan a directory recursively
async fn scan_directory(
    State(_state): State<AppState>,
    Path(dir_path): Path<String>,
    _user: UserInfo,
) -> Result<Json<Vec<ScanResponse>>, StatusCode> {
    let full_path = std::path::Path::new("./data").join(&dir_path);

    if !full_path.exists() || !full_path.is_dir() {
        return Err(StatusCode::NOT_FOUND);
    }

    let mut results = Vec::new();
    let mut entries = tokio::fs::read_dir(&full_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    while let Ok(Some(entry)) = entries.next_entry().await {
        let entry_path = entry.path();
        if entry_path.is_file() {
            let start = std::time::Instant::now();
            
            if let Ok(scan_result) = virus_scan::scan_file(&entry_path).await {
                let duration = start.elapsed().as_millis() as u64;
                let relative_path = entry_path
                    .strip_prefix("./data")
                    .unwrap_or(&entry_path)
                    .to_string_lossy()
                    .to_string();

                results.push(ScanResponse {
                    file_path: relative_path,
                    status: if scan_result.is_infected { "infected" } else { "clean" }.to_string(),
                    threat_name: scan_result.threat_name,
                    scan_duration_ms: duration,
                    action_taken: None,
                });
            }
        }
    }

    Ok(Json(results))
}

/// Get quarantine list
async fn get_quarantine(
    State(_state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<QuarantineEntry>>, StatusCode> {
    // Admin only
    if user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let quarantine_dir = std::path::Path::new("./data/.quarantine");
    let mut entries = Vec::new();

    if quarantine_dir.exists() {
        let mut dir_entries = tokio::fs::read_dir(quarantine_dir)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        while let Ok(Some(entry)) = dir_entries.next_entry().await {
            let meta_path = entry.path().with_extension("meta.json");
            if meta_path.exists() {
                if let Ok(meta_content) = tokio::fs::read_to_string(&meta_path).await {
                    if let Ok(meta) = serde_json::from_str::<serde_json::Value>(&meta_content) {
                        entries.push(QuarantineEntry {
                            id: entry.file_name().to_string_lossy().to_string(),
                            original_path: meta["original_path"].as_str().unwrap_or("").to_string(),
                            threat_name: meta["threat_name"].as_str().unwrap_or("Unknown").to_string(),
                            quarantined_at: meta["quarantined_at"].as_str().unwrap_or("").to_string(),
                            file_size: meta["file_size"].as_u64().unwrap_or(0),
                            can_restore: true,
                        });
                    }
                }
            }
        }
    }

    Ok(Json(entries))
}

/// Restore file from quarantine
async fn restore_from_quarantine(
    State(_state): State<AppState>,
    Path(quarantine_id): Path<String>,
    user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Admin only
    if user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let quarantine_dir = std::path::Path::new("./data/.quarantine");
    let quarantine_path = quarantine_dir.join(&quarantine_id);

    if !quarantine_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    virus_scan::restore_from_quarantine(&quarantine_path)
        .await
        .map_err(|e| {
            tracing::error!("Failed to restore from quarantine: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "File restored from quarantine"
    })))
}

/// Delete file from quarantine permanently
async fn delete_from_quarantine(
    State(_state): State<AppState>,
    Path(quarantine_id): Path<String>,
    user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    // Admin only
    if user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let quarantine_dir = std::path::Path::new("./data/.quarantine");
    let quarantine_path = quarantine_dir.join(&quarantine_id);
    let meta_path = quarantine_path.with_extension("meta.json");

    if quarantine_path.exists() {
        tokio::fs::remove_file(&quarantine_path)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    if meta_path.exists() {
        let _ = tokio::fs::remove_file(&meta_path).await;
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Get scan statistics
async fn get_scan_stats(
    State(_state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<ScanStats>, StatusCode> {
    let stats = virus_scan::get_scan_stats()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ScanStats {
        total_scanned: stats.total_scanned,
        threats_found: stats.threats_found,
        files_quarantined: stats.files_quarantined,
        last_scan_at: stats.last_scan_at,
        scanner_version: stats.scanner_version,
        definitions_date: stats.definitions_date,
    }))
}

/// Check scanner health
async fn check_scanner_health(
    State(_state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let is_available = virus_scan::check_scanner_available().await;

    Ok(Json(serde_json::json!({
        "available": is_available,
        "scanner": "ClamAV",
        "message": if is_available { "Scanner is ready" } else { "Scanner not available - using fallback" }
    })))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/scan/file/{*path}", post(scan_file))
        .route("/scan/directory/{*path}", post(scan_directory))
        .route("/scan/quarantine", get(get_quarantine))
        .route("/scan/quarantine/{id}/restore", post(restore_from_quarantine))
        .route("/scan/quarantine/{id}", axum::routing::delete(delete_from_quarantine))
        .route("/scan/stats", get(get_scan_stats))
        .route("/scan/health", get(check_scanner_health))
}
