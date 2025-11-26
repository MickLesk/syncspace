//! Job queue management with SQLite backend
//! Part of background job processing system - some features deferred pending refactoring
#![allow(dead_code)]

use super::types::{Job, JobStatus};
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct JobQueue {
    pool: Arc<SqlitePool>,
}

impl JobQueue {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    /// Enqueue a new job
    pub async fn enqueue(&self, job: Job) -> Result<Job, sqlx::Error> {
        sqlx::query_as::<_, Job>(
            r#"
            INSERT INTO jobs (id, job_type, status, payload, result, error, attempts, max_attempts, 
                            created_at, started_at, completed_at, scheduled_for, created_by)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(&job.id)
        .bind(&job.job_type)
        .bind(&job.status)
        .bind(&job.payload)
        .bind(&job.result)
        .bind(&job.error)
        .bind(job.attempts)
        .bind(job.max_attempts)
        .bind(&job.created_at)
        .bind(&job.started_at)
        .bind(&job.completed_at)
        .bind(&job.scheduled_for)
        .bind(&job.created_by)
        .fetch_one(self.pool.as_ref())
        .await
    }

    /// Get next pending job that is due
    pub async fn dequeue(&self) -> Result<Option<Job>, sqlx::Error> {
        let now = chrono::Utc::now().to_rfc3339();
        
        sqlx::query_as::<_, Job>(
            r#"
            SELECT * FROM jobs 
            WHERE status = 'pending' 
            AND (scheduled_for IS NULL OR scheduled_for <= ?)
            ORDER BY created_at ASC 
            LIMIT 1
            "#
        )
        .bind(&now)
        .fetch_optional(self.pool.as_ref())
        .await
    }

    /// Mark job as running
    pub async fn mark_running(&self, job_id: &str) -> Result<(), sqlx::Error> {
        let now = chrono::Utc::now().to_rfc3339();
        
        sqlx::query(
            r#"
            UPDATE jobs 
            SET status = 'running', started_at = ?, attempts = attempts + 1
            WHERE id = ?
            "#
        )
        .bind(&now)
        .bind(job_id)
        .execute(self.pool.as_ref())
        .await?;
        
        Ok(())
    }

    /// Mark job as completed successfully
    pub async fn mark_success(&self, job_id: &str, result: &str) -> Result<(), sqlx::Error> {
        let now = chrono::Utc::now().to_rfc3339();
        
        sqlx::query(
            r#"
            UPDATE jobs 
            SET status = 'success', result = ?, completed_at = ?
            WHERE id = ?
            "#
        )
        .bind(result)
        .bind(&now)
        .bind(job_id)
        .execute(self.pool.as_ref())
        .await?;
        
        Ok(())
    }

    /// Mark job as failed (retry if possible)
    pub async fn mark_failed(&self, job_id: &str, error: &str) -> Result<(), sqlx::Error> {
        let job = self.get_job(job_id).await?;
        
        if let Some(job) = job {
            if job.can_retry() {
                // Retry with exponential backoff
                let delay_seconds = 2_i64.pow(job.attempts as u32) * 60;  // 1min, 2min, 4min, ...
                let scheduled_for = chrono::Utc::now() + chrono::Duration::seconds(delay_seconds);
                
                sqlx::query(
                    r#"
                    UPDATE jobs 
                    SET status = 'pending', error = ?, scheduled_for = ?
                    WHERE id = ?
                    "#
                )
                .bind(error)
                .bind(scheduled_for.to_rfc3339())
                .bind(job_id)
                .execute(self.pool.as_ref())
                .await?;
                
                println!("🔄 Job {} will retry in {} seconds (attempt {}/{})", 
                         job_id, delay_seconds, job.attempts + 1, job.max_attempts);
            } else {
                // Max attempts reached
                let now = chrono::Utc::now().to_rfc3339();
                
                sqlx::query(
                    r#"
                    UPDATE jobs 
                    SET status = 'failed', error = ?, completed_at = ?
                    WHERE id = ?
                    "#
                )
                .bind(error)
                .bind(&now)
                .bind(job_id)
                .execute(self.pool.as_ref())
                .await?;
                
                println!("❌ Job {} failed permanently after {} attempts", job_id, job.attempts);
            }
        }
        
        Ok(())
    }

    /// Get job by ID
    pub async fn get_job(&self, job_id: &str) -> Result<Option<Job>, sqlx::Error> {
        sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE id = ?")
            .bind(job_id)
            .fetch_optional(self.pool.as_ref())
            .await
    }

    /// List jobs with filters
    pub async fn list_jobs(
        &self,
        status: Option<JobStatus>,
        job_type: Option<&str>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Job>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM jobs WHERE 1=1");
        
        if let Some(s) = status {
            query.push_str(&format!(" AND status = '{}'", s));
        }
        
        if let Some(t) = job_type {
            query.push_str(&format!(" AND job_type = '{}'", t));
        }
        
        query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");
        
        sqlx::query_as::<_, Job>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(self.pool.as_ref())
            .await
    }

    /// Count jobs by status
    pub async fn count_jobs(&self, status: Option<JobStatus>) -> Result<i64, sqlx::Error> {
        let query = if let Some(s) = status {
            format!("SELECT COUNT(*) FROM jobs WHERE status = '{}'", s)
        } else {
            "SELECT COUNT(*) FROM jobs".to_string()
        };
        
        sqlx::query_scalar(&query)
            .fetch_one(self.pool.as_ref())
            .await
    }

    /// Cancel a pending job
    pub async fn cancel_job(&self, job_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE jobs 
            SET status = 'cancelled', completed_at = ?
            WHERE id = ? AND status = 'pending'
            "#
        )
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(job_id)
        .execute(self.pool.as_ref())
        .await?;
        
        Ok(())
    }

    /// Clean up old completed jobs (older than 30 days)
    pub async fn cleanup_old_jobs(&self) -> Result<u64, sqlx::Error> {
        let cutoff = (chrono::Utc::now() - chrono::Duration::days(30)).to_rfc3339();
        
        let result = sqlx::query(
            r#"
            DELETE FROM jobs 
            WHERE status IN ('success', 'failed', 'cancelled') 
            AND completed_at < ?
            "#
        )
        .bind(&cutoff)
        .execute(self.pool.as_ref())
        .await?;
        
        Ok(result.rows_affected())
    }

    /// Get job statistics
    pub async fn get_stats(&self) -> Result<JobStats, sqlx::Error> {
        let pending = self.count_jobs(Some(JobStatus::Pending)).await?;
        let running = self.count_jobs(Some(JobStatus::Running)).await?;
        let success = self.count_jobs(Some(JobStatus::Success)).await?;
        let failed = self.count_jobs(Some(JobStatus::Failed)).await?;
        let cancelled = self.count_jobs(Some(JobStatus::Cancelled)).await?;
        let total = self.count_jobs(None).await?;
        
        Ok(JobStats {
            pending,
            running,
            success,
            failed,
            cancelled,
            total,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct JobStats {
    pub pending: i64,
    pub running: i64,
    pub success: i64,
    pub failed: i64,
    pub cancelled: i64,
    pub total: i64,
}
