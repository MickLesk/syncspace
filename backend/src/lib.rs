//! SyncSpace Backend Library
//! 
//! This library exposes core functionality for integration tests.

pub mod auth;
pub mod cron;
pub mod database;
pub mod jobs;
pub mod workers;
pub mod websocket;

// Re-export commonly used types
pub use jobs::{JobType, JobStatus, BackgroundJob};
pub use websocket::FileChangeEvent;
