-- OAuth2 Provider Configuration Table
CREATE TABLE IF NOT EXISTS oauth_providers (
    id TEXT PRIMARY KEY,
    provider TEXT NOT NULL UNIQUE,  -- 'google', 'github', 'microsoft', etc.
    client_id TEXT NOT NULL,
    client_secret_encrypted TEXT NOT NULL,
    redirect_uri TEXT NOT NULL,
    scopes TEXT NOT NULL,  -- JSON array as string
    enabled INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- OAuth2 Token Storage Table
CREATE TABLE IF NOT EXISTS oauth_tokens (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    provider TEXT NOT NULL,
    access_token_encrypted TEXT NOT NULL,
    refresh_token_encrypted TEXT,
    expires_at TEXT NOT NULL,
    scope TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Index for faster lookups
CREATE INDEX IF NOT EXISTS idx_oauth_tokens_user_provider ON oauth_tokens(user_id, provider);
CREATE INDEX IF NOT EXISTS idx_oauth_providers_enabled ON oauth_providers(enabled);

-- Insert default OAuth providers (disabled by default)
INSERT OR IGNORE INTO oauth_providers (id, provider, client_id, client_secret_encrypted, redirect_uri, scopes, enabled) VALUES
    ('google-provider', 'google', '', '', 'http://localhost:8080/api/auth/oauth/callback', '["openid", "profile", "email"]', 0),
    ('github-provider', 'github', '', '', 'http://localhost:8080/api/auth/oauth/callback', '["user:email"]', 0),
    ('microsoft-provider', 'microsoft', '', '', 'http://localhost:8080/api/auth/oauth/callback', '["openid", "profile", "email"]', 0);
