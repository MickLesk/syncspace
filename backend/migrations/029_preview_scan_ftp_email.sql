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

-- File preview metadata
CREATE TABLE IF NOT EXISTS preview_metadata (
    id TEXT PRIMARY KEY,
    file_path TEXT NOT NULL UNIQUE,
    file_type TEXT NOT NULL,
    preview_type TEXT NOT NULL, -- 'image', 'video', 'pdf', 'document', 'code', 'text'
    preview_available INTEGER NOT NULL DEFAULT 1,
    metadata_json TEXT, -- JSON blob with type-specific metadata
    generated_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_preview_metadata_file_path ON preview_metadata(file_path);
CREATE INDEX IF NOT EXISTS idx_preview_metadata_file_type ON preview_metadata(file_type);

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

-- Virus scan results
CREATE TABLE IF NOT EXISTS scan_results (
    id TEXT PRIMARY KEY,
    file_path TEXT NOT NULL,
    file_hash TEXT NOT NULL,
    scan_status TEXT NOT NULL, -- 'clean', 'infected', 'error', 'skipped'
    threat_name TEXT,
    scanner_used TEXT NOT NULL DEFAULT 'clamav',
    scanner_version TEXT,
    definitions_date TEXT,
    scan_duration_ms INTEGER,
    file_size INTEGER,
    scanned_at TEXT NOT NULL DEFAULT (datetime('now')),
    scanned_by TEXT -- user_id who initiated scan
);

CREATE INDEX IF NOT EXISTS idx_scan_results_file_path ON scan_results(file_path);
CREATE INDEX IF NOT EXISTS idx_scan_results_file_hash ON scan_results(file_hash);
CREATE INDEX IF NOT EXISTS idx_scan_results_scan_status ON scan_results(scan_status);
CREATE INDEX IF NOT EXISTS idx_scan_results_scanned_at ON scan_results(scanned_at);

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

-- Email accounts for attachment sync
CREATE TABLE IF NOT EXISTS email_accounts (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    email_address TEXT NOT NULL,
    protocol TEXT NOT NULL DEFAULT 'imap', -- 'imap', 'pop3'
    server TEXT NOT NULL,
    port INTEGER NOT NULL,
    username TEXT NOT NULL,
    password_encrypted TEXT NOT NULL,
    use_tls INTEGER NOT NULL DEFAULT 1,
    auto_fetch INTEGER NOT NULL DEFAULT 0,
    fetch_interval_minutes INTEGER NOT NULL DEFAULT 30,
    store_attachments INTEGER NOT NULL DEFAULT 1,
    attachment_folder TEXT DEFAULT '/email_attachments',
    last_fetch_at TEXT,
    last_fetch_status TEXT,
    last_fetch_error TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_email_accounts_user_id ON email_accounts(user_id);

-- Email messages (cached)
CREATE TABLE IF NOT EXISTS email_messages (
    id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL,
    message_id TEXT NOT NULL,
    subject TEXT,
    from_address TEXT,
    to_address TEXT,
    date TEXT,
    body_text TEXT,
    body_html TEXT,
    has_attachments INTEGER NOT NULL DEFAULT 0,
    is_read INTEGER NOT NULL DEFAULT 0,
    fetched_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (account_id) REFERENCES email_accounts(id) ON DELETE CASCADE,
    UNIQUE(account_id, message_id)
);

CREATE INDEX IF NOT EXISTS idx_email_messages_account_id ON email_messages(account_id);

-- Email attachments
CREATE TABLE IF NOT EXISTS email_attachments (
    id TEXT PRIMARY KEY,
    email_id TEXT NOT NULL,
    file_id TEXT NOT NULL,
    filename TEXT NOT NULL,
    content_type TEXT,
    size_bytes INTEGER,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (email_id) REFERENCES email_messages(id) ON DELETE CASCADE,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_email_attachments_email_id ON email_attachments(email_id);
