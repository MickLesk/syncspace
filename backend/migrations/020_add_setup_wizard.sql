-- First-Start Setup Wizard Support
-- Tracks if initial setup has been completed

-- Rename old system_settings table if it exists (different schema)
ALTER TABLE system_settings RENAME TO system_settings_old;

-- Create new system_settings table (singleton table with only one row)
CREATE TABLE IF NOT EXISTS system_settings (
    id INTEGER PRIMARY KEY,
    setup_completed INTEGER NOT NULL DEFAULT 0,
    server_name TEXT DEFAULT 'SyncSpace',
    server_description TEXT DEFAULT 'Self-hosted file synchronization',
    default_language TEXT DEFAULT 'en',
    allow_registration INTEGER NOT NULL DEFAULT 0,
    require_email_verification INTEGER NOT NULL DEFAULT 1,
    max_file_size_mb INTEGER DEFAULT 100,
    default_quota_gb INTEGER DEFAULT 10,
    session_timeout_minutes INTEGER DEFAULT 1440,
    enable_2fa_requirement INTEGER NOT NULL DEFAULT 0,
    password_min_length INTEGER DEFAULT 8,
    password_require_uppercase INTEGER NOT NULL DEFAULT 1,
    password_require_lowercase INTEGER NOT NULL DEFAULT 1,
    password_require_numbers INTEGER NOT NULL DEFAULT 1,
    password_require_special INTEGER NOT NULL DEFAULT 0,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Insert default settings (only one row allowed with id=1)
INSERT OR IGNORE INTO system_settings (id, setup_completed) VALUES (1, 0);

-- Index for fast lookup
CREATE INDEX IF NOT EXISTS idx_system_settings_setup ON system_settings(setup_completed);
