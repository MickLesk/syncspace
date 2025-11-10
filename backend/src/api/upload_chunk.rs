use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::{auth::UserInfo, AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkFinalizeRequest {
    pub path: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "uploadId")]
    pub upload_id: String,
    #[serde(rename = "totalChunks")]
    pub total_chunks: usize,
}

#[derive(Debug, Serialize)]
pub struct ChunkUploadResponse {
    pub success: bool,
    pub chunk_index: usize,
    pub total_chunks: usize,
}

#[derive(Debug, Serialize)]
pub struct ChunkFinalizeResponse {
    pub success: bool,
    pub message: String,
    pub file_path: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/upload-chunk", post(upload_chunk))
        .route("/upload-chunk-finalize", post(finalize_chunked_upload))
}

/// Upload a single chunk of a large file
async fn upload_chunk(
    State(_state): State<AppState>,
    _user: UserInfo,
    mut multipart: Multipart,
) -> Result<Json<ChunkUploadResponse>, StatusCode> {
    let mut chunk_data: Option<Vec<u8>> = None;
    let mut _path: Option<String> = None;
    let mut file_name: Option<String> = None;
    let mut chunk_index: Option<usize> = None;
    let mut total_chunks: Option<usize> = None;
    let mut upload_id: Option<String> = None;

    // Parse multipart fields
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "chunk" => {
                chunk_data = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|_| StatusCode::BAD_REQUEST)?
                        .to_vec(),
                );
            }
            "path" => {
                _path = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            "fileName" => {
                file_name = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            "chunkIndex" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                chunk_index = Some(text.parse().map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            "totalChunks" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                total_chunks = Some(text.parse().map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            "uploadId" => {
                upload_id = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            _ => {}
        }
    }

    // Validate required fields
    let chunk_data = chunk_data.ok_or(StatusCode::BAD_REQUEST)?;
    let file_name = file_name.ok_or(StatusCode::BAD_REQUEST)?;
    let chunk_index = chunk_index.ok_or(StatusCode::BAD_REQUEST)?;
    let total_chunks = total_chunks.ok_or(StatusCode::BAD_REQUEST)?;
    let upload_id = upload_id.ok_or(StatusCode::BAD_REQUEST)?;

    // Create temp directory for chunks
    let temp_dir = PathBuf::from("./data/temp_uploads");
    fs::create_dir_all(&temp_dir)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create upload-specific directory
    let upload_dir = temp_dir.join(&upload_id);
    fs::create_dir_all(&upload_dir)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Save chunk to temp file
    let chunk_path = upload_dir.join(format!("chunk_{:06}", chunk_index));
    let mut file = fs::File::create(&chunk_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    file.write_all(&chunk_data)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    file.flush()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tracing::info!(
        "Chunk {}/{} received for file '{}' (upload_id: {})",
        chunk_index + 1,
        total_chunks,
        file_name,
        upload_id
    );

    Ok(Json(ChunkUploadResponse {
        success: true,
        chunk_index,
        total_chunks,
    }))
}

/// Finalize chunked upload by merging all chunks
async fn finalize_chunked_upload(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<ChunkFinalizeRequest>,
) -> Result<Json<ChunkFinalizeResponse>, StatusCode> {
    let temp_dir = PathBuf::from("./data/temp_uploads");
    let upload_dir = temp_dir.join(&req.upload_id);

    // Calculate total size by summing all chunks
    let mut total_size: i64 = 0;
    for i in 0..req.total_chunks {
        let chunk_path = upload_dir.join(format!("chunk_{:06}", i));
        if !chunk_path.exists() {
            tracing::error!("Missing chunk {} for upload {}", i, req.upload_id);
            return Err(StatusCode::BAD_REQUEST);
        }
        let metadata = fs::metadata(&chunk_path)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        total_size += metadata.len() as i64;
    }

    // SECURITY: Check quota before finalizing upload
    let has_quota = crate::api::quota::check_quota_available(&state.db_pool, &user.id, total_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !has_quota {
        tracing::warn!(
            "User {} exceeded quota. Required: {} bytes",
            user.id,
            total_size
        );
        // Clean up temp chunks on quota failure
        let _ = fs::remove_dir_all(&upload_dir).await;
        return Err(StatusCode::INSUFFICIENT_STORAGE);
    }

    // Verify all chunks exist
    for i in 0..req.total_chunks {
        let chunk_path = upload_dir.join(format!("chunk_{:06}", i));
        if !chunk_path.exists() {
            tracing::error!("Missing chunk {} for upload {}", i, req.upload_id);
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    // Determine final file path
    let base_dir = PathBuf::from("./data");
    let clean_path = req.path.trim_start_matches('/');
    let final_path = if clean_path.is_empty() {
        base_dir.join(&req.file_name)
    } else {
        base_dir.join(clean_path).join(&req.file_name)
    };

    // Create parent directories
    if let Some(parent) = final_path.parent() {
        fs::create_dir_all(parent)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Merge chunks into final file
    let mut final_file = fs::File::create(&final_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for i in 0..req.total_chunks {
        let chunk_path = upload_dir.join(format!("chunk_{:06}", i));
        let chunk_data = fs::read(&chunk_path)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        final_file
            .write_all(&chunk_data)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    final_file
        .flush()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Clean up temp chunks
    fs::remove_dir_all(&upload_dir).await.map_err(|e| {
        tracing::warn!("Failed to cleanup temp upload directory: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::info!(
        "Chunked upload finalized: {} ({} chunks merged)",
        final_path.display(),
        req.total_chunks
    );

    Ok(Json(ChunkFinalizeResponse {
        success: true,
        message: "Chunked upload complete".to_string(),
        file_path: final_path.to_string_lossy().to_string(),
    }))
}
