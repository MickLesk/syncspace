use crate::auth::UserInfo;
//! API Router Module
//! Centralizes all API route definitions

pub mod auth;
pub mod files;
pub mod directories;
pub mod users;
pub mod search;
pub mod sharing;
pub mod activity;
pub mod tags;
pub mod favorites;
pub mod backup;
pub mod collaboration;
pub mod system;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};

use crate::AppState;

/// Build the complete API router
pub fn build_api_router(state: AppState) -> Router<AppState> {
    Router::new()
        // Health check
        .route("/health", get(|| async { "OK" }))
        
        // Auth routes (public)
        .merge(auth::router())
        
        // Protected routes
        .merge(
            Router::new()
                .merge(users::router())
                .merge(files::router())
                .merge(directories::router())
                .merge(search::router())
                .merge(sharing::router())
                .merge(activity::router())
                .merge(tags::router())
                .merge(favorites::router())
                .merge(backup::router())
                .merge(collaboration::router())
                .merge(system::router())
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    crate::middleware::auth::auth_middleware,
                )),
        )
        .with_state(state)
}
