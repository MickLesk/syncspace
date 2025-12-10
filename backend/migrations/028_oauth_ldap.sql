-- OAuth providers configuration
CREATE TABLE IF NOT EXISTS oauth_providers (
    id TEXT PRIMARY KEY,
    provider TEXT NOT NULL UNIQUE, -- 'google', 'github', 'microsoft'
    client_id TEXT NOT NULL,
    client_secret_encrypted TEXT NOT NULL,
    redirect_uri TEXT NOT NULL,
    scopes TEXT NOT NULL DEFAULT '[]', -- JSON array
    enabled INTEGER DEFAULT 0,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT
);

-- OAuth tokens for linked accounts
CREATE TABLE IF NOT EXISTS oauth_tokens (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    access_token_encrypted TEXT NOT NULL,
    refresh_token_encrypted TEXT,
    expires_at TEXT NOT NULL,
    scope TEXT NOT NULL DEFAULT '',
    provider_user_id TEXT,
    provider_email TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- OAuth state for CSRF protection
CREATE TABLE IF NOT EXISTS oauth_states (
    id TEXT PRIMARY KEY,
    state TEXT NOT NULL UNIQUE,
    provider TEXT NOT NULL,
    user_id TEXT REFERENCES users(id) ON DELETE CASCADE, -- NULL for login, set for linking
    redirect_url TEXT,
    expires_at TEXT NOT NULL,
    created_at TEXT DEFAULT (datetime('now'))
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
    username_attribute TEXT NOT NULL DEFAULT 'uid',
    email_attribute TEXT NOT NULL DEFAULT 'mail',
    display_name_attribute TEXT NOT NULL DEFAULT 'cn',
    enabled INTEGER DEFAULT 0,
    auto_create_users INTEGER DEFAULT 1,
    default_role TEXT DEFAULT 'user',
    group_role_mapping TEXT DEFAULT '{}', -- JSON: {"cn=admins": "admin"}
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT
);

-- LDAP user mappings
CREATE TABLE IF NOT EXISTS ldap_user_mappings (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    ldap_config_id TEXT NOT NULL REFERENCES ldap_configs(id) ON DELETE CASCADE,
    ldap_dn TEXT NOT NULL,
    ldap_username TEXT NOT NULL,
    synced_at TEXT DEFAULT (datetime('now')),
    UNIQUE(ldap_config_id, ldap_dn)
);

-- Indexes for OAuth
CREATE INDEX IF NOT EXISTS idx_oauth_tokens_user ON oauth_tokens(user_id);
CREATE INDEX IF NOT EXISTS idx_oauth_tokens_provider ON oauth_tokens(provider);
CREATE INDEX IF NOT EXISTS idx_oauth_tokens_provider_user ON oauth_tokens(provider, provider_user_id);
CREATE INDEX IF NOT EXISTS idx_oauth_states_state ON oauth_states(state);
CREATE INDEX IF NOT EXISTS idx_oauth_states_expires ON oauth_states(expires_at);

-- Indexes for LDAP
CREATE INDEX IF NOT EXISTS idx_ldap_configs_enabled ON ldap_configs(enabled);
CREATE INDEX IF NOT EXISTS idx_ldap_mappings_user ON ldap_user_mappings(user_id);
CREATE INDEX IF NOT EXISTS idx_ldap_mappings_config ON ldap_user_mappings(ldap_config_id);
CREATE INDEX IF NOT EXISTS idx_ldap_mappings_dn ON ldap_user_mappings(ldap_dn);
