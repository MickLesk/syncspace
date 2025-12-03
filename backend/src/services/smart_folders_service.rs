use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Smart Folder Rule condition
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RuleCondition {
    pub field: String,    // file_type, size, date, tags, name_pattern
    pub operator: String, // equals, contains, greater_than, less_than, in, matches_regex
    pub value: String,    // the comparison value
}

/// Smart Folder Rule (combination of conditions)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SmartFolderRule {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub conditions: String, // JSON string of conditions array
    pub logic: String,      // "AND" or "OR"
    pub is_active: bool,
    pub icon: Option<String>,  // bootstrap icon name
    pub color: Option<String>, // hex color
    pub sort_by: String,       // name, modified, size, type
    pub sort_order: String,    // asc, desc
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Smart Folder with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SmartFolder {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub file_count: usize,
    pub total_size: u64,
    pub last_updated: DateTime<Utc>,
    pub conditions_count: usize,
}

/// Result of evaluating a file against rules
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RuleEvaluationResult {
    pub file_path: String,
    pub matches_rules: Vec<String>, // IDs of matching rules
}

#[allow(dead_code)]
pub struct SmartFoldersService;

#[allow(dead_code)]
impl SmartFoldersService {
    /// Evaluate file metadata against a single condition
    pub fn evaluate_condition(
        file_name: &str,
        file_size: u64,
        _file_type: &str,
        modified_at: DateTime<Utc>,
        file_tags: &[String],
        condition: &RuleCondition,
    ) -> bool {
        match condition.field.as_str() {
            // File type matching (e.g., "pdf", "image", "document", "video", "audio")
            "file_type" => match condition.operator.as_str() {
                "equals" => {
                    let file_ext = file_name.split('.').last().unwrap_or("").to_lowercase();
                    Self::get_file_category(&file_ext) == condition.value
                        || file_ext == condition.value
                }
                "contains" => {
                    let file_ext = file_name.split('.').last().unwrap_or("").to_lowercase();
                    file_ext.contains(&condition.value.to_lowercase())
                }
                _ => false,
            },

            // File size matching (in bytes)
            "size" => {
                if let Ok(comp_size) = condition.value.parse::<u64>() {
                    match condition.operator.as_str() {
                        "equals" => file_size == comp_size,
                        "greater_than" => file_size > comp_size,
                        "less_than" => file_size < comp_size,
                        "greater_equal" => file_size >= comp_size,
                        "less_equal" => file_size <= comp_size,
                        _ => false,
                    }
                } else {
                    false
                }
            }

            // Modified date matching
            "date" => {
                if let Ok(comp_date) = DateTime::parse_from_rfc3339(&condition.value) {
                    let comp_utc = comp_date.with_timezone(&Utc);
                    match condition.operator.as_str() {
                        "after" => modified_at > comp_utc,
                        "before" => modified_at < comp_utc,
                        "today" => {
                            let today = Utc::now().date_naive();
                            modified_at.date_naive() == today
                        }
                        "this_week" => {
                            let now = Utc::now();
                            let days_diff = now.signed_duration_since(modified_at).num_days();
                            days_diff >= 0 && days_diff <= 7
                        }
                        "this_month" => {
                            let now = Utc::now();
                            let days_diff = now.signed_duration_since(modified_at).num_days();
                            days_diff >= 0 && days_diff <= 30
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }

            // Tags matching
            "tags" => {
                let tag_values: Vec<&str> = condition.value.split(',').collect();
                match condition.operator.as_str() {
                    "contains_any" => tag_values
                        .iter()
                        .any(|tag| file_tags.iter().any(|ft| ft == *tag)),
                    "contains_all" => tag_values
                        .iter()
                        .all(|tag| file_tags.iter().any(|ft| ft == *tag)),
                    _ => false,
                }
            }

            // File name pattern matching
            "name_pattern" => match condition.operator.as_str() {
                "contains" => file_name
                    .to_lowercase()
                    .contains(&condition.value.to_lowercase()),
                "starts_with" => file_name
                    .to_lowercase()
                    .starts_with(&condition.value.to_lowercase()),
                "ends_with" => file_name
                    .to_lowercase()
                    .ends_with(&condition.value.to_lowercase()),
                "matches_regex" => {
                    if let Ok(re) = regex::Regex::new(&condition.value) {
                        re.is_match(file_name)
                    } else {
                        false
                    }
                }
                _ => false,
            },

            _ => false,
        }
    }

    /// Evaluate file against all conditions with AND/OR logic
    pub fn evaluate_rule(
        file_name: &str,
        file_size: u64,
        file_type: &str,
        modified_at: DateTime<Utc>,
        file_tags: &[String],
        rule: &SmartFolderRule,
    ) -> bool {
        if let Ok(conditions) = serde_json::from_str::<Vec<RuleCondition>>(&rule.conditions) {
            if conditions.is_empty() {
                return false;
            }

            let results: Vec<bool> = conditions
                .iter()
                .map(|cond| {
                    Self::evaluate_condition(
                        file_name,
                        file_size,
                        file_type,
                        modified_at,
                        file_tags,
                        cond,
                    )
                })
                .collect();

            match rule.logic.as_str() {
                "AND" => results.iter().all(|&r| r),
                "OR" => results.iter().any(|&r| r),
                _ => false,
            }
        } else {
            false
        }
    }

    /// Categorize file by extension
    pub fn get_file_category(ext: &str) -> String {
        match ext.to_lowercase().as_str() {
            // Documents
            "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt" => "document".to_string(),
            // Spreadsheets
            "xls" | "xlsx" | "csv" | "ods" => "spreadsheet".to_string(),
            // Presentations
            "ppt" | "pptx" | "odp" => "presentation".to_string(),
            // Images
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" | "ico" => "image".to_string(),
            // Videos
            "mp4" | "avi" | "mov" | "mkv" | "flv" | "wmv" | "webm" => "video".to_string(),
            // Audio
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" => "audio".to_string(),
            // Archives
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" => "archive".to_string(),
            // Code
            "js" | "ts" | "py" | "java" | "cpp" | "c" | "rs" | "go" | "rb" => "code".to_string(),
            // Data
            "json" | "xml" | "yaml" | "sql" => "data".to_string(),
            _ => "other".to_string(),
        }
    }

    /// Parse human-readable size (e.g., "10MB", "1GB") to bytes
    pub fn parse_size(size_str: &str) -> Option<u64> {
        let s = size_str.trim().to_uppercase();
        let (num_str, multiplier) = if s.ends_with("GB") {
            (&s[..s.len() - 2], 1024_u64.pow(3))
        } else if s.ends_with("MB") {
            (&s[..s.len() - 2], 1024_u64.pow(2))
        } else if s.ends_with("KB") {
            (&s[..s.len() - 2], 1024_u64)
        } else if s.ends_with("B") {
            (&s[..s.len() - 1], 1_u64)
        } else {
            (&s[..], 1_u64)
        };

        num_str
            .trim()
            .parse::<f64>()
            .ok()
            .map(|n| (n * multiplier as f64) as u64)
    }

    /// Format bytes to human-readable size
    pub fn format_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_idx = 0;

        while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
            size /= 1024.0;
            unit_idx += 1;
        }

        format!("{:.2} {}", size, UNITS[unit_idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_category() {
        assert_eq!(SmartFoldersService::get_file_category("pdf"), "document");
        assert_eq!(SmartFoldersService::get_file_category("mp4"), "video");
        assert_eq!(SmartFoldersService::get_file_category("jpg"), "image");
    }

    #[test]
    fn test_parse_size() {
        assert_eq!(SmartFoldersService::parse_size("10MB"), Some(10_485_760));
        assert_eq!(SmartFoldersService::parse_size("1GB"), Some(1_073_741_824));
        assert_eq!(SmartFoldersService::parse_size("512KB"), Some(524_288));
    }

    #[test]
    fn test_format_size() {
        assert!(SmartFoldersService::format_size(1024).contains("KB"));
        assert!(SmartFoldersService::format_size(1_048_576).contains("MB"));
    }
}
