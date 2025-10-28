//! Notifications API endpoints

use axum::{routing::{delete, get, post}, Router};
use crate::{handlers, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/notifications", get(handlers::notification_handlers::get_notifications))
        .route("/notifications", delete(handlers::notification_handlers::delete_all_notifications))
        .route("/notifications/:id/read", post(handlers::notification_handlers::mark_notification_read))
        .route("/notifications/read-all", post(handlers::notification_handlers::mark_all_notifications_read))
        .route("/notifications/:id", delete(handlers::notification_handlers::delete_notification))
}
