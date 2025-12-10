/// Archive management - ZIP/TAR creation and extraction
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Read, Write};
use uuid::Uuid;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;
use tar::Builder;
use flate2::write::GzEncoder;
use flate2::Compression;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Archive {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub archive_type: String, // "zip", "tar", "tar.gz", "tar.bz2"
    pub file_path: String,
    pub original_paths: String, // JSON array of original file paths
    pub file_count: i32,
    pub total_size_bytes: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateArchiveRequest {
    pub name: String,
    pub archive_type: String,
    pub file_paths: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtractArchiveRequest {
    pub archive_path: String,
    pub extract_to: String,
}

/// Create archive from files
pub async fn create_archive(
    pool: &SqlitePool,
    user_id: &str,
    req: CreateArchiveRequest,
) -> Result<Archive, Box<dyn std::error::Error + Send + Sync>> {
    let archive_id = Uuid::new_v4().to_string();
    let archive_filename = format!("{}.{}", req.name, req.archive_type);
    let archive_path = format!("./data/archives/{}/{}", user_id, archive_filename);
    
    // Create archives directory
    std::fs::create_dir_all(format!("./data/archives/{}", user_id))?;
    
    let mut total_size = 0i64;
    let file_count = req.file_paths.len() as i32;
    
    match req.archive_type.as_str() {
        "zip" => {
            let file = File::create(&archive_path)?;
            let mut zip = ZipWriter::new(file);
            let options = FileOptions::default()
                .compression_method(CompressionMethod::Deflated);
            
            for file_path in &req.file_paths {
                let full_path = format!("./data/{}", file_path);
                let path = Path::new(&full_path);
                
                if path.is_file() {
                    let mut f = File::open(&full_path)?;
                    let mut buffer = Vec::new();
                    f.read_to_end(&mut buffer)?;
                    total_size += buffer.len() as i64;
                    
                    zip.start_file(file_path, options)?;
                    zip.write_all(&buffer)?;
                }
            }
            
            zip.finish()?;
        }
        "tar" | "tar.gz" => {
            let tar_file = File::create(&archive_path)?;
            let mut tar_builder = if req.archive_type == "tar.gz" {
                let encoder = GzEncoder::new(tar_file, Compression::default());
                Builder::new(Box::new(encoder) as Box<dyn Write>)
            } else {
                Builder::new(Box::new(tar_file) as Box<dyn Write>)
            };
            
            for file_path in &req.file_paths {
                let full_path = format!("./data/{}", file_path);
                let path = Path::new(&full_path);
                
                if path.is_file() {
                    let mut file = File::open(&full_path)?;
                    let metadata = file.metadata()?;
                    total_size += metadata.len() as i64;
                    tar_builder.append_file(file_path, &mut file)?;
                }
            }
            
            tar_builder.finish()?;
        }
        _ => return Err("Unsupported archive type".into()),
    }
    
    let original_paths = serde_json::to_string(&req.file_paths)?;
    
    sqlx::query(
        "INSERT INTO archives 
         (id, user_id, name, archive_type, file_path, original_paths, file_count, total_size_bytes, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&archive_id)
    .bind(user_id)
    .bind(&req.name)
    .bind(&req.archive_type)
    .bind(&archive_path)
    .bind(&original_paths)
    .bind(file_count)
    .bind(total_size)
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM archives WHERE id = ?")
        .bind(&archive_id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
}

/// Extract archive
pub async fn extract_archive(
    pool: &SqlitePool,
    user_id: &str,
    req: ExtractArchiveRequest,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let archive_path = format!("./data/{}", req.archive_path);
    let extract_path = format!("./data/{}", req.extract_to);
    
    std::fs::create_dir_all(&extract_path)?;
    
    let mut extracted_files = Vec::new();
    
    // Detect archive type by extension
    let archive_type = if archive_path.ends_with(".zip") {
        "zip"
    } else if archive_path.ends_with(".tar.gz") || archive_path.ends_with(".tgz") {
        "tar.gz"
    } else if archive_path.ends_with(".tar") {
        "tar"
    } else {
        return Err("Unsupported archive type".into());
    };
    
    match archive_type {
        "zip" => {
            use zip::ZipArchive;
            let file = File::open(&archive_path)?;
            let mut archive = ZipArchive::new(file)?;
            
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                let outpath = PathBuf::from(&extract_path).join(file.name());
                
                if file.is_dir() {
                    std::fs::create_dir_all(&outpath)?;
                } else {
                    if let Some(p) = outpath.parent() {
                        std::fs::create_dir_all(p)?;
                    }
                    let mut outfile = File::create(&outpath)?;
                    std::io::copy(&mut file, &mut outfile)?;
                    extracted_files.push(outpath.to_string_lossy().to_string());
                }
            }
        }
        "tar" | "tar.gz" => {
            use tar::Archive;
            use flate2::read::GzDecoder;
            
            let file = File::open(&archive_path)?;
            let mut archive = if archive_type == "tar.gz" {
                let decoder = GzDecoder::new(file);
                Archive::new(Box::new(decoder) as Box<dyn Read>)
            } else {
                Archive::new(Box::new(file) as Box<dyn Read>)
            };
            
            archive.unpack(&extract_path)?;
            
            // List extracted files
            for entry in std::fs::read_dir(&extract_path)? {
                let entry = entry?;
                extracted_files.push(entry.path().to_string_lossy().to_string());
            }
        }
        _ => return Err("Unsupported archive type".into()),
    }
    
    Ok(extracted_files)
}

/// List archives for user
pub async fn list_archives(
    for entry in archive.entries()? {
        let entry = entry?;
        if let Ok(path) = entry.path() {
            extracted_files.push(path.to_string_lossy().to_string());
        }
    }
    
    Ok(extracted_files)
    */
    
    // Placeholder return
    Ok(vec![])
}

/// List archives for user
pub async fn list_archives(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<Vec<Archive>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM archives WHERE user_id = ? ORDER BY created_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

/// Delete archive
pub async fn delete_archive(
    pool: &SqlitePool,
    archive_id: &str,
    user_id: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let archive: Archive = sqlx::query_as(
        "SELECT * FROM archives WHERE id = ? AND user_id = ?"
    )
    .bind(archive_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    
    // Delete physical file
    if Path::new(&archive.file_path).exists() {
        std::fs::remove_file(&archive.file_path)?;
    }
    
    // Delete from database
    sqlx::query("DELETE FROM archives WHERE id = ? AND user_id = ?")
        .bind(archive_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    
    Ok(())
}

/// Get archive info
pub async fn get_archive_info(
    pool: &SqlitePool,
    archive_id: &str,
    user_id: &str,
) -> Result<Archive, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM archives WHERE id = ? AND user_id = ?"
    )
    .bind(archive_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
}

/// List archive contents without extracting
pub async fn list_archive_contents(
    archive_path: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut files = Vec::new();
    
    if archive_path.ends_with(".zip") {
        use zip::ZipArchive;
        let file = File::open(archive_path)?;
        let mut archive = ZipArchive::new(file)?;
        
        for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            files.push(file.name().to_string());
        }
    } else if archive_path.ends_with(".tar") || archive_path.ends_with(".tar.gz") {
        use tar::Archive;
        use flate2::read::GzDecoder;
        
        let file = File::open(archive_path)?;
        let mut archive = if archive_path.ends_with(".tar.gz") {
            let decoder = GzDecoder::new(file);
            Archive::new(Box::new(decoder) as Box<dyn Read>)
        } else {
            Archive::new(Box::new(file) as Box<dyn Read>)
        };
        
        for entry in archive.entries()? {
            let entry = entry?;
            if let Ok(path) = entry.path() {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }
    
    Ok(files)
}
    Ok(files)
    */
    
    Ok(vec![])
}
