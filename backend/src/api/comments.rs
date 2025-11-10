//! Comments API Routes

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{self, get, post, put},
    Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::auth::UserInfo;
use crate::AppState;

/// Comment model (matches 002_add_comments_tags.sql schema)
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: String,
    pub item_type: String,
    pub item_id: String,
    pub file_path: String,
    pub author_id: String,
    pub text: String,
    pub is_resolved: bool,
    pub resolved_at: Option<String>,
    pub resolved_by: Option<String>,
    pub edited_at: Option<String>,
    pub edited_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Comment response model (simplified for API)
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentResponse {
    pub id: String,
    pub file_path: String,
    pub author_id: String,
    pub text: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_resolved: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub file_path: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ListCommentsQuery {
    pub file_path: String,
}

/// Create a new comment
async fn create_comment(
    State(state): State<AppState>,
    user_info: UserInfo,
    Json(req): Json<CreateCommentRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // SECURITY: Sanitize HTML content to prevent XSS attacks
    let sanitized_content = crate::security::sanitize_html(&req.content);

    let comment_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO comments (
            id, item_type, item_id, file_path, author_id, 
            text, is_resolved, created_at, updated_at
        )
        VALUES (?, 'file', '', ?, ?, ?, 0, ?, ?)
        "#,
    )
    .bind(&comment_id)
    .bind(&req.file_path)
    .bind(&user_info.id)
    .bind(&sanitized_content)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Failed to create comment: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": comment_id,
            "message": "Comment created successfully"
        })),
    ))
}

/// List comments for a file
async fn list_comments(
    State(state): State<AppState>,
    Query(query): Query<ListCommentsQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let comments = sqlx::query_as::<_, Comment>(
        r#"
        SELECT id, item_type, item_id, file_path, author_id, text,
               is_resolved, resolved_at, resolved_by, edited_at, edited_by,
               created_at, updated_at
        FROM comments
        WHERE file_path = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(&query.file_path)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Failed to list comments: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Convert to simplified response format
    let responses: Vec<CommentResponse> = comments
        .into_iter()
        .map(|c| CommentResponse {
            id: c.id,
            file_path: c.file_path,
            author_id: c.author_id,
            text: c.text,
            created_at: c.created_at,
            updated_at: c.updated_at,
            is_resolved: c.is_resolved,
        })
        .collect();

    Ok(Json(responses))
}

/// Update a comment (for editing)
async fn update_comment(
    State(state): State<AppState>,
    Path(comment_id): Path<String>,
    user_info: UserInfo,
    Json(req): Json<CreateCommentRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let sanitized_content = crate::security::sanitize_html(&req.content);
    let now = Utc::now().to_rfc3339();

    let result = sqlx::query(
        r#"
        UPDATE comments
        SET text = ?, edited_at = ?, edited_by = ?, updated_at = ?
        WHERE id = ? AND author_id = ?
        "#,
    )
    .bind(&sanitized_content)
    .bind(&now)
    .bind(&user_info.id)
    .bind(&now)
    .bind(&comment_id)
    .bind(&user_info.id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Failed to update comment: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(serde_json::json!({
        "message": "Comment updated successfully"
    })))
}

/// Delete a comment
async fn delete_comment(
    State(state): State<AppState>,
    Path(comment_id): Path<String>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query(
        r#"
        DELETE FROM comments
        WHERE id = ? AND author_id = ?
        "#,
    )
    .bind(&comment_id)
    .bind(&user_info.id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Failed to delete comment: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Build comments router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/comments", post(create_comment).get(list_comments))
        .route(
            "/comments/{comment_id}",
            routing::put(update_comment).delete(routing::delete(delete_comment)),
        )
}

/// File-scoped comments routes: /files/{path}/comments/*
/// This is the primary interface for frontend
pub fn file_comments_router() -> Router<AppState> {
    Router::new()
        .route(
            "/files/*path/comments",
            get(list_file_comments).post(create_file_comment),
        )
        .route(
            "/files/*path/comments/:comment_id",
            put(update_file_comment).delete(delete_file_comment),
        )
        .route(
            "/files/*path/comments/:comment_id/reactions",
            post(add_reaction),
        )
}

// ============================================================================
// FILE-SCOPED COMMENT ENDPOINTS: /files/{path}/comments/*
// ============================================================================

/// GET /files/{path}/comments - List all comments for a file
async fn list_file_comments(
    State(state): State<AppState>,
    Path(path): Path<String>,
    _user: UserInfo,
) -> Result<Json<Vec<CommentResponse>>, StatusCode> {
    let comments = sqlx::query_as::<_, Comment>(
        r#"
        SELECT id, item_type, item_id, file_path, author_id, text,
               is_resolved, resolved_at, resolved_by, edited_at, edited_by,
               created_at, updated_at
        FROM comments
        WHERE file_path = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(&path)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let responses: Vec<CommentResponse> = comments
        .into_iter()
        .map(|c| CommentResponse {
            id: c.id,
            file_path: c.file_path,
            author_id: c.author_id,
            text: c.text,
            created_at: c.created_at,
            updated_at: c.updated_at,
            is_resolved: c.is_resolved,
        })
        .collect();

    Ok(Json(responses))
}

/// POST /files/{path}/comments - Create comment for a file
async fn create_file_comment(
    State(state): State<AppState>,
    Path(path): Path<String>,
    user_info: UserInfo,
    Json(req): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let sanitized_content = crate::security::sanitize_html(&req.content);
    let comment_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO comments (
            id, item_type, item_id, file_path, author_id, 
            text, is_resolved, created_at, updated_at
        )
        VALUES (?, 'file', '', ?, ?, ?, 0, ?, ?)
        "#,
    )
    .bind(&comment_id)
    .bind(&path)
    .bind(&user_info.id)
    .bind(&sanitized_content)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": comment_id,
            "text": sanitized_content,
            "author_id": user_info.id,
            "created_at": now,
            "updated_at": now,
        })),
    ))
}

/// PUT /files/{path}/comments/{comment_id} - Update comment (edit)
async fn update_file_comment(
    State(state): State<AppState>,
    Path((path, comment_id)): Path<(String, String)>,
    user_info: UserInfo,
    Json(req): Json<CreateCommentRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let sanitized_content = crate::security::sanitize_html(&req.content);
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        UPDATE comments
        SET text = ?, edited_at = ?, edited_by = ?, updated_at = ?
        WHERE id = ? AND author_id = ?
        "#,
    )
    .bind(&sanitized_content)
    .bind(&now)
    .bind(&user_info.id)
    .bind(&now)
    .bind(&comment_id)
    .bind(&user_info.id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "id": comment_id,
        "text": sanitized_content,
        "updated_at": now,
    })))
}

/// DELETE /files/{path}/comments/{comment_id} - Delete comment
async fn delete_file_comment(
    State(state): State<AppState>,
    Path((path, comment_id)): Path<(String, String)>,
    user_info: UserInfo,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        r#"
        DELETE FROM comments
        WHERE id = ? AND author_id = ?
        "#,
    )
    .bind(&comment_id)
    .bind(&user_info.id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

/// POST /files/{path}/comments/{comment_id}/reactions - Add emoji reaction
async fn add_reaction(
    State(state): State<AppState>,
    Path((path, comment_id)): Path<(String, String)>,
    user_info: UserInfo,
    Json(req): Json<ReactionRequest>,
) -> Result<StatusCode, StatusCode> {
    let reaction_id = Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT OR IGNORE INTO comment_reactions (id, comment_id, emoji, user_id)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(&reaction_id)
    .bind(&comment_id)
    .bind(&req.emoji)
    .bind(&user_info.id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

#[derive(Debug, Deserialize)]
pub struct ReactionRequest {
    pub emoji: String,
}
