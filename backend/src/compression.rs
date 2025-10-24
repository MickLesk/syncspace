/// Automatic file compression for cold storage
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::Path;
use uuid::Uuid;

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

/// Compress file (placeholder - requires compression crates)
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
            "bzip2" => "bz2",
            "zstd" => "zst",
            "lz4" => "lz4",
            _ => "gz",
        }
    );
    
    let original_size = std::fs::metadata(file_path)?.len() as i64;
    
    // Placeholder - in production use compression crates
    /*
    Example with gzip (flate2):
    
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::fs::File;
    use std::io::{Read, Write};
    
    let mut input = File::open(file_path)?;
    let output = File::create(&compressed_path)?;
    let mut encoder = GzEncoder::new(output, Compression::default());
    
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;
    encoder.write_all(&buffer)?;
    encoder.finish()?;
    */
    
    /*
    Example with zstd:
    
    use zstd::stream::copy_encode;
    use std::fs::File;
    
    let mut input = File::open(file_path)?;
    let output = File::create(&compressed_path)?;
    copy_encode(&mut input, output, 0)?; // 0 = default compression level
    */
    
    /*
    Example with lz4:
    
    use lz4::EncoderBuilder;
    use std::fs::File;
    use std::io::Read;
    
    let mut input = File::open(file_path)?;
    let output = File::create(&compressed_path)?;
    let mut encoder = EncoderBuilder::new().level(4).build(output)?;
    
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;
    encoder.write_all(&buffer)?;
    let (_, result) = encoder.finish();
    result?;
    */
    
    let compressed_size = 0i64; // Placeholder
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
    
    // Optionally delete original file after compression
    // std::fs::remove_file(file_path)?;
    
    sqlx::query_as("SELECT * FROM compressed_files WHERE id = ?")
        .bind(&compressed_id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
}

/// Decompress file (placeholder)
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
    
    // Placeholder - in production use compression crates
    /*
    Example with gzip (flate2):
    
    use flate2::read::GzDecoder;
    use std::fs::File;
    use std::io::{Read, Write};
    
    let input = File::open(&compressed_file.compressed_path)?;
    let mut decoder = GzDecoder::new(input);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;
    
    let mut output = File::create(&compressed_file.original_path)?;
    output.write_all(&buffer)?;
    */
    
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
            "SELECT id, file_path FROM files 
             WHERE created_at < ? 
             AND size_bytes >= ? 
             AND file_path LIKE ?
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
