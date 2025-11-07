//! Job scheduler for recurring tasks

use super::queue::JobQueue;
use super::types::{Job, JobType};
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub struct JobScheduler {
    queue: Arc<JobQueue>,
    shutdown: Arc<tokio::sync::Notify>,
}

impl JobScheduler {
    pub fn new(pool: Arc<SqlitePool>, shutdown: Arc<tokio::sync::Notify>) -> Self {
        let queue = Arc::new(JobQueue::new(pool));
        Self { queue, shutdown }
    }

    /// Start scheduler loop (checks every 60 seconds)
    pub async fn start(self) {
        println!("⏰ Job scheduler started");
        
        loop {
            tokio::select! {
                _ = self.shutdown.notified() => {
                    println!("🛑 Job scheduler shutting down");
                    break;
                }
                _ = sleep(Duration::from_secs(60)) => {
                    if let Err(e) = self.schedule_recurring_jobs().await {
                        eprintln!("❌ Scheduler error: {}", e);
                    }
                }
            }
        }
    }

    async fn schedule_recurring_jobs(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let now = chrono::Utc::now();
        let hour = now.hour();
        let minute = now.minute();
        let day_of_week = now.weekday().num_days_from_monday();
        
        // Daily jobs at 02:00
        if hour == 2 && minute == 0 {
            self.schedule_daily_jobs().await?;
        }
        
        // Weekly jobs on Sunday at 03:00
        if day_of_week == 6 && hour == 3 && minute == 0 {
            self.schedule_weekly_jobs().await?;
        }
        
        // Hourly jobs at :00
        if minute == 0 {
            self.schedule_hourly_jobs().await?;
        }
        
        Ok(())
    }

    async fn schedule_daily_jobs(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("📅 Scheduling daily jobs...");
        
        // Database cleanup (jobs older than 30 days)
        let job = Job::new(
            JobType::DatabaseCleanup { table: "jobs".to_string() },
            None
        )?;
        self.queue.enqueue(job).await?;
        
        // Login attempts cleanup
        let job = Job::new(
            JobType::DatabaseCleanup { table: "login_attempts".to_string() },
            None
        )?;
        self.queue.enqueue(job).await?;
        
        // Version cleanup (files older than 90 days)
        let job = Job::new(
            JobType::VersionCleanup { file_id: None },
            None
        )?;
        self.queue.enqueue(job).await?;
        
        println!("✅ Daily jobs scheduled");
        Ok(())
    }

    async fn schedule_weekly_jobs(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("📅 Scheduling weekly jobs...");
        
        // Full search index rebuild
        let job = Job::new(
            JobType::SearchIndexRebuild { full_rebuild: true },
            None
        )?;
        self.queue.enqueue(job).await?;
        
        println!("✅ Weekly jobs scheduled");
        Ok(())
    }

    async fn schedule_hourly_jobs(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Could add hourly tasks here if needed
        Ok(())
    }
}

/// Start job scheduler
pub async fn start_job_scheduler(pool: Arc<SqlitePool>) -> Arc<tokio::sync::Notify> {
    let shutdown = Arc::new(tokio::sync::Notify::new());
    let scheduler = JobScheduler::new(pool, shutdown.clone());
    
    tokio::spawn(async move {
        scheduler.start().await;
    });
    
    shutdown
}
