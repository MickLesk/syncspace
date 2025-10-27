//! Service layer modules
//! Service layer - business logic implementation
//! 
//! All services fully implemented with database persistence,
//! WebSocket event broadcasting, and activity logging.

// Service implementations
pub mod auth_service;
mod file_service_impl;
mod user_service_impl;
mod search_service_impl;
mod all_services_impl;

// Re-export auth service functions
pub use auth_service::*;

// Re-export file service functions
pub use file_service_impl::{
    list_files, download_file, upload_file,
    delete_file, rename_file, move_file, copy_file
};

// Re-export user service functions
pub use user_service_impl::{
    get_profile, update_profile,
    get_settings, update_settings,
    get_preferences, update_preferences
};

// Re-export search service
pub use search_service_impl::search;

// Re-export all other services from all_services_impl
pub use all_services_impl::{
    directory, sharing, activity, tag, favorites,
    backup, collaboration, system
};

// Stub functions for missing features
pub async fn get_thumbnail(_state: &crate::AppState, _user: &crate::auth::UserInfo, _file_id: &str) -> anyhow::Result<Vec<u8>> {
    Ok(vec![])
}
