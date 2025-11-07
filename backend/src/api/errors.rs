//! Frontend error reporting endpoint
//! Receives and logs client-side errors for centralized monitoring

use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::AppState;

// ==================== REQUEST/RESPONSE TYPES ====================

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorReport {
    pub message: String,
    pub stack: Option<String>,
    #[serde(rename = "type")]
    pub error_type: String,
    pub url: String,
    pub user_agent: String,
    pub timestamp: String,
    pub context: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct BatchErrorReport {
    pub errors: Vec<ErrorReport>,
}

// ==================== ROUTER ====================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/errors/report", post(report_errors_handler))
}

// ==================== HANDLERS ====================

/// Report client-side errors
#[tracing::instrument(skip(state, payload), fields(error_count = payload.errors.len()))]
async fn report_errors_handler(
    State(state): State<AppState>,
    Json(payload): Json<BatchErrorReport>,
) -> Result<StatusCode, StatusCode> {
    tracing::info!("Received {} client-side error reports", payload.errors.len());

    // Log each error with full context
    for error in &payload.errors {
        let severity = error.context.get("severity")
            .and_then(|v| v.as_str())
            .unwrap_or("error");

        match severity {
            "warning" => {
                tracing::warn!(
                    message = %error.message,
                    error_type = %error.error_type,
                    url = %error.url,
                    timestamp = %error.timestamp,
                    stack = ?error.stack,
                    context = ?error.context,
                    "Frontend warning"
                );
            }
            "performance" => {
                tracing::info!(
                    message = %error.message,
                    url = %error.url,
                    timestamp = %error.timestamp,
                    context = ?error.context,
                    "Frontend performance issue"
                );
            }
            _ => {
                tracing::error!(
                    message = %error.message,
                    error_type = %error.error_type,
                    url = %error.url,
                    user_agent = %error.user_agent,
                    timestamp = %error.timestamp,
                    stack = ?error.stack,
                    context = ?error.context,
                    "Frontend error"
                );
            }
        }

        // TODO: Store in database for error analytics
        // TODO: Send alerts for critical errors
        // TODO: Aggregate similar errors
    }

    // Store errors in database for later analysis
    let result = store_errors_in_db(&state, &payload.errors).await;
    
    match result {
        Ok(_) => {
            tracing::debug!("Errors stored in database successfully");
            Ok(StatusCode::OK)
        }
        Err(e) => {
            tracing::error!("Failed to store errors in database: {}", e);
            // Still return OK to client - don't fail their app because of logging issues
            Ok(StatusCode::OK)
        }
    }
}

/// Store errors in database for analytics
async fn store_errors_in_db(
    state: &AppState,
    errors: &[ErrorReport],
) -> Result<(), sqlx::Error> {
    // Create errors table if not exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS frontend_errors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            message TEXT NOT NULL,
            stack TEXT,
            error_type TEXT NOT NULL,
            url TEXT NOT NULL,
            user_agent TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            context TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(&state.db_pool)
    .await?;

    // Insert each error
    for error in errors {
        sqlx::query(
            r#"
            INSERT INTO frontend_errors 
            (message, stack, error_type, url, user_agent, timestamp, context)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&error.message)
        .bind(&error.stack)
        .bind(&error.error_type)
        .bind(&error.url)
        .bind(&error.user_agent)
        .bind(&error.timestamp)
        .bind(error.context.to_string())
        .execute(&state.db_pool)
        .await?;
    }

    Ok(())
}
