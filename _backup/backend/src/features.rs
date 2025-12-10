/**
 * BACKEND FEATURE EXTENSIONS
 * Additional endpoints for advanced file management
 * 
 * Features to implement:
 * 1. File Compression (ZIP)
 * 2. Share Links with expiry
 * 3. Trash/Recycle Bin
 * 4. Thumbnail Generation
 * 5. Full-Text Search
 * 6. File Versioning
 * 7. Activity Log
 * 8. Duplicate Detection
 */

//! Additional backend features module
//! This file will be integrated into main.rs

use std::collections::HashMap;
use std::path::Path;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =============================================================================
// SHARE LINKS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareLink {
    pub id: String,
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub password: Option<String>,
    pub download_count: u32,
    pub max_downloads: Option<u32>,
}

impl ShareLink {
    pub fn new(path: String, duration_hours: i64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            path,
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(duration_hours),
            password: None,
            download_count: 0,
            max_downloads: None,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn is_exhausted(&self) -> bool {
        if let Some(max) = self.max_downloads {
            self.download_count >= max
        } else {
            false
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.is_expired() && !self.is_exhausted()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateShareLinkRequest {
    pub path: String,
    #[serde(default = "default_duration")]
    pub duration_hours: i64,
    pub password: Option<String>,
    pub max_downloads: Option<u32>,
}

fn default_duration() -> i64 {
    24 // 24 hours default
}

#[derive(Debug, Clone, Serialize)]
pub struct ShareLinkResponse {
    pub link_id: String,
    pub url: String,
    pub expires_at: DateTime<Utc>,
}

// =============================================================================
// TRASH/RECYCLE BIN
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrashItem {
    pub id: String,
    pub original_path: String,
    pub trash_path: String,
    pub deleted_at: DateTime<Utc>,
    pub size: u64,
    pub is_dir: bool,
}

impl TrashItem {
    pub fn new(original_path: String, trash_path: String, size: u64, is_dir: bool) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            original_path,
            trash_path,
            deleted_at: Utc::now(),
            size,
            is_dir,
        }
    }
}

// =============================================================================
// ACTIVITY LOG
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLogEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub user: String,
    pub action: String, // "upload", "download", "delete", "rename", "share"
    pub path: String,
    pub details: Option<String>,
    pub ip_address: Option<String>,
}

impl ActivityLogEntry {
    pub fn new(user: String, action: String, path: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            user,
            action,
            path,
            details: None,
            ip_address: None,
        }
    }
}

// =============================================================================
// FILE VERSIONING
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileVersion {
    pub version_id: String,
    pub path: String,
    pub version_number: u32,
    pub created_at: DateTime<Utc>,
    pub size: u64,
    pub checksum: String, // SHA256
    pub author: String,
}

impl FileVersion {
    pub fn new(path: String, version_number: u32, size: u64, checksum: String, author: String) -> Self {
        Self {
            version_id: Uuid::new_v4().to_string(),
            path,
            version_number,
            created_at: Utc::now(),
            size,
            checksum,
            author,
        }
    }
}

// =============================================================================
// DUPLICATE DETECTION
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub checksum: String,
    pub total_size: u64,
    pub files: Vec<DuplicateFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateFile {
    pub path: String,
    pub size: u64,
    pub modified: DateTime<Utc>,
}

// =============================================================================
// THUMBNAIL CACHE
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThumbnailInfo {
    pub path: String,
    pub thumbnail_path: String,
    pub size: u32, // 150, 300, etc.
    pub created_at: DateTime<Utc>,
}

// =============================================================================
// STORAGE STATISTICS
// =============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct StorageStats {
    pub total_files: u64,
    pub total_dirs: u64,
    pub total_size: u64,
    pub file_types: HashMap<String, u64>,
    pub largest_files: Vec<LargeFile>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LargeFile {
    pub path: String,
    pub size: u64,
}

// =============================================================================
// API REQUEST/RESPONSE TYPES
// =============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct CompressRequest {
    pub paths: Vec<String>,
    pub archive_name: String,
    pub format: String, // "zip", "tar", "tar.gz"
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtractRequest {
    pub archive_path: String,
    pub destination: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CompressResponse {
    pub archive_path: String,
    pub size: u64,
    pub files_count: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchIndexRequest {
    pub query: String,
    pub content_search: bool, // Search inside files
    pub file_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchIndexResult {
    pub path: String,
    pub score: f32,
    pub matches: Vec<SearchMatch>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchMatch {
    pub line_number: u32,
    pub content: String,
    pub context_before: String,
    pub context_after: String,
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Calculate SHA256 checksum of a file
pub async fn calculate_checksum(path: &Path) -> Result<String, std::io::Error> {
    use sha2::{Sha256, Digest};
    use tokio::io::AsyncReadExt;
    
    let mut file = tokio::fs::File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 8192];
    
    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    
    Ok(format!("{:x}", hasher.finalize()))
}

/// Get MIME type from file extension
pub fn get_mime_type(filename: &str) -> &'static str {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        // Images
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        
        // Videos
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "avi" => "video/x-msvideo",
        "mkv" => "video/x-matroska",
        
        // Audio
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "flac" => "audio/flac",
        
        // Documents
        "pdf" => "application/pdf",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        
        // Archives
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        "7z" => "application/x-7z-compressed",
        
        // Code
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "html" => "text/html",
        "css" => "text/css",
        "txt" => "text/plain",
        "md" => "text/markdown",
        
        _ => "application/octet-stream",
    }
}

/// Check if file type supports text search
pub fn is_text_searchable(filename: &str) -> bool {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    matches!(
        ext.as_str(),
        "txt" | "md" | "json" | "xml" | "html" | "css" | "js" | "ts" |
        "rs" | "py" | "java" | "cpp" | "c" | "h" | "go" | "php" | "rb" |
        "sh" | "yml" | "yaml" | "toml" | "ini" | "cfg" | "conf"
    )
}

/// Check if file type can have thumbnails
pub fn can_generate_thumbnail(filename: &str) -> bool {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    matches!(
        ext.as_str(),
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" |
        "mp4" | "webm" | "avi" | "mkv" | "mov"
    )
}

// =============================================================================
// WARP ROUTE HANDLERS (IMPLEMENTATIONS)
// =============================================================================

/*
Example implementation for share link creation:

async fn create_share_link(
    req: CreateShareLinkRequest,
    db: Arc<Mutex<HashMap<String, ShareLink>>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut link = ShareLink::new(req.path.clone(), req.duration_hours);
    link.password = req.password;
    link.max_downloads = req.max_downloads;
    
    let link_id = link.id.clone();
    let url = format!("/share/{}", link_id);
    
    db.lock().unwrap().insert(link_id.clone(), link.clone());
    
    let response = ShareLinkResponse {
        link_id,
        url: url.clone(),
        expires_at: link.expires_at,
    };
    
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::CREATED,
    ))
}

async fn download_share_link(
    link_id: String,
    password: Option<String>,
    db: Arc<Mutex<HashMap<String, ShareLink>>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut links = db.lock().unwrap();
    
    if let Some(link) = links.get_mut(&link_id) {
        if !link.is_valid() {
            return Ok(warp::reply::with_status(
                warp::reply::json(&serde_json::json!({"error": "Link expired or exhausted"})),
                StatusCode::GONE,
            ));
        }
        
        if let Some(ref pw) = link.password {
            if password.as_ref() != Some(pw) {
                return Ok(warp::reply::with_status(
                    warp::reply::json(&serde_json::json!({"error": "Invalid password"})),
                    StatusCode::UNAUTHORIZED,
                ));
            }
        }
        
        link.download_count += 1;
        
        // Serve file here...
        Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"success": true})),
            StatusCode::OK,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"error": "Link not found"})),
            StatusCode::NOT_FOUND,
        ))
    }
}
*/

// =============================================================================
// DATABASE/STORAGE HELPERS
// =============================================================================

/// Simple JSON-based storage for share links
pub async fn save_share_links(links: &HashMap<String, ShareLink>) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(links)?;
    tokio::fs::write("./share_links.json", json).await?;
    Ok(())
}

pub async fn load_share_links() -> Result<HashMap<String, ShareLink>, std::io::Error> {
    match tokio::fs::read_to_string("./share_links.json").await {
        Ok(json) => {
            Ok(serde_json::from_str(&json).unwrap_or_default())
        }
        Err(_) => Ok(HashMap::new()),
    }
}

/// Simple JSON-based storage for trash items
pub async fn save_trash_items(items: &Vec<TrashItem>) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(items)?;
    tokio::fs::write("./trash_items.json", json).await?;
    Ok(())
}

pub async fn load_trash_items() -> Result<Vec<TrashItem>, std::io::Error> {
    match tokio::fs::read_to_string("./trash_items.json").await {
        Ok(json) => {
            Ok(serde_json::from_str(&json).unwrap_or_default())
        }
        Err(_) => Ok(Vec::new()),
    }
}

/// Simple JSON-based storage for activity log
pub async fn append_activity_log(entry: &ActivityLogEntry) -> Result<(), std::io::Error> {
    use tokio::io::AsyncWriteExt;
    
    let json = serde_json::to_string(entry)? + "\n";
    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("./activity_log.jsonl")
        .await?;
    
    file.write_all(json.as_bytes()).await?;
    Ok(())
}

pub async fn read_activity_log(limit: usize) -> Result<Vec<ActivityLogEntry>, std::io::Error> {
    use tokio::io::AsyncBufReadExt;
    
    let file = tokio::fs::File::open("./activity_log.jsonl").await?;
    let reader = tokio::io::BufReader::new(file);
    let mut lines = reader.lines();
    let mut entries = Vec::new();
    
    while let Some(line) = lines.next_line().await? {
        if let Ok(entry) = serde_json::from_str::<ActivityLogEntry>(&line) {
            entries.push(entry);
        }
    }
    
    entries.reverse(); // Most recent first
    entries.truncate(limit);
    Ok(entries)
}
