-- Migration 025: Add Refresh Tokens Table
-- Purpose: Store refresh tokens for JWT authentication with rotation and revocation support
-- Created: 2024-01-XX for JWT Refresh Token System

-- Create refresh_tokens table
CREATE TABLE IF NOT EXISTS refresh_tokens (
    id TEXT PRIMARY KEY,                    -- UUID for token identifier
    user_id TEXT NOT NULL,                  -- Foreign key to users table
    token_hash TEXT NOT NULL UNIQUE,        -- SHA256 hash of refresh token (for lookup)
    token_version INTEGER NOT NULL DEFAULT 1, -- Version number for token rotation
    expires_at TEXT NOT NULL,               -- ISO 8601 timestamp when token expires
    created_at TEXT NOT NULL,               -- ISO 8601 timestamp when token was created
    last_used_at TEXT,                      -- ISO 8601 timestamp when token was last used
    revoked_at TEXT,                        -- ISO 8601 timestamp when token was revoked (NULL if active)
    user_agent TEXT,                        -- User agent string for security tracking
    ip_address TEXT,                        -- IP address for security tracking
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Index for fast lookups by token hash
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_token_hash ON refresh_tokens(token_hash);

-- Index for fast lookups by user_id
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_user_id ON refresh_tokens(user_id);

-- Index for fast lookups by expiration (for cleanup)
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_expires_at ON refresh_tokens(expires_at);

-- Index for finding active tokens (not revoked and not expired)
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_active ON refresh_tokens(user_id, revoked_at, expires_at);

-- Add token_version to users table for global token invalidation
ALTER TABLE users ADD COLUMN token_version INTEGER NOT NULL DEFAULT 1;

-- Index for fast token version checks
CREATE INDEX IF NOT EXISTS idx_users_token_version ON users(id, token_version);
