//! API Router Module
//! Centralizes all API route definitions

pub mod auth;
pub mod auth_security;
pub mod setup;
// pub mod twofa; // TODO: Fix compilation errors
pub mod groups;
pub mod quota;
pub mod file_versions;
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
pub mod folder_colors;
pub mod errors;
pub mod jobs;
pub mod cron;
pub mod db_health;
pub mod database_health;
pub mod upload_chunk;

use axum::{
    middleware,
    Router,
};

use crate::AppState;

/// Build the complete API router
pub fn build_api_router(state: AppState) -> Router<AppState> {
    Router::new()
        // Public auth routes (login, register)
        .merge(auth::public_router())
        
        // Setup routes (public - no auth required)
        .merge(setup::router())
        
        // Protected routes
        .merge(
            Router::new()
                .merge(auth::protected_router())  // Protected auth routes (2FA, change-password, etc.)
                .merge(auth_security::router())   // Auth security (sessions, login attempts, password policy)
                .merge(users::router())
                .merge(groups::router())
                .merge(quota::router())
                .merge(file_versions::router())
                // .merge(twofa::router()) // TODO: Fix compilation
                .merge(versions::router())  // MUST come before files::router() (more specific routes first)
                .merge(files::router())     // Has catch-all /files/{*path}, must be last for /files/*
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
                .merge(batch::router())
                .merge(config::router())
                .merge(peers::router())
                .merge(recent::router())
                .merge(duplicates::router())
                .merge(folder_colors::router())
                .merge(errors::router())  // Error reporting endpoint
                .merge(jobs::router())    // Background jobs management
                .merge(cron::router())    // Cron scheduler management
                .merge(db_health::router()) // Database health and monitoring
                .merge(database_health::router()) // Advanced database health check
                .merge(upload_chunk::router()) // Chunked upload support
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    crate::middleware::auth::auth_middleware,
                )),
        )
        .with_state(state)
}
