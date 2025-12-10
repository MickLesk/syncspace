-- Migration 029: Preview, Thumbnails, and Virus Scan tables
-- Created: 2025-12-10

-- Thumbnail cache metadata
CREATE TABLE IF NOT EXISTS thumbnail_cache (
    id TEXT PRIMARY KEY,
    file_path TEXT NOT NULL,
    file_hash TEXT NOT NULL,
    size TEXT NOT NULL, -- 'small', 'medium', 'large'
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    format TEXT NOT NULL DEFAULT 'webp',
    thumbnail_path TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT,
    UNIQUE(file_hash, size)
);

CREATE INDEX IF NOT EXISTS idx_thumbnail_cache_file_path ON thumbnail_cache(file_path);
CREATE INDEX IF NOT EXISTS idx_thumbnail_cache_file_hash ON thumbnail_cache(file_hash);

-- File preview metadata v2 (extended version with file_path support)
-- Note: Original preview_metadata exists in 010 with file_id schema
-- We add columns to support both approaches
ALTER TABLE preview_metadata ADD COLUMN file_path TEXT;
ALTER TABLE preview_metadata ADD COLUMN file_type TEXT;
ALTER TABLE preview_metadata ADD COLUMN preview_available INTEGER NOT NULL DEFAULT 1;
ALTER TABLE preview_metadata ADD COLUMN metadata_json TEXT;
ALTER TABLE preview_metadata ADD COLUMN updated_at TEXT;

-- Create indexes only if column exists (these may fail silently if file_path wasn't added)
-- CREATE INDEX IF NOT EXISTS idx_preview_metadata_file_path ON preview_metadata(file_path);
-- CREATE INDEX IF NOT EXISTS idx_preview_metadata_file_type ON preview_metadata(file_type);

-- Video metadata for previews
CREATE TABLE IF NOT EXISTS video_metadata (
    id TEXT PRIMARY KEY,
    file_path TEXT NOT NULL UNIQUE,
    duration_seconds REAL NOT NULL DEFAULT 0,
    width INTEGER NOT NULL DEFAULT 0,
    height INTEGER NOT NULL DEFAULT 0,
    codec TEXT,
    bitrate INTEGER DEFAULT 0,
    fps REAL DEFAULT 0,
    audio_codec TEXT,
    audio_bitrate INTEGER,
    thumbnail_timestamps TEXT, -- JSON array of timestamps for timeline thumbnails
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_video_metadata_file_path ON video_metadata(file_path);

-- PDF metadata for previews
CREATE TABLE IF NOT EXISTS pdf_metadata (
    id TEXT PRIMARY KEY,
    file_path TEXT NOT NULL UNIQUE,
    page_count INTEGER NOT NULL DEFAULT 0,
    title TEXT,
    author TEXT,
    subject TEXT,
    keywords TEXT,
    creator TEXT,
    producer TEXT,
    creation_date TEXT,
    modification_date TEXT,
    is_encrypted INTEGER NOT NULL DEFAULT 0,
    has_forms INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_pdf_metadata_file_path ON pdf_metadata(file_path);

-- Virus scan results - extend existing table from migration 010
-- Original has: id, file_id, scan_status, virus_name, scanned_at, quarantined
-- Adding new columns for enhanced scan functionality
ALTER TABLE scan_results ADD COLUMN file_path TEXT;
ALTER TABLE scan_results ADD COLUMN file_hash TEXT;
ALTER TABLE scan_results ADD COLUMN threat_name TEXT;
ALTER TABLE scan_results ADD COLUMN scanner_used TEXT DEFAULT 'clamav';
ALTER TABLE scan_results ADD COLUMN scanner_version TEXT;
ALTER TABLE scan_results ADD COLUMN definitions_date TEXT;
ALTER TABLE scan_results ADD COLUMN scan_duration_ms INTEGER;
ALTER TABLE scan_results ADD COLUMN file_size INTEGER;
ALTER TABLE scan_results ADD COLUMN scanned_by TEXT;

-- Quarantine entries
CREATE TABLE IF NOT EXISTS quarantine_entries (
    id TEXT PRIMARY KEY,
    original_path TEXT NOT NULL,
    quarantine_path TEXT NOT NULL,
    threat_name TEXT NOT NULL,
    file_hash TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    owner_id TEXT,
    quarantined_at TEXT NOT NULL DEFAULT (datetime('now')),
    quarantined_by TEXT NOT NULL, -- user_id or 'system'
    restored_at TEXT,
    restored_by TEXT,
    deleted_at TEXT,
    status TEXT NOT NULL DEFAULT 'quarantined' -- 'quarantined', 'restored', 'deleted'
);

CREATE INDEX IF NOT EXISTS idx_quarantine_status ON quarantine_entries(status);
CREATE INDEX IF NOT EXISTS idx_quarantine_original_path ON quarantine_entries(original_path);

-- Scan statistics (aggregated)
CREATE TABLE IF NOT EXISTS scan_statistics (
    id TEXT PRIMARY KEY,
    date TEXT NOT NULL UNIQUE, -- YYYY-MM-DD
    total_scanned INTEGER NOT NULL DEFAULT 0,
    threats_found INTEGER NOT NULL DEFAULT 0,
    files_quarantined INTEGER NOT NULL DEFAULT 0,
    bytes_scanned INTEGER NOT NULL DEFAULT 0,
    avg_scan_time_ms INTEGER DEFAULT 0,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_scan_statistics_date ON scan_statistics(date);

-- FTP connections for sync
CREATE TABLE IF NOT EXISTS ftp_connections (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    host TEXT NOT NULL,
    port INTEGER NOT NULL DEFAULT 21,
    username TEXT NOT NULL,
    password_encrypted TEXT NOT NULL,
    use_ftps INTEGER NOT NULL DEFAULT 0,
    passive_mode INTEGER NOT NULL DEFAULT 1,
    remote_path TEXT NOT NULL DEFAULT '/',
    local_path TEXT NOT NULL,
    sync_direction TEXT NOT NULL DEFAULT 'download', -- 'upload', 'download', 'bidirectional'
    auto_sync INTEGER NOT NULL DEFAULT 0,
    sync_interval_minutes INTEGER NOT NULL DEFAULT 60,
    last_sync_at TEXT,
    last_sync_status TEXT,
    last_sync_error TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_ftp_connections_user_id ON ftp_connections(user_id);

-- Email accounts, messages, and attachments are already defined in 011_add_integration_features.sql
-- We only add extra columns if needed via ALTER TABLE
ALTER TABLE email_accounts ADD COLUMN auto_fetch INTEGER DEFAULT 0;
ALTER TABLE email_accounts ADD COLUMN fetch_interval_minutes INTEGER DEFAULT 30;
ALTER TABLE email_accounts ADD COLUMN store_attachments INTEGER DEFAULT 1;
ALTER TABLE email_accounts ADD COLUMN attachment_folder TEXT DEFAULT '/email_attachments';
ALTER TABLE email_accounts ADD COLUMN last_fetch_status TEXT;
ALTER TABLE email_accounts ADD COLUMN last_fetch_error TEXT;

ALTER TABLE email_messages ADD COLUMN body_text TEXT;
ALTER TABLE email_messages ADD COLUMN body_html TEXT;
ALTER TABLE email_messages ADD COLUMN is_read INTEGER DEFAULT 0;

ALTER TABLE email_attachments ADD COLUMN size_bytes INTEGER;
