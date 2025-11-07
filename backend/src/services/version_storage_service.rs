//! Version Storage Service
//! Handles differential storage, compression, and version management

use std::path::{Path, PathBuf};
use sqlx::SqlitePool;
use chrono::Utc;
use uuid::Uuid;
use flate2::Compression;
use flate2::write::{GzEncoder, GzDecoder};
use std::io::{Read, Write};

const VERSION_STORAGE_DIR: &str = "./data/versions";
const MAX_VERSIONS_PER_FILE: usize = 50;
const VERSION_RETENTION_DAYS: i64 = 90;
const COMPRESSION_THRESHOLD_BYTES: u64 = 1024; // 1KB - compress files larger than this

/// Version metadata stored in database
#[derive(Debug, Clone)]
pub struct VersionMetadata {
    pub id: String,
    pub file_id: String,
    pub version_number: i32,
    pub storage_path: String,
    pub original_size: i64,
    pub compressed_size: i64,
    pub is_compressed: bool,
    pub is_differential: bool,
    pub base_version_id: Option<String>,
    pub checksum: String,
    pub created_by: String,
    pub created_at: String,
    pub comment: Option<String>,
}

/// Initialize version storage directory
pub fn init_version_storage() -> std::io::Result<()> {
    std::fs::create_dir_all(VERSION_STORAGE_DIR)?;
    Ok(())
}

/// Create a new version with differential storage and compression
pub async fn create_version(
    pool: &SqlitePool,
    file_id: &str,
    file_path: &Path,
    user_id: &str,
    comment: Option<&str>,
) -> Result<VersionMetadata, Box<dyn std::error::Error + Send + Sync>> {
    // Read file content
    let file_content = tokio::fs::read(file_path).await?;
    let original_size = file_content.len() as i64;
    let checksum = calculate_checksum(&file_content);
    
    // Get current version number
    let current_version: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(version_number), 0) FROM file_versions WHERE file_id = ?"
    )
    .bind(file_id)
    .fetch_one(pool)
    .await
    .unwrap_or(0);
    
    let new_version_number = current_version + 1;
    let version_id = Uuid::new_v4().to_string();
    
    // Check if we should use differential storage
    let (is_differential, base_version_id, stored_content) = if new_version_number > 1 {
        // Try to create diff from previous version
        if let Some(prev_version) = get_previous_version(pool, file_id).await? {
            match create_diff(&prev_version, &file_content).await {
                Ok(diff_data) if diff_data.len() < file_content.len() => {
                    // Diff is smaller, use it
                    (true, Some(prev_version.id), diff_data)
                }
                _ => {
                    // Diff failed or is larger, store full content
                    (false, None, file_content)
                }
            }
        } else {
            (false, None, file_content)
        }
    } else {
        // First version, always store full content
        (false, None, file_content)
    };
    
    // Compress if beneficial
    let (is_compressed, final_content) = if stored_content.len() as u64 > COMPRESSION_THRESHOLD_BYTES {
        match compress_data(&stored_content) {
            Ok(compressed) if compressed.len() < stored_content.len() => {
                (true, compressed)
            }
            _ => (false, stored_content),
        }
    } else {
        (false, stored_content)
    };
    
    let compressed_size = final_content.len() as i64;
    
    // Store version file
    let storage_path = format!(
        "{}/{}/{}_v{}.dat",
        VERSION_STORAGE_DIR,
        file_id,
        version_id,
        new_version_number
    );
    
    let storage_dir = PathBuf::from(&storage_path).parent().unwrap().to_path_buf();
    tokio::fs::create_dir_all(&storage_dir).await?;
    tokio::fs::write(&storage_path, &final_content).await?;
    
    // Store metadata in database
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        r#"
        INSERT INTO file_versions 
        (id, file_id, version_number, storage_path, original_size, compressed_size, 
         is_compressed, is_differential, base_version_id, checksum, created_by, created_at, comment)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&version_id)
    .bind(file_id)
    .bind(new_version_number)
    .bind(&storage_path)
    .bind(original_size)
    .bind(compressed_size)
    .bind(is_compressed as i32)
    .bind(is_differential as i32)
    .bind(&base_version_id)
    .bind(&checksum)
    .bind(user_id)
    .bind(&now)
    .bind(comment)
    .execute(pool)
    .await?;
    
    // Cleanup old versions if limit exceeded
    cleanup_old_versions(pool, file_id).await?;
    
    Ok(VersionMetadata {
        id: version_id,
        file_id: file_id.to_string(),
        version_number: new_version_number,
        storage_path,
        original_size,
        compressed_size,
        is_compressed,
        is_differential,
        base_version_id,
        checksum,
        created_by: user_id.to_string(),
        created_at: now,
        comment: comment.map(|s| s.to_string()),
    })
}

/// Restore a version to original content
pub async fn restore_version(
    pool: &SqlitePool,
    version_id: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    // Get version metadata
    let version = get_version_metadata(pool, version_id).await?;
    
    // Read stored content
    let mut stored_content = tokio::fs::read(&version.storage_path).await?;
    
    // Decompress if needed
    if version.is_compressed {
        stored_content = decompress_data(&stored_content)?;
    }
    
    // Apply diff if needed
    if version.is_differential {
        if let Some(base_id) = version.base_version_id {
            // Recursively restore base version - requires boxing for async recursion
            let base_content = Box::pin(restore_version(pool, &base_id)).await?;
            // Apply diff
            stored_content = apply_diff(&base_content, &stored_content)?;
        }
    }
    
    Ok(stored_content)
}

/// Get version diff preview
pub async fn get_version_diff(
    pool: &SqlitePool,
    from_version_id: &str,
    to_version_id: &str,
) -> Result<VersionDiff, Box<dyn std::error::Error + Send + Sync>> {
    let from_content = restore_version(pool, from_version_id).await?;
    let to_content = restore_version(pool, to_version_id).await?;
    
    // Check if text file
    if is_text_file(&from_content) && is_text_file(&to_content) {
        let from_text = String::from_utf8_lossy(&from_content);
        let to_text = String::from_utf8_lossy(&to_content);
        
        // Line-by-line diff
        let diff_lines = compute_text_diff(&from_text, &to_text);
        
        Ok(VersionDiff {
            from_version_id: from_version_id.to_string(),
            to_version_id: to_version_id.to_string(),
            diff_type: DiffType::Text,
            text_diff: Some(diff_lines),
            binary_diff: None,
            size_change: to_content.len() as i64 - from_content.len() as i64,
        })
    } else {
        // Binary file - just metadata
        Ok(VersionDiff {
            from_version_id: from_version_id.to_string(),
            to_version_id: to_version_id.to_string(),
            diff_type: DiffType::Binary,
            text_diff: None,
            binary_diff: Some(BinaryDiff {
                from_size: from_content.len(),
                to_size: to_content.len(),
                checksum_changed: calculate_checksum(&from_content) != calculate_checksum(&to_content),
            }),
            size_change: to_content.len() as i64 - from_content.len() as i64,
        })
    }
}

/// Cleanup old versions (keep max 50, delete older than 90 days)
pub async fn cleanup_old_versions(
    pool: &SqlitePool,
    file_id: &str,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let mut deleted_count = 0;
    
    // Keep max 50 versions
    let versions_to_delete: Vec<(String, String)> = sqlx::query_as(
        r#"
        SELECT id, storage_path FROM file_versions 
        WHERE file_id = ? 
        ORDER BY version_number DESC 
        LIMIT -1 OFFSET ?
        "#
    )
    .bind(file_id)
    .bind(MAX_VERSIONS_PER_FILE as i32)
    .fetch_all(pool)
    .await?;
    
    for (version_id, storage_path) in versions_to_delete {
        // Delete file
        let _ = tokio::fs::remove_file(&storage_path).await;
        
        // Delete from database
        sqlx::query("DELETE FROM file_versions WHERE id = ?")
            .bind(&version_id)
            .execute(pool)
            .await?;
        
        deleted_count += 1;
    }
    
    // Delete versions older than 90 days
    let cutoff = (Utc::now() - chrono::Duration::days(VERSION_RETENTION_DAYS)).to_rfc3339();
    let old_versions: Vec<(String, String)> = sqlx::query_as(
        "SELECT id, storage_path FROM file_versions WHERE file_id = ? AND created_at < ?"
    )
    .bind(file_id)
    .bind(&cutoff)
    .fetch_all(pool)
    .await?;
    
    for (version_id, storage_path) in old_versions {
        let _ = tokio::fs::remove_file(&storage_path).await;
        sqlx::query("DELETE FROM file_versions WHERE id = ?")
            .bind(&version_id)
            .execute(pool)
            .await?;
        deleted_count += 1;
    }
    
    Ok(deleted_count)
}

/// Background cleanup job for all files
pub async fn cleanup_all_old_versions(pool: &SqlitePool) -> Result<usize, sqlx::Error> {
    let file_ids: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT file_id FROM file_versions"
    )
    .fetch_all(pool)
    .await?;
    
    let mut total_deleted = 0;
    for file_id in file_ids {
        if let Ok(count) = cleanup_old_versions(pool, &file_id).await {
            total_deleted += count;
        }
    }
    
    Ok(total_deleted)
}

// ==================== HELPER FUNCTIONS ====================

fn calculate_checksum(data: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn compress_data(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    encoder.finish()
}

fn decompress_data(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    use std::io::Cursor;
    let cursor = Cursor::new(data);
    let mut decoder = flate2::read::GzDecoder::new(cursor);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result)?;
    Ok(result)
}

async fn create_diff(
    prev_version: &VersionMetadata,
    new_content: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    // Load previous version content
    let prev_content = tokio::fs::read(&prev_version.storage_path).await?;
    
    // For text files, use line-based diff
    if is_text_file(&prev_content) && is_text_file(new_content) {
        let prev_text = String::from_utf8_lossy(&prev_content);
        let new_text = String::from_utf8_lossy(new_content);
        
        // Use similar crate for text diff
        use similar::TextDiff;
        let diff = TextDiff::from_lines(&prev_text, &new_text);
        
        // Serialize diff operations as JSON
        let mut operations = Vec::new();
        for op in diff.iter_all_changes() {
            operations.push(serde_json::json!({
                "tag": format!("{:?}", op.tag()),
                "value": op.value()
            }));
        }
        
        let diff_json = serde_json::to_vec(&operations)?;
        return Ok(diff_json);
    }
    
    // For binary files, use simple RLE compression
    let mut diff_data = Vec::new();
    
    // Simple byte-level diff: store changed byte ranges
    let min_len = prev_content.len().min(new_content.len());
    let mut i = 0;
    
    while i < min_len {
        if prev_content[i] != new_content[i] {
            // Find extent of change
            let start = i;
            while i < min_len && prev_content[i] != new_content[i] {
                i += 1;
            }
            
            // Store: [operation: 1=change] [start: u32] [length: u32] [data]
            diff_data.push(1u8); // Change operation
            diff_data.extend_from_slice(&(start as u32).to_le_bytes());
            diff_data.extend_from_slice(&((i - start) as u32).to_le_bytes());
            diff_data.extend_from_slice(&new_content[start..i]);
        } else {
            i += 1;
        }
    }
    
    // If new file is longer, append the rest
    if new_content.len() > prev_content.len() {
        diff_data.push(2u8); // Append operation
        diff_data.extend_from_slice(&(prev_content.len() as u32).to_le_bytes());
        diff_data.extend_from_slice(&((new_content.len() - prev_content.len()) as u32).to_le_bytes());
        diff_data.extend_from_slice(&new_content[prev_content.len()..]);
    }
    
    // If new file is shorter, record truncation
    if new_content.len() < prev_content.len() {
        diff_data.push(3u8); // Truncate operation
        diff_data.extend_from_slice(&(new_content.len() as u32).to_le_bytes());
    }
    
    Ok(diff_data)
}

fn apply_diff(base_content: &[u8], diff_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    // Check if it's a JSON text diff
    if let Ok(operations) = serde_json::from_slice::<Vec<serde_json::Value>>(diff_data) {
        // Text diff - reconstruct from operations
        let mut result = String::new();
        
        for op in &operations {  // Borrow operations instead of consuming
            if let Some(tag) = op.get("tag").and_then(|v| v.as_str()) {
                if let Some(value) = op.get("value").and_then(|v| v.as_str()) {
                    match tag {
                        "\"Equal\"" | "Equal" => result.push_str(value),
                        "\"Insert\"" | "Insert" => result.push_str(value),
                        // Skip Delete operations
                        _ => {}
                    }
                }
            }
        }
        
        return Ok(result.into_bytes());
    }
    
    // Binary diff - apply operations
    let mut result = base_content.to_vec();
    let mut cursor = 0;
    
    while cursor < diff_data.len() {
        let operation = diff_data[cursor];
        cursor += 1;
        
        match operation {
            1 => { // Change operation
                if cursor + 8 > diff_data.len() {
                    break;
                }
                
                let start = u32::from_le_bytes([
                    diff_data[cursor],
                    diff_data[cursor + 1],
                    diff_data[cursor + 2],
                    diff_data[cursor + 3],
                ]) as usize;
                cursor += 4;
                
                let length = u32::from_le_bytes([
                    diff_data[cursor],
                    diff_data[cursor + 1],
                    diff_data[cursor + 2],
                    diff_data[cursor + 3],
                ]) as usize;
                cursor += 4;
                
                if cursor + length > diff_data.len() {
                    break;
                }
                
                // Apply change
                if start + length <= result.len() {
                    result[start..start + length].copy_from_slice(&diff_data[cursor..cursor + length]);
                }
                cursor += length;
            }
            2 => { // Append operation
                if cursor + 8 > diff_data.len() {
                    break;
                }
                
                cursor += 4; // Skip start position (always at end)
                
                let length = u32::from_le_bytes([
                    diff_data[cursor],
                    diff_data[cursor + 1],
                    diff_data[cursor + 2],
                    diff_data[cursor + 3],
                ]) as usize;
                cursor += 4;
                
                if cursor + length > diff_data.len() {
                    break;
                }
                
                // Append data
                result.extend_from_slice(&diff_data[cursor..cursor + length]);
                cursor += length;
            }
            3 => { // Truncate operation
                if cursor + 4 > diff_data.len() {
                    break;
                }
                
                let new_length = u32::from_le_bytes([
                    diff_data[cursor],
                    diff_data[cursor + 1],
                    diff_data[cursor + 2],
                    diff_data[cursor + 3],
                ]) as usize;
                cursor += 4;
                
                // Truncate to new length
                result.truncate(new_length);
            }
            _ => {
                // Unknown operation, stop
                break;
            }
        }
    }
    
    Ok(result)
}

fn is_text_file(content: &[u8]) -> bool {
    // Check if content is valid UTF-8
    String::from_utf8(content.to_vec()).is_ok()
}

fn compute_text_diff(from: &str, to: &str) -> Vec<DiffLine> {
    use similar::{ChangeTag, TextDiff};
    
    let diff = TextDiff::from_lines(from, to);
    let mut result = Vec::new();
    let mut line_num = 0;
    
    for change in diff.iter_all_changes() {
        let change_type = match change.tag() {
            ChangeTag::Delete => {
                line_num += 1;
                ChangeType::Deleted
            }
            ChangeTag::Insert => {
                line_num += 1;
                ChangeType::Added
            }
            ChangeTag::Equal => {
                line_num += 1;
                ChangeType::Unchanged
            }
        };
        
        // Only include changed lines and limited context
        if matches!(change_type, ChangeType::Deleted | ChangeType::Added) 
            || (line_num > 0 && result.len() < 1000) {  // Limit output size
            result.push(DiffLine {
                line_number: line_num,
                change_type,
                content: change.value().trim_end().to_string(),
            });
        }
    }
    
    result
}

async fn get_previous_version(
    pool: &SqlitePool,
    file_id: &str,
) -> Result<Option<VersionMetadata>, sqlx::Error> {
    let row: Option<(String, i32, String, i64, i64, i32, i32, Option<String>, String, String, String, Option<String>)> = sqlx::query_as(
        r#"
        SELECT id, version_number, storage_path, original_size, compressed_size,
               is_compressed, is_differential, base_version_id, checksum, created_by, created_at, comment
        FROM file_versions 
        WHERE file_id = ? 
        ORDER BY version_number DESC 
        LIMIT 1
        "#
    )
    .bind(file_id)
    .fetch_optional(pool)
    .await?;
    
    Ok(row.map(|(id, version_number, storage_path, original_size, compressed_size, is_compressed, is_differential, base_version_id, checksum, created_by, created_at, comment)| {
        VersionMetadata {
            id,
            file_id: file_id.to_string(),
            version_number,
            storage_path,
            original_size,
            compressed_size,
            is_compressed: is_compressed != 0,
            is_differential: is_differential != 0,
            base_version_id,
            checksum,
            created_by,
            created_at,
            comment,
        }
    }))
}

async fn get_version_metadata(
    pool: &SqlitePool,
    version_id: &str,
) -> Result<VersionMetadata, sqlx::Error> {
    let row: (String, i32, String, i64, i64, i32, i32, Option<String>, String, String, String, Option<String>) = sqlx::query_as(
        r#"
        SELECT file_id, version_number, storage_path, original_size, compressed_size,
               is_compressed, is_differential, base_version_id, checksum, created_by, created_at, comment
        FROM file_versions 
        WHERE id = ?
        "#
    )
    .bind(version_id)
    .fetch_one(pool)
    .await?;
    
    Ok(VersionMetadata {
        id: version_id.to_string(),
        file_id: row.0,
        version_number: row.1,
        storage_path: row.2,
        original_size: row.3,
        compressed_size: row.4,
        is_compressed: row.5 != 0,
        is_differential: row.6 != 0,
        base_version_id: row.7,
        checksum: row.8,
        created_by: row.9,
        created_at: row.10,
        comment: row.11,
    })
}

// ==================== PUBLIC TYPES ====================

#[derive(Debug, serde::Serialize)]
pub struct VersionDiff {
    pub from_version_id: String,
    pub to_version_id: String,
    pub diff_type: DiffType,
    pub text_diff: Option<Vec<DiffLine>>,
    pub binary_diff: Option<BinaryDiff>,
    pub size_change: i64,
}

#[derive(Debug, serde::Serialize)]
pub enum DiffType {
    Text,
    Binary,
}

#[derive(Debug, serde::Serialize)]
pub struct DiffLine {
    pub line_number: usize,
    pub change_type: ChangeType,
    pub content: String,
}

#[derive(Debug, serde::Serialize)]
pub enum ChangeType {
    Added,
    Deleted,
    Modified,
    Unchanged,
}

#[derive(Debug, serde::Serialize)]
pub struct BinaryDiff {
    pub from_size: usize,
    pub to_size: usize,
    pub checksum_changed: bool,
}
