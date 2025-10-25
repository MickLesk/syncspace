-- Add user preferences columns for client-side settings
-- This allows storing all frontend preferences in the backend for multi-platform consistency

ALTER TABLE users ADD COLUMN view_mode TEXT DEFAULT 'grid';
ALTER TABLE users ADD COLUMN recent_searches TEXT DEFAULT '[]';
ALTER TABLE users ADD COLUMN sidebar_collapsed INTEGER DEFAULT 0;
ALTER TABLE users ADD COLUMN sort_by TEXT DEFAULT 'name';
ALTER TABLE users ADD COLUMN sort_order TEXT DEFAULT 'asc';
ALTER TABLE users ADD COLUMN auto_refresh INTEGER DEFAULT 1;
ALTER TABLE users ADD COLUMN upload_progress_visible INTEGER DEFAULT 1;

-- Update existing users with default values
UPDATE users SET 
    view_mode = 'grid',
    recent_searches = '[]',
    sidebar_collapsed = 0,
    sort_by = 'name',
    sort_order = 'asc',
    auto_refresh = 1,
    upload_progress_visible = 1
WHERE view_mode IS NULL;