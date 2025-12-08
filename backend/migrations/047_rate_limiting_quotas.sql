-- Migration 047: API Rate Limiting & Storage Quotas
-- Provides user-based storage quotas, bandwidth limits, and API rate limiting

-- User storage quotas (GB-based)
CREATE TABLE IF NOT EXISTS user_storage_quotas (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    storage_limit_bytes INTEGER NOT NULL DEFAULT 10737418240, -- 10GB default
    storage_used_bytes INTEGER NOT NULL DEFAULT 0,
    warning_threshold_percent INTEGER NOT NULL DEFAULT 80,
    is_unlimited INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Bandwidth quotas (per day/month)
CREATE TABLE IF NOT EXISTS user_bandwidth_quotas (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    daily_upload_limit_bytes INTEGER DEFAULT NULL, -- NULL = unlimited
    daily_download_limit_bytes INTEGER DEFAULT NULL,
    monthly_upload_limit_bytes INTEGER DEFAULT NULL,
    monthly_download_limit_bytes INTEGER DEFAULT NULL,
    is_unlimited INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Bandwidth usage tracking
CREATE TABLE IF NOT EXISTS bandwidth_usage (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    date TEXT NOT NULL, -- YYYY-MM-DD format
    upload_bytes INTEGER NOT NULL DEFAULT 0,
    download_bytes INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, date)
);

-- API rate limiting configuration per user/role
CREATE TABLE IF NOT EXISTS api_rate_limits (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    role_name TEXT, -- NULL means user-specific, else applies to all users with this role
    endpoint_pattern TEXT NOT NULL DEFAULT '*', -- Wildcard pattern for endpoints
    requests_per_minute INTEGER NOT NULL DEFAULT 60,
    requests_per_hour INTEGER NOT NULL DEFAULT 1000,
    requests_per_day INTEGER NOT NULL DEFAULT 10000,
    burst_limit INTEGER NOT NULL DEFAULT 10, -- Max burst requests
    is_enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, role_name, endpoint_pattern)
);

-- API request tracking (for rate limiting and analytics)
CREATE TABLE IF NOT EXISTS api_request_log (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT REFERENCES users(id) ON DELETE SET NULL,
    endpoint TEXT NOT NULL,
    method TEXT NOT NULL,
    status_code INTEGER,
    response_time_ms INTEGER,
    request_size_bytes INTEGER DEFAULT 0,
    response_size_bytes INTEGER DEFAULT 0,
    ip_address TEXT,
    user_agent TEXT,
    is_rate_limited INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Quota alert notifications
CREATE TABLE IF NOT EXISTS quota_alerts (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    alert_type TEXT NOT NULL, -- 'storage_warning', 'storage_exceeded', 'bandwidth_warning', 'bandwidth_exceeded', 'rate_limit_warning'
    threshold_percent INTEGER,
    message TEXT NOT NULL,
    is_acknowledged INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Default rate limits per role
INSERT OR IGNORE INTO api_rate_limits (id, role_name, endpoint_pattern, requests_per_minute, requests_per_hour, requests_per_day, burst_limit)
VALUES 
    (lower(hex(randomblob(16))), 'admin', '*', 120, 5000, 50000, 50),
    (lower(hex(randomblob(16))), 'user', '*', 60, 1000, 10000, 20),
    (lower(hex(randomblob(16))), 'guest', '*', 20, 200, 2000, 5),
    (lower(hex(randomblob(16))), 'viewer', '*', 30, 500, 5000, 10);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_bandwidth_usage_user_date ON bandwidth_usage(user_id, date);
CREATE INDEX IF NOT EXISTS idx_api_request_log_user ON api_request_log(user_id, created_at);
CREATE INDEX IF NOT EXISTS idx_api_request_log_endpoint ON api_request_log(endpoint, created_at);
CREATE INDEX IF NOT EXISTS idx_quota_alerts_user ON quota_alerts(user_id, is_acknowledged);
CREATE INDEX IF NOT EXISTS idx_user_storage_quotas_user ON user_storage_quotas(user_id);
CREATE INDEX IF NOT EXISTS idx_user_bandwidth_quotas_user ON user_bandwidth_quotas(user_id);
CREATE INDEX IF NOT EXISTS idx_api_rate_limits_user ON api_rate_limits(user_id);
CREATE INDEX IF NOT EXISTS idx_api_rate_limits_role ON api_rate_limits(role_name);
