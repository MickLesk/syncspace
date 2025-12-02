#![allow(dead_code)]

//! Comments API Routes

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{self, get, post, put},
    Router,
};
use chrono::Utc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::auth::UserInfo;
use crate::AppState;

/// Extract @mentions from comment text
/// Matches @username patterns and returns list of usernames
fn extract_mentions(text: &str) -> Vec<String> {
    let re = Regex::new(r"@([a-zA-Z0-9_]+)").unwrap();
    re.captures_iter(text)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

/// Send notifications to mentioned users
async fn send_mention_notifications(
    pool: &sqlx::SqlitePool,
    mentions: &[String],
    comment_author_id: &str,
    comment_author_name: &str,
    file_path: &str,
) {
    for username in mentions {
        // Look up user_id by username
        let user_result: Result<Option<(String,)>, _> =
            sqlx::query_as("SELECT id FROM users WHERE username = ? AND id != ?")
                .bind(username)
                .bind(comment_author_id)
                .fetch_optional(pool)
                .await;

        if let Ok(Some((user_id,))) = user_result {
            let notification_id = Uuid::new_v4().to_string();
            let now = Utc::now().to_rfc3339();
            let title = format!("@{} mentioned you", comment_author_name);
            let message = format!("You were mentioned in a comment on {}", file_path);

            let _ = sqlx::query(
                r#"INSERT INTO notifications (id, user_id, type, title, message, read_status, created_at)
                   VALUES (?, ?, 'mention', ?, ?, 0, ?)"#
            )
            .bind(&notification_id)
            .bind(&user_id)
            .bind(&title)
            .bind(&message)
            .bind(&now)
            .execute(pool)
            .await;
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FilePathQuery {
    pub path: String,
}

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
    .execute(&state.db_pool)
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
    .fetch_all(&state.db_pool)
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
    .execute(&state.db_pool)
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
    .execute(&state.db_pool)
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

/// File-scoped comments routes using query parameters
/// Frontend should call: /api/file-comments/list?path={path}
pub fn file_comments_router() -> Router<AppState> {
    Router::new()
        .route("/api/file-comments/list", get(list_file_comments))
        .route("/api/file-comments/create", post(create_file_comment))
        .route(
            "/api/file-comments/update/{comment_id}",
            put(update_file_comment),
        )
        .route(
            "/api/file-comments/delete/{comment_id}",
            routing::delete(delete_file_comment),
        )
        .route(
            "/api/file-comments/reaction/{comment_id}",
            post(add_reaction),
        )
}

// ============================================================================
// FILE-SCOPED COMMENT ENDPOINTS with Query Parameters
// ============================================================================

/// GET /api/file-comments/list?path={path} - List all comments for a file
async fn list_file_comments(
    State(state): State<AppState>,
    Query(query): Query<FilePathQuery>,
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
    .bind(&query.path)
    .fetch_all(&state.db_pool)
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

/// POST /api/file-comments/create?path={path} - Create comment for a file
async fn create_file_comment(
    State(state): State<AppState>,
    Query(query): Query<FilePathQuery>,
    user_info: UserInfo,
    Json(req): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let sanitized_content = crate::security::sanitize_html(&req.content);

    // Extract @mentions from comment
    let mentions = extract_mentions(&req.content);
    let mentions_json = serde_json::to_string(&mentions).unwrap_or_else(|_| "[]".to_string());

    let comment_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO comments (
            id, item_type, item_id, file_path, author_id, 
            text, mentions, is_resolved, created_at, updated_at
        )
        VALUES (?, 'file', '', ?, ?, ?, ?, 0, ?, ?)
        "#,
    )
    .bind(&comment_id)
    .bind(&query.path)
    .bind(&user_info.id)
    .bind(&sanitized_content)
    .bind(&mentions_json)
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Send notifications to mentioned users
    if !mentions.is_empty() {
        send_mention_notifications(
            &state.db_pool,
            &mentions,
            &user_info.id,
            &user_info.username,
            &query.path,
        )
        .await;
    }

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": comment_id,
            "text": sanitized_content,
            "author_id": user_info.id,
            "created_at": now,
            "updated_at": now,
            "mentions": mentions,
        })),
    ))
}

/// PUT /api/file-comments/update/{comment_id}?path={path} - Update comment (edit)
async fn update_file_comment(
    State(state): State<AppState>,
    Path(comment_id): Path<String>,
    Query(_query): Query<FilePathQuery>,
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
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "id": comment_id,
        "text": sanitized_content,
        "updated_at": now,
    })))
}

/// DELETE /api/file-comments/delete/{comment_id}?path={path} - Delete comment
async fn delete_file_comment(
    State(state): State<AppState>,
    Path(comment_id): Path<String>,
    Query(_query): Query<FilePathQuery>,
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
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/file-comments/reaction/{comment_id}?path={path} - Add emoji reaction
async fn add_reaction(
    State(state): State<AppState>,
    Path(comment_id): Path<String>,
    Query(_query): Query<FilePathQuery>,
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
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

#[derive(Debug, Deserialize)]
pub struct ReactionRequest {
    pub emoji: String,
}
