-- Migration 010: Add integration features tables

-- System settings table
CREATE TABLE IF NOT EXISTS system_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Insert default settings
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES
    ('revision_safe_delete', 'false', 'Enable revision-safe delete (never permanently delete files)'),
    ('auto_thumbnail', 'true', 'Automatically generate thumbnails for images and videos'),
    ('virus_scan_on_upload', 'false', 'Scan files for viruses on upload'),
    ('max_file_size_mb', '5000', 'Maximum file size in megabytes'),
    ('enable_encryption', 'false', 'Enable file encryption'),
    ('enable_versioning', 'true', 'Enable file versioning'),
    ('retention_days', '30', 'Days to keep deleted files in trash'),
    ('enable_audit_log', 'true', 'Enable audit logging');

-- Email integration tables
CREATE TABLE IF NOT EXISTS email_accounts (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    email_address TEXT NOT NULL,
    protocol TEXT NOT NULL CHECK(protocol IN ('imap', 'pop3')),
    server TEXT NOT NULL,
    port INTEGER NOT NULL,
    username TEXT NOT NULL,
    password_encrypted TEXT NOT NULL,
    use_ssl INTEGER NOT NULL DEFAULT 1,
    auto_fetch INTEGER NOT NULL DEFAULT 0,
    fetch_interval_minutes INTEGER NOT NULL DEFAULT 15,
    last_fetch_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS email_messages (
    id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL,
    message_id TEXT NOT NULL,
    subject TEXT,
    sender TEXT NOT NULL,
    received_at TEXT NOT NULL,
    body_text TEXT,
    has_attachments INTEGER NOT NULL DEFAULT 0,
    attachment_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (account_id) REFERENCES email_accounts(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS email_attachments (
    id TEXT PRIMARY KEY,
    message_id TEXT NOT NULL,
    filename TEXT NOT NULL,
    file_id TEXT,
    file_size_bytes INTEGER NOT NULL,
    content_type TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (message_id) REFERENCES email_messages(id) ON DELETE CASCADE,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE SET NULL
);

-- S3 storage configurations
CREATE TABLE IF NOT EXISTS s3_configs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    provider TEXT NOT NULL CHECK(provider IN ('aws', 'minio', 'wasabi', 'backblaze', 'digitalocean')),
    endpoint TEXT,
    region TEXT NOT NULL,
    bucket TEXT NOT NULL,
    access_key TEXT NOT NULL,
    secret_key_encrypted TEXT NOT NULL,
    use_path_style INTEGER NOT NULL DEFAULT 0,
    is_default INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- FTP connections
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
    remote_path TEXT NOT NULL,
    local_path TEXT NOT NULL,
    sync_direction TEXT NOT NULL CHECK(sync_direction IN ('upload', 'download', 'bidirectional')),
    auto_sync INTEGER NOT NULL DEFAULT 0,
    sync_interval_minutes INTEGER NOT NULL DEFAULT 60,
    last_sync_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- LDAP configurations
CREATE TABLE IF NOT EXISTS ldap_configs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    server_url TEXT NOT NULL,
    base_dn TEXT NOT NULL,
    bind_dn TEXT NOT NULL,
    bind_password_encrypted TEXT NOT NULL,
    user_filter TEXT NOT NULL DEFAULT '(uid={username})',
    group_filter TEXT NOT NULL DEFAULT '(member={user_dn})',
    enabled INTEGER NOT NULL DEFAULT 1,
    auto_create_users INTEGER NOT NULL DEFAULT 1,
    default_role TEXT NOT NULL DEFAULT 'user',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Archives
CREATE TABLE IF NOT EXISTS archives (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    archive_type TEXT NOT NULL CHECK(archive_type IN ('zip', 'tar', 'tar.gz', 'tar.bz2')),
    file_path TEXT NOT NULL,
    original_paths TEXT NOT NULL,
    file_count INTEGER NOT NULL,
    total_size_bytes INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Compression rules
CREATE TABLE IF NOT EXISTS compression_rules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    file_pattern TEXT NOT NULL,
    min_age_days INTEGER NOT NULL,
    min_size_bytes INTEGER NOT NULL,
    compression_algorithm TEXT NOT NULL CHECK(compression_algorithm IN ('gzip', 'bzip2', 'zstd', 'lz4')),
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Compressed files
CREATE TABLE IF NOT EXISTS compressed_files (
    id TEXT PRIMARY KEY,
    original_file_id TEXT NOT NULL,
    original_path TEXT NOT NULL,
    compressed_path TEXT NOT NULL,
    original_size_bytes INTEGER NOT NULL,
    compressed_size_bytes INTEGER NOT NULL,
    compression_ratio REAL NOT NULL,
    algorithm TEXT NOT NULL,
    compressed_at TEXT NOT NULL DEFAULT (datetime('now')),
    is_decompressed INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (original_file_id) REFERENCES files(id) ON DELETE CASCADE
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_email_accounts_user ON email_accounts(user_id);
CREATE INDEX IF NOT EXISTS idx_email_messages_account ON email_messages(account_id);
CREATE INDEX IF NOT EXISTS idx_email_attachments_message ON email_attachments(message_id);
CREATE INDEX IF NOT EXISTS idx_ftp_connections_user ON ftp_connections(user_id);
CREATE INDEX IF NOT EXISTS idx_archives_user ON archives(user_id);
CREATE INDEX IF NOT EXISTS idx_compressed_files_original ON compressed_files(original_file_id);
CREATE INDEX IF NOT EXISTS idx_s3_configs_default ON s3_configs(is_default);
