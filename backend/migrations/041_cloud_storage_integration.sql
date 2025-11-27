-- Migration 041: Cloud Storage Integration
-- Multi-backend storage support (S3, MinIO, GCS, Azure Blob Storage)

-- Storage backends configuration
CREATE TABLE IF NOT EXISTS storage_backends (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    backend_type TEXT NOT NULL, -- 'local', 's3', 'minio', 'gcs', 'azure_blob'
    config TEXT NOT NULL, -- JSON: endpoint, bucket/container, credentials, region, etc.
    is_active INTEGER NOT NULL DEFAULT 1,
    is_default INTEGER NOT NULL DEFAULT 0, -- Default backend for new uploads
    priority INTEGER NOT NULL DEFAULT 100, -- Higher priority = preferred for reads (failover)
    storage_used_bytes INTEGER NOT NULL DEFAULT 0,
    file_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_storage_backends_active ON storage_backends(is_active);
CREATE INDEX IF NOT EXISTS idx_storage_backends_priority ON storage_backends(priority DESC);
CREATE INDEX IF NOT EXISTS idx_storage_backends_type ON storage_backends(backend_type);

-- File storage locations (maps files to storage backends)
CREATE TABLE IF NOT EXISTS file_storage_locations (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    backend_id TEXT NOT NULL,
    storage_path TEXT NOT NULL, -- Path/key in the backend storage
    size_bytes INTEGER NOT NULL,
    checksum_sha256 TEXT,
    is_primary INTEGER NOT NULL DEFAULT 1, -- Primary copy vs replica
    upload_status TEXT NOT NULL DEFAULT 'completed', -- 'pending', 'uploading', 'completed', 'failed'
    created_at TEXT NOT NULL,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (backend_id) REFERENCES storage_backends(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_file_storage_locations_file ON file_storage_locations(file_id);
CREATE INDEX IF NOT EXISTS idx_file_storage_locations_backend ON file_storage_locations(backend_id);
CREATE INDEX IF NOT EXISTS idx_file_storage_locations_primary ON file_storage_locations(is_primary);
CREATE INDEX IF NOT EXISTS idx_file_storage_locations_status ON file_storage_locations(upload_status);

-- Storage migration jobs (for moving files between backends)
CREATE TABLE IF NOT EXISTS storage_migration_jobs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    source_backend_id TEXT NOT NULL,
    target_backend_id TEXT NOT NULL,
    file_filter TEXT, -- JSON: file_type, size_range, date_range, path_pattern
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'running', 'completed', 'failed', 'cancelled'
    progress_current INTEGER NOT NULL DEFAULT 0,
    progress_total INTEGER NOT NULL DEFAULT 0,
    bytes_migrated INTEGER NOT NULL DEFAULT 0,
    bytes_total INTEGER NOT NULL DEFAULT 0,
    files_migrated INTEGER NOT NULL DEFAULT 0,
    files_failed INTEGER NOT NULL DEFAULT 0,
    error_message TEXT,
    started_at TEXT,
    completed_at TEXT,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (source_backend_id) REFERENCES storage_backends(id) ON DELETE RESTRICT,
    FOREIGN KEY (target_backend_id) REFERENCES storage_backends(id) ON DELETE RESTRICT,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_storage_migration_jobs_status ON storage_migration_jobs(status);
CREATE INDEX IF NOT EXISTS idx_storage_migration_jobs_created ON storage_migration_jobs(created_at);

-- Storage backend health checks
CREATE TABLE IF NOT EXISTS storage_backend_health (
    id TEXT PRIMARY KEY,
    backend_id TEXT NOT NULL,
    check_type TEXT NOT NULL, -- 'connectivity', 'read', 'write', 'latency'
    status TEXT NOT NULL, -- 'healthy', 'degraded', 'unhealthy'
    response_time_ms INTEGER,
    error_message TEXT,
    checked_at TEXT NOT NULL,
    FOREIGN KEY (backend_id) REFERENCES storage_backends(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_storage_backend_health_backend ON storage_backend_health(backend_id);
CREATE INDEX IF NOT EXISTS idx_storage_backend_health_checked ON storage_backend_health(checked_at);

-- Insert default local storage backend
INSERT OR IGNORE INTO storage_backends (id, name, backend_type, config, is_active, is_default, priority, created_at, updated_at)
VALUES (
    'backend-local-default',
    'Local Storage',
    'local',
    '{"base_path":"./data","description":"Default local filesystem storage"}',
    1,
    1,
    1000,
    datetime('now'),
    datetime('now')
);
