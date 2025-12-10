//! API Router Module
//! Centralizes all API route definitions

pub mod activity;
pub mod admin;
pub mod api_tokens;
pub mod audit_compliance;
pub mod auth;
pub mod auth_security;
pub mod backup;
pub mod batch;
pub mod bulk_operations;
pub mod cleanup;
pub mod cloud_storage;
pub mod collaboration;
pub mod comments;
pub mod config;
pub mod cron;
pub mod dashboard;
pub mod database_health;
pub mod db_health;
pub mod directories;
pub mod duplicates;
pub mod encryption;
pub mod errors;
pub mod favorites;
pub mod file_comparison;
pub mod file_templates;
pub mod file_versions;
pub mod files;
pub mod folder_colors;
pub mod groups;
pub mod guest;
pub mod jobs;
pub mod metadata;
pub mod notifications;
pub mod peers;
pub mod performance;
pub mod quota;
pub mod rate_limiting;
pub mod rbac;
pub mod recent;
pub mod search;
pub mod setup;
pub mod sharing;
pub mod smart_folders;
pub mod storage_analytics;
pub mod system;
pub mod system_health;
pub mod tags;
pub mod trash;
pub mod upload_chunk;
pub mod users;
pub mod versions;
pub mod webhooks;
pub mod workflow;

// New API modules from POST_ALPHA_ROADMAP
pub mod oauth;
pub mod ldap;
// Temporarily disabled - need API fixes
// pub mod thumbnails;
// pub mod preview;
// pub mod virus_scan;
// pub mod ftp;
// pub mod email;
// pub mod webdav;

use axum::{middleware, Router};

use crate::AppState;

/// Build the complete API router
pub fn build_api_router(state: AppState) -> Router<AppState> {
    Router::new()
        // Public auth routes (login, register)
        .merge(auth::public_router())
        // Public OAuth routes (login via OAuth providers)
        .merge(oauth::public_router())
        // Setup routes (public - no auth required)
        .merge(setup::router())
        // Public sharing routes (NO AUTH - must come before protected routes)
        .merge(sharing::public_router())
        // Public guest access routes (NO AUTH - token-based access)
        .merge(guest::public_router())
        // Protected routes
        .merge(
            Router::new()
                .merge(auth::protected_router()) // Protected auth routes (2FA via TOTP, change-password, etc.)
                .merge(auth_security::router()) // Auth security (sessions, login attempts, password policy)
                .merge(users::router())
                .merge(groups::router())
                .merge(quota::router())
                // === FILE-SCOPED ROUTES (MUST come before generic catch-all routes) ===
                .merge(file_versions::file_versions_router()) // /files/{path}/versions/*
                .merge(tags::file_tags_router()) // /files/{path}/tags/*
                .merge(comments::file_comments_router()) // /files/{path}/comments/*
                // === GENERIC ROUTES (MORE SPECIFIC FIRST) ===
                .merge(file_versions::router())
                // 2FA functionality now integrated into auth::protected_router()
                .merge(versions::router()) // MUST come before files::router() (more specific routes first)
                .merge(file_comparison::router()) // /files/{path}/compare - must come before files::router()
                .merge(files::router()) // Has catch-all /files/{*path}, must be last for /files/*
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
                .merge(bulk_operations::router())
                .merge(config::router())
                .merge(peers::router())
                .merge(recent::router())
                .merge(duplicates::router())
                .merge(folder_colors::router())
                .merge(file_templates::router())
                .merge(rbac::router())
                .merge(workflow::router())
                .merge(cloud_storage::router())
                .merge(metadata::router()) // File metadata extraction (EXIF, ID3, PDF)
                .merge(audit_compliance::router()) // Audit logs & compliance reports
                .merge(dashboard::router()) // Admin dashboard & analytics
                .merge(errors::router()) // Error reporting endpoint
                .merge(jobs::router()) // Background jobs management
                .merge(cron::router()) // Cron scheduler management
                .merge(db_health::router()) // Database health and monitoring
                .merge(database_health::router()) // Advanced database health check
                .merge(upload_chunk::router()) // Chunked upload support
                .merge(webhooks::router()) // Webhook management
                .merge(system_health::router()) // System health monitoring
                .merge(api_tokens::router()) // Personal Access Token management
                .merge(cleanup::router()) // Auto-cleanup of deleted files
                .merge(smart_folders::router()) // Smart folders with dynamic rules
                .merge(storage_analytics::router()) // Storage analytics and statistics
                .merge(admin::router()) // Admin user management
                .merge(encryption::router()) // File encryption at rest
                .merge(guest::router()) // Guest/external user access
                .merge(rate_limiting::router()) // Rate limiting & quotas management
                // New routes from POST_ALPHA_ROADMAP
                // .merge(thumbnails::router()) // Thumbnail generation - disabled
                // .merge(preview::router()) // File preview generation - disabled
                // .merge(virus_scan::router()) // Virus scanning & quarantine - disabled
                .merge(oauth::protected_router()) // OAuth account linking
                .merge(ldap::router()) // LDAP configuration (admin)
                // .merge(ftp::router()) // FTP sync connections - disabled
                // .merge(email::router()) // Email integration - disabled
                // .merge(webdav::router()) // WebDAV protocol support - disabled
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    crate::middleware::auth::auth_middleware,
                )),
        )
        .with_state(state)
}
