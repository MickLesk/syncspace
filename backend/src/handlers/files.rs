//! File operation handlers
//! Handles file upload, download, directory listing, rename, move, copy, delete

use crate::auth;
use crate::{AppState, FileChangeEvent};
use axum::{
    body::Body,
    extract::{Multipart, Path as AxumPath, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

const BASE_DIR: &str = "./data";

#[derive(Debug, Clone, Deserialize)]
pub struct RenameRequest {
    pub new_path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    pub modified: String,
}

/// List files in root directory
pub async fn list_files_root(
    State(_state): State<AppState>,
    _user: auth::User,
) -> Result<Json<Vec<FileInfo>>, StatusCode> {
    list_directory_contents(BASE_DIR).await
}

/// List files in a directory
pub async fn list_files_handler(
    State(_state): State<AppState>,
    _user: auth::User,
    AxumPath(path): AxumPath<String>,
) -> Result<Json<Vec<FileInfo>>, StatusCode> {
    let full_path = format!("{}/{}", BASE_DIR, path);
    list_directory_contents(&full_path).await
}

async fn list_directory_contents(dir_path: &str) -> Result<Json<Vec<FileInfo>>, StatusCode> {
    let path = Path::new(dir_path);
    
    if !path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }
    
    let mut entries = Vec::new();
    let mut read_dir = fs::read_dir(path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    while let Some(entry) = read_dir.next_entry().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        let metadata = entry.metadata().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        let relative_path = entry.path().strip_prefix(BASE_DIR)
            .unwrap_or(entry.path().as_path())
            .to_string_lossy()
            .to_string();
        
        entries.push(FileInfo {
            name: file_name,
            path: relative_path,
            size: metadata.len(),
            is_dir: metadata.is_dir(),
            modified: metadata.modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, 0))
                .flatten()
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "unknown".to_string()),
        });
    }
    
    Ok(Json(entries))
}

/// Download a file
pub async fn download_file_handler(
    State(_state): State<AppState>,
    _user: auth::User,
    AxumPath(path): AxumPath<String>,
) -> Result<Response, StatusCode> {
    let file_path = format!("{}/{}", BASE_DIR, path);
    let path_obj = Path::new(&file_path);
    
    if !path_obj.exists() || path_obj.is_dir() {
        return Err(StatusCode::NOT_FOUND);
    }
    
    let content = fs::read(&file_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let file_name = path_obj.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("download");
    
    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", file_name))
        .body(Body::from(content))
        .unwrap())
}

/// Upload a file
pub async fn upload_file_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(path): AxumPath<String>,
    body: axum::body::Bytes,
) -> Result<StatusCode, StatusCode> {
    let file_path = format!("{}/{}", BASE_DIR, path);
    let path_obj = Path::new(&file_path);
    
    // Create parent directories if they don't exist
    if let Some(parent) = path_obj.parent() {
        fs::create_dir_all(parent).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    let mut file = fs::File::create(&file_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    file.write_all(&body).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Broadcast file change event
    let _ = state.fs_tx.send(FileChangeEvent {
        path: path.clone(),
        kind: "create".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        user_id: Some(user.id.to_string()),
        metadata: None,
    });
    
    Ok(StatusCode::CREATED)
}

/// Upload multipart file
pub async fn upload_multipart_handler(
    State(state): State<AppState>,
    user: auth::User,
    mut multipart: Multipart,
) -> Result<StatusCode, StatusCode> {
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.file_name().unwrap_or("upload").to_string();
        let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
        
        let file_path = format!("{}/{}", BASE_DIR, name);
        let mut file = fs::File::create(&file_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        file.write_all(&data).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let _ = state.fs_tx.send(FileChangeEvent {
            path: name,
            kind: "create".to_string(),
            timestamp: Utc::now().to_rfc3339(),
            user_id: Some(user.id.to_string()),
            metadata: None,
        });
    }
    
    Ok(StatusCode::CREATED)
}

/// Create directory
pub async fn create_dir_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(path): AxumPath<String>,
) -> Result<StatusCode, StatusCode> {
    let dir_path = format!("{}/{}", BASE_DIR, path);
    fs::create_dir_all(&dir_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let _ = state.fs_tx.send(FileChangeEvent {
        path,
        kind: "create".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        user_id: Some(user.id.to_string()),
        metadata: None,
    });
    
    Ok(StatusCode::CREATED)
}

/// Rename/move file
pub async fn rename_file_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(path): AxumPath<String>,
    Json(req): Json<RenameRequest>,
) -> Result<StatusCode, StatusCode> {
    let old_path = format!("{}/{}", BASE_DIR, path);
    let new_path = format!("{}/{}", BASE_DIR, req.new_path);
    
    fs::rename(&old_path, &new_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let _ = state.fs_tx.send(FileChangeEvent {
        path,
        kind: "rename".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        user_id: Some(user.id.to_string()),
        metadata: Some(serde_json::json!({ "new_path": req.new_path })),
    });
    
    Ok(StatusCode::OK)
}

/// Move file
pub async fn move_file_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(path): AxumPath<String>,
    Json(req): Json<RenameRequest>,
) -> Result<StatusCode, StatusCode> {
    rename_file_handler(State(state), user, AxumPath(path), Json(req)).await
}

/// Copy file
pub async fn copy_file_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(path): AxumPath<String>,
    Json(req): Json<RenameRequest>,
) -> Result<StatusCode, StatusCode> {
    let src_path = format!("{}/{}", BASE_DIR, path);
    let dst_path = format!("{}/{}", BASE_DIR, req.new_path);
    
    fs::copy(&src_path, &dst_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let _ = state.fs_tx.send(FileChangeEvent {
        path: req.new_path.clone(),
        kind: "create".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        user_id: Some(user.id.to_string()),
        metadata: Some(serde_json::json!({ "copied_from": path })),
    });
    
    Ok(StatusCode::CREATED)
}

/// Delete file or directory
pub async fn delete_file_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(path): AxumPath<String>,
) -> Result<StatusCode, StatusCode> {
    let file_path = format!("{}/{}", BASE_DIR, path);
    let path_obj = Path::new(&file_path);
    
    if !path_obj.exists() {
        return Err(StatusCode::NOT_FOUND);
    }
    
    if path_obj.is_dir() {
        fs::remove_dir_all(&file_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        fs::remove_file(&file_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    let _ = state.fs_tx.send(FileChangeEvent {
        path,
        kind: "delete".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        user_id: Some(user.id.to_string()),
        metadata: None,
    });
    
    Ok(StatusCode::NO_CONTENT)
}
