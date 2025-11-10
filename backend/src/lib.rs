//! SyncSpace Backend Library
//!
//! This library exposes core functionality for integration tests.

pub mod auth;
pub mod cron;
pub mod database;
pub mod jobs;
pub mod websocket;
pub mod workers;

// Re-export commonly used types
pub use jobs::types::{Job, JobResult, JobStatus, JobType};
pub use websocket::FileChangeEvent;
