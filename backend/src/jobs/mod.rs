//! Background Job System
//! Handles asynchronous task execution with retry logic and scheduling

pub mod queue;
pub mod scheduler;
pub mod types;
pub mod worker;

// Re-exports for external use (via lib.rs)
pub use queue::JobQueue;
pub use scheduler::JobScheduler;
pub use types::{Job, JobResult, JobStatus, JobType};
pub use worker::JobWorker;
