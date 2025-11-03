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
pub mod performance;
pub mod notifications;
pub mod comments;
pub mod trash;
pub mod versions;
pub mod batch;
pub mod config;
pub mod peers;
pub mod recent;
pub mod duplicates;

use axum::{
    middleware,
    Router,
};

use crate::AppState;

/// Build the complete API router
pub fn build_api_router(state: AppState) -> Router<AppState> {
    Router::new()
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
                .merge(performance::router())
                .merge(notifications::router())
                .merge(comments::router())
                .merge(trash::router())
                .merge(versions::router())
                .merge(batch::router())
                .merge(config::router())
                .merge(peers::router())
                .merge(recent::router())
                .merge(duplicates::router())
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    crate::middleware::auth::auth_middleware,
                )),
        )
        .with_state(state)
}
