-- Migration: Add encryption tables
-- Created: 2025-10-24

-- Encryption keys table
CREATE TABLE IF NOT EXISTS encryption_keys (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    key_hash TEXT NOT NULL, -- Encrypted master key
    salt TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    last_used_at TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_encryption_keys_user ON encryption_keys(user_id);
CREATE INDEX idx_encryption_keys_active ON encryption_keys(is_active);

-- Encrypted files tracking table
CREATE TABLE IF NOT EXISTS encrypted_files (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    key_id TEXT NOT NULL,
    nonce TEXT NOT NULL, -- Base64-encoded nonce for AES-GCM
    encrypted_metadata TEXT, -- Optional encrypted JSON metadata
    created_at TEXT NOT NULL,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (key_id) REFERENCES encryption_keys(id) ON DELETE CASCADE
);

CREATE INDEX idx_encrypted_files_file ON encrypted_files(file_id);
CREATE INDEX idx_encrypted_files_key ON encrypted_files(key_id);
CREATE UNIQUE INDEX idx_encrypted_files_file_unique ON encrypted_files(file_id);
