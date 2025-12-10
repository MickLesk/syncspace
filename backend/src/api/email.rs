//! Email Integration API endpoints
//! Manages email accounts and message fetching

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;

use crate::{
    auth::UserInfo,
    email_integration::{
        self, CreateEmailAccountRequest, EmailAccount, EmailMessage, FetchResult,
        UpdateEmailAccountRequest,
    },
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_limit")]
    pub limit: i32,
    #[serde(default)]
    pub offset: i32,
}

fn default_limit() -> i32 {
    50
}

/// List all email accounts for user
async fn list_accounts(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<EmailAccount>>, StatusCode> {
    email_integration::list_email_accounts(&state.pool, &user.user_id)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to list email accounts: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Get single email account
async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<String>,
    user: UserInfo,
) -> Result<Json<EmailAccount>, StatusCode> {
    email_integration::get_email_account(&state.pool, &id, &user.user_id)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to get email account: {}", e);
            StatusCode::NOT_FOUND
        })
}

/// Create new email account
async fn create_account(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateEmailAccountRequest>,
) -> Result<(StatusCode, Json<EmailAccount>), StatusCode> {
    email_integration::create_email_account(&state.pool, &user.user_id, req)
        .await
        .map(|account| (StatusCode::CREATED, Json(account)))
        .map_err(|e| {
            tracing::error!("Failed to create email account: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Update email account
async fn update_account(
    State(state): State<AppState>,
    Path(id): Path<String>,
    user: UserInfo,
    Json(req): Json<UpdateEmailAccountRequest>,
) -> Result<Json<EmailAccount>, StatusCode> {
    email_integration::update_email_account(&state.pool, &id, &user.user_id, req)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to update email account: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Delete email account
async fn delete_account(
    State(state): State<AppState>,
    Path(id): Path<String>,
    user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    email_integration::delete_email_account(&state.pool, &id, &user.user_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| {
            tracing::error!("Failed to delete email account: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Test email connection
async fn test_connection(
    State(state): State<AppState>,
    Path(id): Path<String>,
    user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let account = email_integration::get_email_account(&state.pool, &id, &user.user_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    match email_integration::test_email_connection(&account).await {
        Ok(true) => Ok(Json(serde_json::json!({
            "success": true,
            "message": "Connection test passed"
        }))),
        Ok(false) => Ok(Json(serde_json::json!({
            "success": false,
            "message": "Connection test failed"
        }))),
        Err(e) => Ok(Json(serde_json::json!({
            "success": false,
            "message": format!("Connection error: {}", e)
        }))),
    }
}

/// Fetch emails from account
async fn fetch_emails(
    State(state): State<AppState>,
    Path(id): Path<String>,
    user: UserInfo,
) -> Result<Json<FetchResult>, StatusCode> {
    let account = email_integration::get_email_account(&state.pool, &id, &user.user_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    email_integration::fetch_emails(&state.pool, &account)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to fetch emails: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Get messages for account
async fn get_messages(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(pagination): Query<PaginationQuery>,
    user: UserInfo,
) -> Result<Json<Vec<EmailMessage>>, StatusCode> {
    // Verify account belongs to user
    let _ = email_integration::get_email_account(&state.pool, &id, &user.user_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    email_integration::get_messages(&state.pool, &id, pagination.limit, pagination.offset)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to get messages: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Mark message as read
async fn mark_read(
    State(state): State<AppState>,
    Path(message_id): Path<String>,
    _user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    email_integration::mark_as_read(&state.pool, &message_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| {
            tracing::error!("Failed to mark message as read: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Delete message
async fn delete_message(
    State(state): State<AppState>,
    Path(message_id): Path<String>,
    _user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    email_integration::delete_message(&state.pool, &message_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| {
            tracing::error!("Failed to delete message: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/email/accounts", get(list_accounts).post(create_account))
        .route(
            "/email/accounts/{id}",
            get(get_account).put(update_account).delete(delete_account),
        )
        .route("/email/accounts/{id}/test", post(test_connection))
        .route("/email/accounts/{id}/fetch", post(fetch_emails))
        .route("/email/accounts/{id}/messages", get(get_messages))
        .route("/email/messages/{message_id}/read", post(mark_read))
        .route("/email/messages/{message_id}", delete(delete_message))
}
