-- Migration 021: Add 2FA/TOTP Authentication Support
-- Creates tables for TOTP secrets, backup codes, and 2FA settings

-- 2FA secrets for users
CREATE TABLE IF NOT EXISTS user_2fa (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    totp_secret TEXT NOT NULL,
    totp_enabled INTEGER NOT NULL DEFAULT 0,
    backup_codes_generated INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    enabled_at TEXT,
    last_used_at TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_user_2fa_user_id ON user_2fa(user_id);
CREATE INDEX IF NOT EXISTS idx_user_2fa_enabled ON user_2fa(totp_enabled);

-- Backup codes for 2FA recovery
CREATE TABLE IF NOT EXISTS user_2fa_backup_codes (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    code_hash TEXT NOT NULL,
    used INTEGER NOT NULL DEFAULT 0,
    used_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_backup_codes_user_id ON user_2fa_backup_codes(user_id);
CREATE INDEX IF NOT EXISTS idx_backup_codes_used ON user_2fa_backup_codes(used);

-- 2FA audit log
CREATE TABLE IF NOT EXISTS user_2fa_audit (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    action TEXT NOT NULL, -- 'setup', 'enable', 'disable', 'verify_success', 'verify_failed', 'backup_used'
    ip_address TEXT,
    user_agent TEXT,
    success INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_2fa_audit_user_id ON user_2fa_audit(user_id);
CREATE INDEX IF NOT EXISTS idx_2fa_audit_action ON user_2fa_audit(action);
CREATE INDEX IF NOT EXISTS idx_2fa_audit_created ON user_2fa_audit(created_at);

-- Add 2FA requirement flag to users table
ALTER TABLE users ADD COLUMN requires_2fa INTEGER NOT NULL DEFAULT 0;
CREATE INDEX IF NOT EXISTS idx_users_requires_2fa ON users(requires_2fa);
