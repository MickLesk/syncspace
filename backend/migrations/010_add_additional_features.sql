-- Migration: Add preview metadata, audit logs, virus scanning tables
-- Created: 2025-10-24

-- Preview metadata table
CREATE TABLE IF NOT EXISTS preview_metadata (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    preview_type TEXT NOT NULL CHECK(preview_type IN ('thumbnail', 'pdf', 'video', 'document', 'code')),
    preview_path TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    width INTEGER,
    height INTEGER,
    duration REAL,
    page_count INTEGER,
    generated_at TEXT NOT NULL,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
);

CREATE INDEX idx_preview_file ON preview_metadata(file_id);
CREATE INDEX idx_preview_type ON preview_metadata(preview_type);

-- Audit logs table
CREATE TABLE IF NOT EXISTS audit_logs (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    action TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    metadata TEXT, -- JSON
    severity TEXT NOT NULL CHECK(severity IN ('info', 'warning', 'critical')),
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_audit_user ON audit_logs(user_id);
CREATE INDEX idx_audit_action ON audit_logs(action);
CREATE INDEX idx_audit_resource ON audit_logs(resource_type, resource_id);
CREATE INDEX idx_audit_created ON audit_logs(created_at);

-- Data retention policies table
CREATE TABLE IF NOT EXISTS data_retention_policies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    retention_days INTEGER NOT NULL,
    auto_delete INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

-- Virus scan results table
CREATE TABLE IF NOT EXISTS scan_results (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    scan_status TEXT NOT NULL CHECK(scan_status IN ('clean', 'infected', 'error')),
    virus_name TEXT,
    scanned_at TEXT NOT NULL,
    quarantined INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
);

CREATE INDEX idx_scan_file ON scan_results(file_id);
CREATE INDEX idx_scan_status ON scan_results(scan_status);

-- Smart folders table
CREATE TABLE IF NOT EXISTS smart_folders (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    rules TEXT NOT NULL, -- JSON array of rules
    auto_update INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_smart_folder_user ON smart_folders(user_id);

-- External storage providers table
CREATE TABLE IF NOT EXISTS storage_providers (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    provider_type TEXT NOT NULL CHECK(provider_type IN ('s3', 'gdrive', 'dropbox', 'sftp', 'webdav')),
    name TEXT NOT NULL,
    config TEXT NOT NULL, -- JSON config
    is_active INTEGER NOT NULL DEFAULT 1,
    sync_enabled INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_storage_provider_user ON storage_providers(user_id);
CREATE INDEX idx_storage_provider_type ON storage_providers(provider_type);

-- Search index table (full-text search)
CREATE TABLE IF NOT EXISTS search_index (
    file_id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    indexed_at TEXT NOT NULL,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
);

-- Create FTS5 virtual table for full-text search (if SQLite supports it)
-- CREATE VIRTUAL TABLE IF NOT EXISTS search_index_fts USING fts5(file_id, content, indexed_at);
