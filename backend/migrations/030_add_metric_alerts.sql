-- Add metric alerts table for monitoring thresholds
CREATE TABLE IF NOT EXISTS metric_alerts (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    metric TEXT NOT NULL,  -- e.g., 'http_error_rate', 'storage_usage', 'cache_hit_rate'
    condition TEXT NOT NULL,  -- 'gt', 'lt', 'eq', 'gte', 'lte'
    threshold REAL NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1,
    triggered INTEGER NOT NULL DEFAULT 0,
    last_triggered_at TEXT,
    notification_channels TEXT,  -- JSON array: ['email', 'webhook', 'ui']
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Index for quick alert lookups
CREATE INDEX IF NOT EXISTS idx_metric_alerts_enabled ON metric_alerts(enabled);
CREATE INDEX IF NOT EXISTS idx_metric_alerts_metric ON metric_alerts(metric);

-- Metrics history for time-series data
CREATE TABLE IF NOT EXISTS metrics_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    http_requests INTEGER NOT NULL DEFAULT 0,
    http_errors INTEGER NOT NULL DEFAULT 0,
    file_uploads INTEGER NOT NULL DEFAULT 0,
    file_downloads INTEGER NOT NULL DEFAULT 0,
    active_users INTEGER NOT NULL DEFAULT 0,
    websocket_connections INTEGER NOT NULL DEFAULT 0,
    storage_bytes INTEGER NOT NULL DEFAULT 0,
    cache_hits INTEGER NOT NULL DEFAULT 0,
    cache_misses INTEGER NOT NULL DEFAULT 0,
    database_queries INTEGER NOT NULL DEFAULT 0
);

-- Index for time-based queries
CREATE INDEX IF NOT EXISTS idx_metrics_history_timestamp ON metrics_history(timestamp);
