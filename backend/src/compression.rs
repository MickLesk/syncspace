/// Automatic file compression for cold storage
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::Path;
use uuid::Uuid;
use tokio::io::AsyncWriteExt;
use async_compression::tokio::write::{GzipEncoder, BrotliEncoder, ZstdEncoder};
use async_compression::tokio::bufread::{GzipDecoder, BrotliDecoder, ZstdDecoder};
use tokio::io::{AsyncReadExt, BufReader};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CompressionRule {
    pub id: String,
    pub name: String,
    pub file_pattern: String, // e.g., "*.log", "*.txt"
    pub min_age_days: i32,
    pub min_size_bytes: i64,
    pub compression_algorithm: String, // "gzip", "bzip2", "zstd", "lz4"
    pub enabled: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CompressedFile {
    pub id: String,
    pub original_file_id: String,
    pub original_path: String,
    pub compressed_path: String,
    pub original_size_bytes: i64,
    pub compressed_size_bytes: i64,
    pub compression_ratio: f64,
    pub algorithm: String,
    pub compressed_at: String,
    pub is_decompressed: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCompressionRuleRequest {
    pub name: String,
    pub file_pattern: String,
    pub min_age_days: i32,
    pub min_size_bytes: i64,
    pub compression_algorithm: String,
    pub enabled: bool,
}

/// Add compression rule
pub async fn add_compression_rule(
    pool: &SqlitePool,
    req: CreateCompressionRuleRequest,
) -> Result<CompressionRule, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO compression_rules 
         (id, name, file_pattern, min_age_days, min_size_bytes, compression_algorithm, enabled, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&id)
    .bind(&req.name)
    .bind(&req.file_pattern)
    .bind(req.min_age_days)
    .bind(req.min_size_bytes)
    .bind(&req.compression_algorithm)
    .bind(if req.enabled { 1 } else { 0 })
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM compression_rules WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Compress file using specified algorithm
pub async fn compress_file(
    pool: &SqlitePool,
    file_id: &str,
    file_path: &str,
    algorithm: &str,
) -> Result<CompressedFile, Box<dyn std::error::Error + Send + Sync>> {
    let compressed_id = Uuid::new_v4().to_string();
    let compressed_path = format!("{}.{}", file_path, 
        match algorithm {
            "gzip" => "gz",
            "brotli" => "br",
            "zstd" => "zst",
            _ => "gz",
        }
    );
    
    let original_size = tokio::fs::metadata(file_path).await?.len() as i64;
    
    // Read input file
    let input_data = tokio::fs::read(file_path).await?;
    
    // Compress based on algorithm
    let compressed_data = match algorithm {
        "gzip" => {
            let mut encoder = GzipEncoder::new(Vec::new());
            encoder.write_all(&input_data).await?;
            encoder.shutdown().await?;
            encoder.into_inner()
        },
        "brotli" => {
            let mut encoder = BrotliEncoder::new(Vec::new());
            encoder.write_all(&input_data).await?;
            encoder.shutdown().await?;
            encoder.into_inner()
        },
        "zstd" => {
            let mut encoder = ZstdEncoder::new(Vec::new());
            encoder.write_all(&input_data).await?;
            encoder.shutdown().await?;
            encoder.into_inner()
        },
        _ => return Err("Unsupported compression algorithm".into()),
    };
    
    // Write compressed file
    tokio::fs::write(&compressed_path, &compressed_data).await?;
    
    let compressed_size = compressed_data.len() as i64;
    let compression_ratio = if original_size > 0 {
        compressed_size as f64 / original_size as f64
    } else {
        1.0
    };
    
    sqlx::query(
        "INSERT INTO compressed_files 
         (id, original_file_id, original_path, compressed_path, original_size_bytes, 
          compressed_size_bytes, compression_ratio, algorithm, compressed_at, is_decompressed)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), 0)"
    )
    .bind(&compressed_id)
    .bind(file_id)
    .bind(file_path)
    .bind(&compressed_path)
    .bind(original_size)
    .bind(compressed_size)
    .bind(compression_ratio)
    .bind(algorithm)
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM compressed_files WHERE id = ?")
        .bind(&compressed_id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
}

/// Decompress file
pub async fn decompress_file(
    pool: &SqlitePool,
    compressed_id: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let compressed_file: CompressedFile = sqlx::query_as(
        "SELECT * FROM compressed_files WHERE id = ?"
    )
    .bind(compressed_id)
    .fetch_one(pool)
    .await?;
    
    // Read compressed file
    let compressed_data = tokio::fs::read(&compressed_file.compressed_path).await?;
    
    // Decompress based on algorithm
    let decompressed_data = match compressed_file.algorithm.as_str() {
        "gzip" => {
            let reader = BufReader::new(&compressed_data[..]);
            let mut decoder = GzipDecoder::new(reader);
            let mut output = Vec::new();
            decoder.read_to_end(&mut output).await?;
            output
        },
        "brotli" => {
            let reader = BufReader::new(&compressed_data[..]);
            let mut decoder = BrotliDecoder::new(reader);
            let mut output = Vec::new();
            decoder.read_to_end(&mut output).await?;
            output
        },
        "zstd" => {
            let reader = BufReader::new(&compressed_data[..]);
            let mut decoder = ZstdDecoder::new(reader);
            let mut output = Vec::new();
            decoder.read_to_end(&mut output).await?;
            output
        },
        _ => return Err("Unsupported decompression algorithm".into()),
    };
    
    // Write decompressed file
    tokio::fs::write(&compressed_file.original_path, &decompressed_data).await?;
    
    sqlx::query(
        "UPDATE compressed_files SET is_decompressed = 1 WHERE id = ?"
    )
    .bind(compressed_id)
    .execute(pool)
    .await?;
    
    Ok(compressed_file.original_path)
}

/// Run compression job - compress files matching rules
pub async fn run_compression_job(
    pool: &SqlitePool,
) -> Result<CompressionJobResult, Box<dyn std::error::Error + Send + Sync>> {
    let rules: Vec<CompressionRule> = sqlx::query_as(
        "SELECT * FROM compression_rules WHERE enabled = 1"
    )
    .fetch_all(pool)
    .await?;
    
    let mut compressed_count = 0;
    let mut total_bytes_saved = 0i64;
    
    for rule in rules {
        // Find files matching pattern and age criteria
        let cutoff_date = chrono::Utc::now() - chrono::Duration::days(rule.min_age_days as i64);
        
        let files: Vec<(String, String)> = sqlx::query_as(
            "SELECT id, path FROM files 
             WHERE created_at < ? 
             AND size_bytes >= ? 
             AND path LIKE ?
             AND id NOT IN (SELECT original_file_id FROM compressed_files)"
        )
        .bind(cutoff_date.format("%Y-%m-%d %H:%M:%S").to_string())
        .bind(rule.min_size_bytes)
        .bind(rule.file_pattern.replace("*", "%"))
        .fetch_all(pool)
        .await?;
        
        for (file_id, file_path) in files {
            match compress_file(pool, &file_id, &file_path, &rule.compression_algorithm).await {
                Ok(compressed) => {
                    compressed_count += 1;
                    total_bytes_saved += compressed.original_size_bytes - compressed.compressed_size_bytes;
                },
                Err(e) => {
                    eprintln!("Failed to compress {}: {}", file_path, e);
                }
            }
        }
    }
    
    Ok(CompressionJobResult {
        files_compressed: compressed_count,
        bytes_saved: total_bytes_saved,
    })
}

#[derive(Debug, Clone, Serialize)]
pub struct CompressionJobResult {
    pub files_compressed: u32,
    pub bytes_saved: i64,
}

/// List compression rules
pub async fn list_compression_rules(
    pool: &SqlitePool,
) -> Result<Vec<CompressionRule>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM compression_rules ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
}

/// List compressed files
pub async fn list_compressed_files(
    pool: &SqlitePool,
) -> Result<Vec<CompressedFile>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM compressed_files ORDER BY compressed_at DESC"
    )
    .fetch_all(pool)
    .await
}

/// Delete compression rule
pub async fn delete_compression_rule(
    pool: &SqlitePool,
    rule_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM compression_rules WHERE id = ?")
        .bind(rule_id)
        .execute(pool)
        .await?;
    
    Ok(())
}

/// Get compression statistics
pub async fn get_compression_stats(
    pool: &SqlitePool,
) -> Result<CompressionStats, sqlx::Error> {
    let result: (i64, i64, i64) = sqlx::query_as(
        "SELECT 
            COUNT(*) as total_files,
            SUM(original_size_bytes) as total_original_size,
            SUM(compressed_size_bytes) as total_compressed_size
         FROM compressed_files"
    )
    .fetch_one(pool)
    .await?;
    
    let total_files = result.0;
    let total_original_size = result.1;
    let total_compressed_size = result.2;
    
    let bytes_saved = total_original_size - total_compressed_size;
    let average_compression_ratio = if total_original_size > 0 {
        total_compressed_size as f64 / total_original_size as f64
    } else {
        1.0
    };
    
    Ok(CompressionStats {
        total_files,
        total_original_size,
        total_compressed_size,
        bytes_saved,
        average_compression_ratio,
    })
}

#[derive(Debug, Clone, Serialize)]
pub struct CompressionStats {
    pub total_files: i64,
    pub total_original_size: i64,
    pub total_compressed_size: i64,
    pub bytes_saved: i64,
    pub average_compression_ratio: f64,
}
