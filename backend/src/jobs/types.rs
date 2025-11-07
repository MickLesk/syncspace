//! Job types and structures

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Job {
    pub id: String,
    pub job_type: String,
    pub status: String,
    pub payload: String,  // JSON
    pub result: Option<String>,  // JSON
    pub error: Option<String>,
    pub attempts: i32,
    pub max_attempts: i32,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub scheduled_for: Option<String>,
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum JobType {
    FileIndexing { file_id: String, file_path: String },
    ThumbnailGeneration { file_id: String, file_path: String },
    BackupCreation { backup_id: String, include_files: bool },
    VersionCleanup { file_id: Option<String> },  // None = all files
    WebhookDelivery { webhook_id: String, event: String, payload: serde_json::Value },
    EmailNotification { to: String, subject: String, body: String },
    SearchIndexRebuild { full_rebuild: bool },
    DatabaseCleanup { table: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Pending,
    Running,
    Success,
    Failed,
    Cancelled,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Pending => write!(f, "pending"),
            JobStatus::Running => write!(f, "running"),
            JobStatus::Success => write!(f, "success"),
            JobStatus::Failed => write!(f, "failed"),
            JobStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResult {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl JobResult {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
            data: None,
        }
    }

    pub fn success_with_data(message: impl Into<String>, data: serde_json::Value) -> Self {
        Self {
            success: true,
            message: message.into(),
            data: Some(data),
        }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            data: None,
        }
    }
}

impl Job {
    pub fn new(job_type: JobType, created_by: Option<String>) -> Result<Self, serde_json::Error> {
        let id = uuid::Uuid::new_v4().to_string();
        let payload = serde_json::to_string(&job_type)?;
        let type_str = match &job_type {
            JobType::FileIndexing { .. } => "file_indexing",
            JobType::ThumbnailGeneration { .. } => "thumbnail_generation",
            JobType::BackupCreation { .. } => "backup_creation",
            JobType::VersionCleanup { .. } => "version_cleanup",
            JobType::WebhookDelivery { .. } => "webhook_delivery",
            JobType::EmailNotification { .. } => "email_notification",
            JobType::SearchIndexRebuild { .. } => "search_index_rebuild",
            JobType::DatabaseCleanup { .. } => "database_cleanup",
        };

        Ok(Self {
            id,
            job_type: type_str.to_string(),
            status: JobStatus::Pending.to_string(),
            payload,
            result: None,
            error: None,
            attempts: 0,
            max_attempts: 3,
            created_at: Utc::now().to_rfc3339(),
            started_at: None,
            completed_at: None,
            scheduled_for: None,
            created_by,
        })
    }

    pub fn with_schedule(mut self, scheduled_for: DateTime<Utc>) -> Self {
        self.scheduled_for = Some(scheduled_for.to_rfc3339());
        self
    }

    pub fn with_max_attempts(mut self, max_attempts: i32) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    pub fn parse_type(&self) -> Result<JobType, serde_json::Error> {
        serde_json::from_str(&self.payload)
    }

    pub fn is_due(&self) -> bool {
        match &self.scheduled_for {
            None => true,  // No schedule = run immediately
            Some(schedule) => {
                if let Ok(scheduled_time) = chrono::DateTime::parse_from_rfc3339(schedule) {
                    scheduled_time.with_timezone(&Utc) <= Utc::now()
                } else {
                    true  // Parse error = run immediately
                }
            }
        }
    }

    pub fn can_retry(&self) -> bool {
        self.attempts < self.max_attempts && self.status == JobStatus::Failed.to_string()
    }
}
