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
    // Simple byte-level diff using RLE-like compression
    // In production, use a proper diff library like `diff` or `similar`
    
    // For now, just return new content if different
    // TODO: Implement proper binary diff
    Ok(new_content.to_vec())
}

fn apply_diff(base_content: &[u8], diff_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    // Apply diff to base content
    // TODO: Implement proper diff application
    Ok(diff_data.to_vec())
}

fn is_text_file(content: &[u8]) -> bool {
    // Check if content is valid UTF-8
    String::from_utf8(content.to_vec()).is_ok()
}

fn compute_text_diff(from: &str, to: &str) -> Vec<DiffLine> {
    let from_lines: Vec<&str> = from.lines().collect();
    let to_lines: Vec<&str> = to.lines().collect();
    
    let mut result = Vec::new();
    
    // Simple line-by-line comparison
    let max_len = from_lines.len().max(to_lines.len());
    
    for i in 0..max_len {
        match (from_lines.get(i), to_lines.get(i)) {
            (Some(&from_line), Some(&to_line)) => {
                if from_line == to_line {
                    result.push(DiffLine {
                        line_number: i + 1,
                        change_type: ChangeType::Unchanged,
                        content: from_line.to_string(),
                    });
                } else {
                    result.push(DiffLine {
                        line_number: i + 1,
                        change_type: ChangeType::Modified,
                        content: format!("- {}\n+ {}", from_line, to_line),
                    });
                }
            }
            (Some(&from_line), None) => {
                result.push(DiffLine {
                    line_number: i + 1,
                    change_type: ChangeType::Deleted,
                    content: format!("- {}", from_line),
                });
            }
            (None, Some(&to_line)) => {
                result.push(DiffLine {
                    line_number: i + 1,
                    change_type: ChangeType::Added,
                    content: format!("+ {}", to_line),
                });
            }
            (None, None) => break,
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
