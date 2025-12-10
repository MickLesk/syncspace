-- OAuth providers configuration (may already exist from 012)
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

-- OAuth tokens for linked accounts (may already exist from 012)
-- We need to recreate if it doesn't have provider_user_id column
-- First check and add columns if missing
ALTER TABLE oauth_tokens ADD COLUMN provider_user_id TEXT;
ALTER TABLE oauth_tokens ADD COLUMN provider_email TEXT;

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

-- LDAP configurations (may already exist from 011)
-- Add missing columns
ALTER TABLE ldap_configs ADD COLUMN name TEXT;
ALTER TABLE ldap_configs ADD COLUMN user_filter TEXT DEFAULT '(uid={username})';
ALTER TABLE ldap_configs ADD COLUMN group_filter TEXT DEFAULT '(member={user_dn})';
ALTER TABLE ldap_configs ADD COLUMN username_attribute TEXT DEFAULT 'uid';
ALTER TABLE ldap_configs ADD COLUMN email_attribute TEXT DEFAULT 'mail';
ALTER TABLE ldap_configs ADD COLUMN display_name_attribute TEXT DEFAULT 'cn';
ALTER TABLE ldap_configs ADD COLUMN auto_create_users INTEGER DEFAULT 1;
ALTER TABLE ldap_configs ADD COLUMN default_role TEXT DEFAULT 'user';
ALTER TABLE ldap_configs ADD COLUMN group_role_mapping TEXT DEFAULT '{}';

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

-- Indexes for OAuth (only create if columns exist)
CREATE INDEX IF NOT EXISTS idx_oauth_tokens_user ON oauth_tokens(user_id);
CREATE INDEX IF NOT EXISTS idx_oauth_tokens_provider ON oauth_tokens(provider);
CREATE INDEX IF NOT EXISTS idx_oauth_states_state ON oauth_states(state);
CREATE INDEX IF NOT EXISTS idx_oauth_states_expires ON oauth_states(expires_at);

-- Indexes for LDAP
CREATE INDEX IF NOT EXISTS idx_ldap_configs_enabled ON ldap_configs(enabled);
CREATE INDEX IF NOT EXISTS idx_ldap_mappings_user ON ldap_user_mappings(user_id);
CREATE INDEX IF NOT EXISTS idx_ldap_mappings_config ON ldap_user_mappings(ldap_config_id);
CREATE INDEX IF NOT EXISTS idx_ldap_mappings_dn ON ldap_user_mappings(ldap_dn);
