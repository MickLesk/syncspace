-- Migration: Add peers table for P2P synchronization
-- Created: 2025-11-25
-- Description: Store P2P peer information for distributed sync

CREATE TABLE IF NOT EXISTS peers (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    address TEXT NOT NULL UNIQUE,
    status TEXT NOT NULL DEFAULT 'offline', -- online, offline, syncing, error
    last_seen TEXT,
    sync_enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_peers_user_id ON peers(user_id);
CREATE INDEX IF NOT EXISTS idx_peers_status ON peers(status);
CREATE INDEX IF NOT EXISTS idx_peers_address ON peers(address);
CREATE INDEX IF NOT EXISTS idx_peers_created ON peers(created_at);
