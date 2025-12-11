-- 050: Add Google OAuth Provider Schema
-- Configure Google OAuth for authentication
-- NOTE: Credentials configured via .env.local file (not in git!)

-- This migration is intentionally left empty
-- OAuth providers are configured at runtime via environment variables
-- See backend/.env.example for Google OAuth configuration

-- Example .env.local setup:
-- GOOGLE_CLIENT_ID=your_client_id
-- GOOGLE_CLIENT_SECRET=your_client_secret
-- GOOGLE_REDIRECT_URI=http://localhost:8080/api/oauth/google/callback
