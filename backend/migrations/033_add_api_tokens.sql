-- Personal Access Tokens (API Keys) for programmatic API access
-- Allows users to create tokens for scripts, CI/CD, and third-party integrations

CREATE TABLE IF NOT EXISTS api_tokens (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    token_hash TEXT NOT NULL UNIQUE,
    scopes TEXT NOT NULL DEFAULT '[]', -- JSON array of permission scopes
    expires_at TEXT, -- NULL means never expires
    last_used_at TEXT,
    is_revoked INTEGER DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Index for fast lookups by user
CREATE INDEX IF NOT EXISTS idx_api_tokens_user_id ON api_tokens(user_id);

-- Index for token authentication (hash lookup)
CREATE INDEX IF NOT EXISTS idx_api_tokens_hash ON api_tokens(token_hash);

-- Index for active tokens (not revoked, not expired)
CREATE INDEX IF NOT EXISTS idx_api_tokens_active ON api_tokens(user_id, is_revoked) 
    WHERE is_revoked = 0;
