-- Add Folder Colors Feature
-- Allows users to assign custom colors to folders for better organization

-- Add folder_color column to files table
ALTER TABLE files ADD COLUMN folder_color TEXT;

-- Create index for faster queries
CREATE INDEX IF NOT EXISTS idx_files_folder_color ON files(folder_color);

-- Add color preferences to user_preferences table if not exists
-- Note: user_preferences should already have a preferences JSON column
-- We'll store folder colors there as well
