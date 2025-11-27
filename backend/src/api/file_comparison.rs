//! File Comparison API
//! Compare file versions or different files

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

use crate::auth::UserInfo;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct CompareQuery {
    pub version_a: Option<String>,
    pub version_b: Option<String>,
    pub file_b: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ComparisonResponse {
    pub file_a: FileContent,
    pub file_b: FileContent,
    pub comparison_type: String,
}

#[derive(Debug, Serialize)]
pub struct FileContent {
    pub path: String,
    pub name: String,
    pub content: Option<String>,
    pub size_bytes: i64,
    pub mime_type: String,
    pub version: Option<String>,
    pub is_text: bool,
}

/// Compare two files or file versions
async fn compare_files(
    State(state): State<AppState>,
    user: UserInfo,
    Path(file_path): Path<String>,
    Query(query): Query<CompareQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    // Get file A (base file)
    let file_a = get_file_content(&state, &user, &file_path, query.version_a.as_deref()).await?;

    // Get file B (comparison target)
    let file_b = if let Some(ref file_b_path) = query.file_b {
        // Compare with different file
        get_file_content(&state, &user, file_b_path, None).await?
    } else if let Some(ref version_b) = query.version_b {
        // Compare with different version of same file
        get_file_content(&state, &user, &file_path, Some(version_b)).await?
    } else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let comparison_type = if query.file_b.is_some() {
        "different_files".to_string()
    } else {
        "versions".to_string()
    };

    Ok(Json(ComparisonResponse {
        file_a,
        file_b,
        comparison_type,
    }))
}

/// Get file content with optional version
async fn get_file_content(
    state: &AppState,
    user: &UserInfo,
    file_path: &str,
    version_id: Option<&str>,
) -> Result<FileContent, StatusCode> {
    let full_path = if let Some(version) = version_id {
        // Get file from version storage
        format!("./data/versions/{}/{}", file_path, version)
    } else {
        // Get current file
        format!("./data/{}", file_path)
    };

    // Check if file exists and get metadata
    let metadata = tokio::fs::metadata(&full_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let size_bytes = metadata.len() as i64;

    // Get mime type from database
    let mime_type: String =
        sqlx::query_scalar("SELECT mime_type FROM files WHERE file_path = ? LIMIT 1")
            .bind(file_path)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .unwrap_or_else(|| "application/octet-stream".to_string());

    // Determine if file is text-based
    let is_text = is_text_file(&mime_type);

    // Read content if it's a text file and not too large (max 1MB)
    let content = if is_text && size_bytes < 1_048_576 {
        match tokio::fs::read_to_string(&full_path).await {
            Ok(text) => Some(text),
            Err(_) => None,
        }
    } else {
        None
    };

    let filename = std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    Ok(FileContent {
        path: file_path.to_string(),
        name: filename,
        content,
        size_bytes,
        mime_type,
        version: version_id.map(|v| v.to_string()),
        is_text,
    })
}

/// Check if mime type indicates text file
fn is_text_file(mime_type: &str) -> bool {
    mime_type.starts_with("text/")
        || matches!(
            mime_type,
            "application/json"
                | "application/javascript"
                | "application/xml"
                | "application/x-yaml"
                | "application/x-sh"
                | "application/x-python"
        )
}

pub fn router() -> Router<AppState> {
    Router::new().route("/compare/{*path}", get(compare_files))
}
