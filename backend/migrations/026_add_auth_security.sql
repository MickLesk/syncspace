-- Migration 026: Enhanced Authentication Security
-- Date: 2025-11-07
-- Description: Adds login attempt tracking, session management, and password policy

-- Login attempts tracking (for account lockout)
CREATE TABLE IF NOT EXISTS login_attempts (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    username TEXT NOT NULL,
    ip_address TEXT NOT NULL,
    user_agent TEXT,
    success INTEGER NOT NULL DEFAULT 0, -- 0=failed, 1=success
    failure_reason TEXT, -- "invalid_password", "account_locked", "2fa_failed", etc.
    attempted_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_login_attempts_username ON login_attempts(username);
CREATE INDEX IF NOT EXISTS idx_login_attempts_ip ON login_attempts(ip_address);
CREATE INDEX IF NOT EXISTS idx_login_attempts_attempted_at ON login_attempts(attempted_at);

-- User sessions (for session management)
CREATE TABLE IF NOT EXISTS user_sessions (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL,
    session_token TEXT NOT NULL UNIQUE, -- JWT token or session ID
    ip_address TEXT NOT NULL,
    user_agent TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    last_active_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT NOT NULL,
    revoked INTEGER NOT NULL DEFAULT 0, -- 0=active, 1=revoked
    revoked_at TEXT,
    revoked_reason TEXT, -- "user_logout", "admin_revoke", "suspicious_activity", etc.
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_user_sessions_user_id ON user_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_user_sessions_token ON user_sessions(session_token);
CREATE INDEX IF NOT EXISTS idx_user_sessions_expires_at ON user_sessions(expires_at);

-- Account lockout tracking (per user)
CREATE TABLE IF NOT EXISTS account_lockouts (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL,
    username TEXT NOT NULL,
    locked_at TEXT NOT NULL DEFAULT (datetime('now')),
    locked_until TEXT NOT NULL, -- 15 minutes from locked_at
    reason TEXT NOT NULL DEFAULT 'too_many_failed_attempts',
    failed_attempts_count INTEGER NOT NULL DEFAULT 0,
    unlocked_at TEXT, -- NULL if still locked
    unlocked_by TEXT, -- NULL=auto-unlock, user_id=admin unlock
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_account_lockouts_user_id ON account_lockouts(user_id);
CREATE INDEX IF NOT EXISTS idx_account_lockouts_username ON account_lockouts(username);
CREATE INDEX IF NOT EXISTS idx_account_lockouts_locked_until ON account_lockouts(locked_until);

-- Password history (prevent reusing last 5 passwords)
CREATE TABLE IF NOT EXISTS password_history (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    changed_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_password_history_user_id ON password_history(user_id);
CREATE INDEX IF NOT EXISTS idx_password_history_changed_at ON password_history(changed_at);

-- Add password policy fields to users table (if not exists)
ALTER TABLE users ADD COLUMN password_last_changed_at TEXT DEFAULT (datetime('now'));
ALTER TABLE users ADD COLUMN password_expires_at TEXT; -- NULL = never expires
ALTER TABLE users ADD COLUMN require_password_change INTEGER DEFAULT 0; -- Force password change on next login
ALTER TABLE users ADD COLUMN failed_login_attempts INTEGER DEFAULT 0; -- Counter for lockout
ALTER TABLE users ADD COLUMN last_failed_login_at TEXT; -- Track when last failed attempt occurred
ALTER TABLE users ADD COLUMN account_locked_until TEXT; -- NULL = not locked, datetime = locked until

-- Common/weak passwords blacklist (to prevent weak passwords)
CREATE TABLE IF NOT EXISTS password_blacklist (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    password_hash TEXT NOT NULL UNIQUE, -- Hash of common weak passwords
    added_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Pre-populate with some common weak passwords (hashed)
-- These are hashes of: "password", "123456", "qwerty", "admin", "letmein"
-- In production, you'd have a much larger list
INSERT OR IGNORE INTO password_blacklist (password_hash) VALUES 
    ('5f4dcc3b5aa765d61d8327deb882cf99'), -- "password"
    ('e10adc3949ba59abbe56e057f20f883e'), -- "123456"
    ('d8578edf8458ce06fbc5bb76a58c5ca4'), -- "qwerty"
    ('21232f297a57a5a743894a0e4a801fc3'), -- "admin"
    ('0d107d09f5bbe40cade3de5c71e9e9b7'); -- "letmein"
