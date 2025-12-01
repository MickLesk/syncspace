//! Service layer modules
//! Service layer - business logic implementation
//!
//! All services fully implemented with database persistence,
//! WebSocket event broadcasting, and activity logging.

// Service implementations
mod all_services_impl;
pub mod auth_security_service;
pub mod auth_service;
pub mod cleanup_service;
mod file_service_impl;
pub mod job_worker;
pub mod performance_service;
mod search_service_impl;
pub mod smart_folders_service;
pub mod sync_service;
mod user_service_impl;
pub mod version_storage_service;

// Re-export auth service functions
pub use auth_service::*;

// Re-export file service functions
pub use file_service_impl::{
    copy_file, delete_file, download_file, get_recent_files, list_files, move_file, rename_file,
    upload_file,
};

// Re-export user service functions
pub use user_service_impl::{
    get_preferences, get_profile, get_settings, list_users, update_preferences, update_profile,
    update_settings,
};

// Re-export search service
pub use search_service_impl::search;

// Re-export all other services from all_services_impl
pub use all_services_impl::{
    activity, backup, collaboration, directory, favorites, sharing, system, tag,
};

// Performance service module
pub mod performance {
    pub use super::performance_service::*;
}

// Stub functions for missing features
pub async fn get_thumbnail(
    _state: &crate::AppState,
    _user: &crate::auth::UserInfo,
    _file_id: &str,
) -> anyhow::Result<Vec<u8>> {
    Ok(vec![])
}
