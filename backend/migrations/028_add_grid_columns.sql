-- Add grid_columns preference to user_preferences table
-- This allows users to configure how many columns are displayed in grid view (1-8)

ALTER TABLE user_preferences ADD COLUMN grid_columns INTEGER DEFAULT 4;

-- Update existing users with default value
UPDATE user_preferences SET grid_columns = 4 WHERE grid_columns IS NULL;
