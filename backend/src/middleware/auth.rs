//! Authentication middleware

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::{auth::{User, UserInfo}, AppState};

/// Auth middleware - validates JWT and extracts user info
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Validate Bearer token
    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    // Decode and validate JWT against SQLite database
    let user_info: UserInfo = crate::auth::validate_token_against_db(&state.db_pool, token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Insert user info into request extensions
    req.extensions_mut().insert(User(user_info));

    Ok(next.run(req).await)
}
