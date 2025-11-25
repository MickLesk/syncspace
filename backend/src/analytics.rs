/// Analytics and metrics for dashboards
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone, Serialize)]
pub struct DashboardStats {
    pub storage_stats: StorageMetrics,
    pub activity_stats: ActivityMetrics,
    pub file_stats: FileMetrics,
    pub user_stats: UserMetrics,
}

#[derive(Debug, Clone, Serialize)]
pub struct StorageMetrics {
    pub total_files: i64,
    pub total_size: i64,
    pub total_folders: i64,
    pub trash_size: i64,
    pub versions_size: i64,
    pub growth_trend: Vec<DataPoint>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ActivityMetrics {
    pub uploads_today: i64,
    pub downloads_today: i64,
    pub shares_today: i64,
    pub recent_activity: Vec<RecentActivity>,
    pub activity_heatmap: Vec<HeatmapData>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileMetrics {
    pub file_type_distribution: Vec<FileTypeCount>,
    pub largest_files: Vec<LargeFile>,
    pub most_shared: Vec<SharedFile>,
    pub most_viewed: Vec<ViewedFile>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserMetrics {
    pub total_users: i64,
    pub active_users_today: i64,
    pub storage_by_user: Vec<UserStorage>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DataPoint {
    pub date: String,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct RecentActivity {
    pub user_id: String,
    pub action: String,
    pub file_path: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HeatmapData {
    pub hour: i32,
    pub day: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct FileTypeCount {
    pub file_type: String,
    pub count: i64,
    pub total_size: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct LargeFile {
    pub id: String,
    pub name: String,
    pub size_bytes: i64,
    pub owner_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SharedFile {
    pub id: String,
    pub name: String,
    pub share_count: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ViewedFile {
    pub id: String,
    pub name: String,
    pub view_count: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct UserStorage {
    pub user_id: String,
    pub used_bytes: i64,
    pub file_count: i64,
}

/// Get comprehensive dashboard statistics
pub async fn get_dashboard_stats(
    pool: &SqlitePool,
    user_id: Option<&str>,
) -> Result<DashboardStats, sqlx::Error> {
    Ok(DashboardStats {
        storage_stats: get_storage_metrics(pool, user_id).await?,
        activity_stats: get_activity_metrics(pool, user_id).await?,
        file_stats: get_file_metrics(pool, user_id).await?,
        user_stats: get_user_metrics(pool).await?,
    })
}

/// Get storage metrics
/// Get storage metrics
pub async fn get_storage_metrics(
    pool: &SqlitePool,
    user_id: Option<&str>,
) -> Result<StorageMetrics, sqlx::Error> {
    let user_filter = if let Some(uid) = user_id {
        format!("WHERE owner_id = '{}'", uid)
    } else {
        String::new()
    };
    
    // Total files and size
    let (total_files, total_size): (i64, i64) = sqlx::query_as(&format!(
        "SELECT COUNT(*), COALESCE(SUM(size_bytes), 0) FROM files {} AND is_deleted = 0",
        user_filter
    ))
    .fetch_one(pool)
    .await?;
    
    // Total folders
    let total_folders: (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(*) FROM folders {} AND is_deleted = 0",
        user_filter
    ))
    .fetch_one(pool)
    .await?;
    
    // Trash size
    let where_clause = if user_id.is_some() { 
        format!("WHERE deleted_by = '{}'", user_id.unwrap()) 
    } else { 
        String::new() 
    };
    let trash_size: (i64,) = sqlx::query_as(&format!(
        "SELECT COALESCE(SUM(size_bytes), 0) FROM trash {}",
        where_clause
    ))
    .fetch_one(pool)
    .await?;
    
    // Versions size (sum of all file versions)
    let versions_size: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(size_bytes), 0) FROM file_versions"
    )
    .fetch_one(pool)
    .await?;
    
    // Growth trend (last 30 days)
    let growth_trend = get_growth_trend(pool, user_id, 30).await?;
    
    Ok(StorageMetrics {
        total_files,
        total_size,
        total_folders: total_folders.0,
        trash_size: trash_size.0,
        versions_size: versions_size.0,
        growth_trend,
    })
}

/// Get activity metrics
/// Get activity metrics
pub async fn get_activity_metrics(
    pool: &SqlitePool,
    user_id: Option<&str>,
) -> Result<ActivityMetrics, sqlx::Error> {
    let user_filter = if let Some(uid) = user_id {
        format!("AND user_id = '{}'", uid)
    } else {
        String::new()
    };
    
    // Uploads today
    let uploads_today: (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(*) FROM file_history 
         WHERE action = 'created' 
         AND DATE(created_at) = DATE('now') {}",
        user_filter
    ))
    .fetch_one(pool)
    .await?;
    
    // Downloads today
    let downloads_today: (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(*) FROM activity 
         WHERE action = 'download' 
         AND DATE(created_at) = DATE('now') {}",
        user_filter
    ))
    .fetch_one(pool)
    .await?;
    
    // Shares created today
    let shares_today: (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(*) FROM shares 
         WHERE DATE(created_at) = DATE('now') {}",
        "".to_string()  // shares don't have user filter in same way
    ))
    .fetch_one(pool)
    .await?;
    
    // Recent activity (last 50)
    let recent_activity: Vec<RecentActivity> = sqlx::query_as(&format!(
        "SELECT user_id, action, file_path, created_at 
         FROM activity 
         WHERE 1=1 {}
         ORDER BY created_at DESC 
         LIMIT 50",
        user_filter
    ))
    .fetch_all(pool)
    .await?;
    
    // Activity heatmap (count by hour and day)
    let heatmap_data: Vec<(i32, String, i64)> = sqlx::query_as(
        "SELECT 
            CAST(STRFTIME('%H', created_at) AS INTEGER) as hour,
            LOWER(STRFTIME('%A', created_at)) as day,
            COUNT(*) as count
         FROM activity
         WHERE created_at >= DATETIME('now', '-30 days')
         GROUP BY hour, day
         ORDER BY hour, day"
    )
    .fetch_all(pool)
    .await?;
    
    let activity_heatmap: Vec<HeatmapData> = heatmap_data
        .into_iter()
        .map(|(hour, day, count)| HeatmapData { hour, day, count })
        .collect();
    
    Ok(ActivityMetrics {
        uploads_today: uploads_today.0,
        downloads_today: downloads_today.0,
        shares_today: shares_today.0,
        recent_activity,
        activity_heatmap,
    })
}

/// Get file metrics
/// Get file metrics
pub async fn get_file_metrics(
    pool: &SqlitePool,
    user_id: Option<&str>,
) -> Result<FileMetrics, sqlx::Error> {
    let user_filter = if let Some(uid) = user_id {
        format!("WHERE owner_id = '{}' AND is_deleted = 0", uid)
    } else {
        "WHERE is_deleted = 0".to_string()
    };
    
    // File type distribution
    let file_type_distribution: Vec<FileTypeCount> = sqlx::query_as(&format!(
        "SELECT 
            CASE 
                WHEN LOWER(SUBSTR(name, -INSTR(REVERSE(name), '.') + 1)) IN ('jpg', 'jpeg', 'png', 'gif', 'webp') THEN 'image'
                WHEN LOWER(SUBSTR(name, -INSTR(REVERSE(name), '.') + 1)) IN ('mp4', 'avi', 'mkv', 'mov') THEN 'video'
                WHEN LOWER(SUBSTR(name, -INSTR(REVERSE(name), '.') + 1)) IN ('mp3', 'wav', 'flac') THEN 'audio'
                WHEN LOWER(SUBSTR(name, -INSTR(REVERSE(name), '.') + 1)) IN ('pdf', 'doc', 'docx', 'txt') THEN 'document'
                ELSE 'other'
            END as file_type,
            COUNT(*) as count,
            COALESCE(SUM(size_bytes), 0) as total_size
         FROM files {}
         GROUP BY file_type
         ORDER BY count DESC",
        user_filter
    ))
    .fetch_all(pool)
    .await?;
    
    // Largest files
    let largest_files: Vec<LargeFile> = sqlx::query_as(&format!(
        "SELECT id, name, size_bytes, owner_id 
         FROM files {}
         ORDER BY size_bytes DESC 
         LIMIT 10",
        user_filter
    ))
    .fetch_all(pool)
    .await?;
    
    // Most shared files (by share count)
    let most_shared_data: Vec<(String, String, i64)> = sqlx::query_as(&format!(
        "SELECT f.id, f.name, COUNT(s.id) as share_count
         FROM files f
         LEFT JOIN shares s ON f.id = s.file_id
         {}
         GROUP BY f.id, f.name
         ORDER BY share_count DESC
         LIMIT 10",
        user_filter
    ))
    .fetch_all(pool)
    .await?;
    
    let most_shared: Vec<SharedFile> = most_shared_data
        .into_iter()
        .map(|(id, name, share_count)| SharedFile { id, name, share_count })
        .collect();
    
    // Most viewed files (from activity logs)
    let most_viewed_data: Vec<(String, String, i64)> = sqlx::query_as(&format!(
        "SELECT f.id, f.name, COUNT(a.id) as view_count
         FROM files f
         LEFT JOIN activity a ON INSTR(a.file_path, f.name) > 0
         {}
         AND a.action IN ('view', 'download')
         GROUP BY f.id, f.name
         ORDER BY view_count DESC
         LIMIT 10",
        user_filter
    ))
    .fetch_all(pool)
    .await?;
    
    let most_viewed: Vec<ViewedFile> = most_viewed_data
        .into_iter()
        .map(|(id, name, view_count)| ViewedFile { id, name, view_count })
        .collect();
    
    Ok(FileMetrics {
        file_type_distribution,
        largest_files,
        most_shared,
        most_viewed,
    })
}

/// Get user metrics (admin only)
/// Get user metrics  
pub async fn get_user_metrics(
    pool: &SqlitePool,
) -> Result<UserMetrics, sqlx::Error> {
    // Total users
    let total_users: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM users"
    )
    .fetch_one(pool)
    .await?;
    
    // Active users today (users who performed actions)
    let active_users_today: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT user_id) FROM file_history 
         WHERE DATE(created_at) = DATE('now')"
    )
    .fetch_one(pool)
    .await?;
    
    // Storage by user
    let storage_by_user: Vec<UserStorage> = sqlx::query_as(
        "SELECT owner_id as user_id, 
                COALESCE(SUM(size_bytes), 0) as used_bytes,
                COUNT(*) as file_count
         FROM files 
         WHERE is_deleted = 0
         GROUP BY owner_id
         ORDER BY used_bytes DESC
         LIMIT 20"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(UserMetrics {
        total_users: total_users.0,
        active_users_today: active_users_today.0,
        storage_by_user,
    })
}

/// Get storage growth trend
async fn get_growth_trend(
    pool: &SqlitePool,
    user_id: Option<&str>,
    days: i64,
) -> Result<Vec<DataPoint>, sqlx::Error> {
    let user_filter = if let Some(uid) = user_id {
        format!("AND owner_id = '{}'", uid)
    } else {
        String::new()
    };
    
    let data_points: Vec<(String, i64)> = sqlx::query_as(&format!(
        "SELECT DATE(created_at) as date, 
                COALESCE(SUM(size_bytes), 0) as value
         FROM files
         WHERE DATE(created_at) >= DATE('now', '-{} days')
         AND is_deleted = 0
         {}
         GROUP BY DATE(created_at)
         ORDER BY date ASC",
        days, user_filter
    ))
    .fetch_all(pool)
    .await?;
    
    Ok(data_points.into_iter().map(|(date, value)| DataPoint { date, value }).collect())
}
