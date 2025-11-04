-- Add Last Activity Visit Tracking
-- Tracks when users last viewed the Activity page for smart badge counting

-- Add column to users table
ALTER TABLE users ADD COLUMN last_activity_visit TIMESTAMP DEFAULT NULL;

-- Index for faster queries
CREATE INDEX IF NOT EXISTS idx_users_last_activity_visit ON users(last_activity_visit);
