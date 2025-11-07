//! Cron Scheduler for Background Jobs
//! 
//! Evaluates cron expressions and automatically enqueues jobs at scheduled times.

use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

// TODO: Re-enable after job system API is fixed
// use crate::jobs::{enqueue_job, CronJob, JobType};

/// Cron scheduler that checks and enqueues jobs
pub struct CronScheduler {
    pool: SqlitePool,
    check_interval_secs: u64,
}

impl CronScheduler {
    /// Create new cron scheduler
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            check_interval_secs: 60, // Check every minute
        }
    }
    
    /// Start the scheduler loop
    pub async fn start(&self) {
        tracing::info!("⏰ Cron scheduler (old system) - DISABLED pending refactor");
        // TODO: Re-enable after job system refactor
        /*
        tracing::info!("⏰ Starting cron scheduler (checking every {}s)", self.check_interval_secs);
        
        loop {
            if let Err(e) = self.process_cron_jobs().await {
                tracing::error!("Error processing cron jobs: {}", e);
            }
            
            sleep(Duration::from_secs(self.check_interval_secs)).await;
        }
        */
    }
    
    /*
    // TODO: Re-enable after job system refactor
    /// Process all enabled cron jobs
    async fn process_cron_jobs(&self) -> Result<(), sqlx::Error> {
        let now = Utc::now();
        let now_str = now.to_rfc3339();
        
        // Get all enabled cron jobs that are due
        let cron_jobs = sqlx::query_as::<_, CronJob>(
            "SELECT * FROM cron_jobs WHERE enabled = 1 AND next_run_at <= ?"
        )
        .bind(&now_str)
        .fetch_all(&self.pool)
        .await?;
        
        for cron_job in cron_jobs {
            tracing::info!("Processing cron job: {} ({})", cron_job.name, cron_job.cron_expression);
            
            // Parse payload
            let payload: serde_json::Value = match serde_json::from_str(&cron_job.payload) {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!("Invalid cron job payload for {}: {}", cron_job.name, e);
                    continue;
                }
            };
            
            // Enqueue the job
            let job_type = JobType::from_str(&cron_job.job_type);
            match enqueue_job(&self.pool, job_type, payload, 5, None).await {
                Ok(job_id) => {
                    tracing::info!("Enqueued job {} from cron {}", job_id, cron_job.name);
                }
                Err(e) => {
                    tracing::error!("Failed to enqueue cron job {}: {}", cron_job.name, e);
                    continue;
                }
            }
            
            // Calculate next run time
            let next_run = match calculate_next_run(&cron_job.cron_expression, now) {
                Some(t) => t,
                None => {
                    tracing::error!("Invalid cron expression for {}: {}", cron_job.name, cron_job.cron_expression);
                    continue;
                }
            };
            
            // Update cron job
            sqlx::query(
                "UPDATE cron_jobs SET last_run_at = ?, next_run_at = ?, updated_at = ? WHERE id = ?"
            )
            .bind(&now_str)
            .bind(next_run.to_rfc3339())
            .bind(&now_str)
            .bind(&cron_job.id)
            .execute(&self.pool)
            .await?;
        }
        
        Ok(())
    }
    */  // End of commented process_cron_jobs
}  // End of impl CronScheduler

// ============================================================================
// Cron Expression Parser (Simplified)
// ============================================================================

/// Parse and evaluate cron expression
/// Format: "minute hour day month weekday"
/// Examples:
///   - "0 * * * *" - Every hour at minute 0
///   - "*/5 * * * *" - Every 5 minutes
///   - "0 0 * * *" - Daily at midnight
///   - "0 0 * * 0" - Weekly on Sunday at midnight
///   - "0 0 1 * *" - Monthly on the 1st at midnight
pub fn calculate_next_run(cron_expr: &str, from: DateTime<Utc>) -> Option<DateTime<Utc>> {
    let parts: Vec<&str> = cron_expr.split_whitespace().collect();
    if parts.len() != 5 {
        return None;
    }
    
    let minute_expr = parts[0];
    let hour_expr = parts[1];
    let day_expr = parts[2];
    let month_expr = parts[3];
    let weekday_expr = parts[4];
    
    // Start from next minute
    let mut candidate = from + chrono::Duration::minutes(1);
    candidate = candidate
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap();
    
    // Try up to 366 days in the future (handle yearly schedules)
    for _ in 0..366 * 24 * 60 {
        if matches_cron(
            candidate,
            minute_expr,
            hour_expr,
            day_expr,
            month_expr,
            weekday_expr,
        ) {
            return Some(candidate);
        }
        
        candidate = candidate + chrono::Duration::minutes(1);
    }
    
    None
}

fn matches_cron(
    dt: DateTime<Utc>,
    minute_expr: &str,
    hour_expr: &str,
    day_expr: &str,
    month_expr: &str,
    weekday_expr: &str,
) -> bool {
    let minute = dt.minute();
    let hour = dt.hour();
    let day = dt.day();
    let month = dt.month();
    let weekday = dt.weekday().number_from_sunday() % 7; // 0 = Sunday
    
    matches_field(minute_expr, minute, 0, 59)
        && matches_field(hour_expr, hour, 0, 23)
        && matches_field(day_expr, day, 1, 31)
        && matches_field(month_expr, month, 1, 12)
        && matches_field(weekday_expr, weekday, 0, 6)
}

fn matches_field(expr: &str, value: u32, min: u32, max: u32) -> bool {
    // Wildcard
    if expr == "*" {
        return true;
    }
    
    // Step values (e.g., */5)
    if expr.starts_with("*/") {
        if let Ok(step) = expr[2..].parse::<u32>() {
            return value % step == 0;
        }
    }
    
    // Range (e.g., 1-5)
    if expr.contains('-') {
        let parts: Vec<&str> = expr.split('-').collect();
        if parts.len() == 2 {
            if let (Ok(start), Ok(end)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                return value >= start && value <= end;
            }
        }
    }
    
    // List (e.g., 1,3,5)
    if expr.contains(',') {
        let values: Vec<u32> = expr
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();
        return values.contains(&value);
    }
    
    // Exact match
    if let Ok(exact) = expr.parse::<u32>() {
        return value == exact;
    }
    
    false
}

// ============================================================================
// Database Operations
// ============================================================================

/// Create new cron job
pub async fn create_cron_job(
    pool: &SqlitePool,
    name: String,
    job_type: JobType,
    cron_expression: String,
    payload: serde_json::Value,
) -> Result<String, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let job_type_str = job_type.as_str();
    let payload_str = serde_json::to_string(&payload).unwrap_or_default();
    let now = Utc::now().to_rfc3339();
    
    // Calculate first run time
    let next_run = calculate_next_run(&cron_expression, Utc::now())
        .ok_or_else(|| sqlx::Error::Protocol("Invalid cron expression".into()))?;
    
    sqlx::query(
        r#"
        INSERT INTO cron_jobs (id, name, job_type, cron_expression, payload, enabled, next_run_at, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, 1, ?, ?, ?)
        "#
    )
    .bind(&id)
    .bind(&name)
    .bind(job_type_str)
    .bind(&cron_expression)
    .bind(&payload_str)
    .bind(next_run.to_rfc3339())
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;
    
    tracing::info!("Created cron job {} with expression {}", name, cron_expression);
    Ok(id)
}

/// List all cron jobs
pub async fn list_cron_jobs(pool: &SqlitePool) -> Result<Vec<CronJob>, sqlx::Error> {
    sqlx::query_as::<_, CronJob>("SELECT * FROM cron_jobs ORDER BY name ASC")
        .fetch_all(pool)
        .await
}

/// Get cron job by ID
pub async fn get_cron_job(pool: &SqlitePool, id: &str) -> Result<Option<CronJob>, sqlx::Error> {
    sqlx::query_as::<_, CronJob>("SELECT * FROM cron_jobs WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Enable/disable cron job
pub async fn set_cron_job_enabled(
    pool: &SqlitePool,
    id: &str,
    enabled: bool,
) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        "UPDATE cron_jobs SET enabled = ?, updated_at = ? WHERE id = ?"
    )
    .bind(if enabled { 1 } else { 0 })
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;
    
    tracing::info!("Cron job {} {}", id, if enabled { "enabled" } else { "disabled" });
    Ok(())
}

/// Delete cron job
pub async fn delete_cron_job(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM cron_jobs WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    tracing::info!("Deleted cron job {}", id);
    Ok(())
}

/// Update cron job
pub async fn update_cron_job(
    pool: &SqlitePool,
    id: &str,
    cron_expression: Option<String>,
    payload: Option<serde_json::Value>,
) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    
    if let Some(expr) = cron_expression {
        // Validate and calculate next run
        let next_run = calculate_next_run(&expr, Utc::now())
            .ok_or_else(|| sqlx::Error::Protocol("Invalid cron expression".into()))?;
        
        sqlx::query(
            "UPDATE cron_jobs SET cron_expression = ?, next_run_at = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&expr)
        .bind(next_run.to_rfc3339())
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;
    }
    
    if let Some(p) = payload {
        let payload_str = serde_json::to_string(&p).unwrap_or_default();
        sqlx::query(
            "UPDATE cron_jobs SET payload = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&payload_str)
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    
    #[test]
    fn test_cron_every_minute() {
        let from = Utc.with_ymd_and_hms(2024, 1, 1, 12, 30, 0).unwrap();
        let next = calculate_next_run("* * * * *", from).unwrap();
        assert_eq!(next, Utc.with_ymd_and_hms(2024, 1, 1, 12, 31, 0).unwrap());
    }
    
    #[test]
    fn test_cron_every_hour() {
        let from = Utc.with_ymd_and_hms(2024, 1, 1, 12, 30, 0).unwrap();
        let next = calculate_next_run("0 * * * *", from).unwrap();
        assert_eq!(next, Utc.with_ymd_and_hms(2024, 1, 1, 13, 0, 0).unwrap());
    }
    
    #[test]
    fn test_cron_every_5_minutes() {
        let from = Utc.with_ymd_and_hms(2024, 1, 1, 12, 32, 0).unwrap();
        let next = calculate_next_run("*/5 * * * *", from).unwrap();
        assert_eq!(next, Utc.with_ymd_and_hms(2024, 1, 1, 12, 35, 0).unwrap());
    }
    
    #[test]
    fn test_cron_daily_midnight() {
        let from = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        let next = calculate_next_run("0 0 * * *", from).unwrap();
        assert_eq!(next, Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap());
    }
    
    #[test]
    fn test_cron_specific_time() {
        let from = Utc.with_ymd_and_hms(2024, 1, 1, 8, 0, 0).unwrap();
        let next = calculate_next_run("30 14 * * *", from).unwrap();
        assert_eq!(next, Utc.with_ymd_and_hms(2024, 1, 1, 14, 30, 0).unwrap());
    }
}
