-- Migration 049: File Conversion Service
-- Add support for file format conversion with job tracking

-- Conversion jobs table
CREATE TABLE IF NOT EXISTS conversion_jobs (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL,
    source_file_path TEXT NOT NULL,
    source_format TEXT NOT NULL,  -- e.g., "pdf", "png", "mp4"
    target_format TEXT NOT NULL,  -- e.g., "docx", "webp", "webm"
    status TEXT NOT NULL DEFAULT 'pending',  -- pending, processing, completed, failed, cancelled
    output_file_path TEXT,  -- Path to converted file
    error_message TEXT,
    conversion_options TEXT,  -- JSON with quality, resolution, etc.
    progress INTEGER DEFAULT 0,  -- 0-100
    file_size_bytes INTEGER,  -- Size of output file
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CHECK (status IN ('pending', 'processing', 'completed', 'failed', 'cancelled')),
    CHECK (progress >= 0 AND progress <= 100)
);

-- Conversion presets table
CREATE TABLE IF NOT EXISTS conversion_presets (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    name TEXT NOT NULL,
    description TEXT,
    source_format TEXT NOT NULL,
    target_format TEXT NOT NULL,
    options TEXT NOT NULL,  -- JSON with default settings
    is_system BOOLEAN DEFAULT 0,  -- System presets can't be deleted
    created_by TEXT,  -- NULL for system presets
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_conversion_jobs_user_status ON conversion_jobs(user_id, status);
CREATE INDEX IF NOT EXISTS idx_conversion_jobs_status_created ON conversion_jobs(status, created_at);
CREATE INDEX IF NOT EXISTS idx_conversion_presets_formats ON conversion_presets(source_format, target_format);

-- Insert system presets for common conversions
INSERT INTO conversion_presets (id, name, description, source_format, target_format, options, is_system) VALUES
-- Image conversions
('preset_png_to_jpg', 'PNG to JPG (High Quality)', 'Convert PNG images to JPG with 95% quality', 'png', 'jpg', '{"quality": 95, "strip_metadata": false}', 1),
('preset_png_to_webp', 'PNG to WebP', 'Convert PNG to WebP for smaller file size', 'png', 'webp', '{"quality": 90, "lossless": false}', 1),
('preset_jpg_to_webp', 'JPG to WebP', 'Convert JPG to modern WebP format', 'jpg', 'webp', '{"quality": 90}', 1),
('preset_heic_to_jpg', 'HEIC to JPG', 'Convert iPhone photos to JPG', 'heic', 'jpg', '{"quality": 90}', 1),

-- Document conversions
('preset_pdf_to_docx', 'PDF to DOCX', 'Convert PDF to editable Word document', 'pdf', 'docx', '{"preserve_formatting": true}', 1),
('preset_docx_to_pdf', 'DOCX to PDF', 'Convert Word document to PDF', 'docx', 'pdf', '{"page_size": "A4", "dpi": 300, "embed_fonts": true}', 1),
('preset_txt_to_pdf', 'Text to PDF', 'Convert plain text to formatted PDF', 'txt', 'pdf', '{"page_size": "A4", "font": "Arial", "font_size": 12}', 1),
('preset_md_to_pdf', 'Markdown to PDF', 'Convert Markdown to formatted PDF', 'md', 'pdf', '{"page_size": "A4", "syntax_highlight": true}', 1),

-- Video conversions
('preset_mp4_to_webm', 'MP4 to WebM', 'Convert MP4 to WebM for web', 'mp4', 'webm', '{"codec": "libvpx-vp9", "quality": "medium", "audio_codec": "opus"}', 1),
('preset_avi_to_mp4', 'AVI to MP4', 'Convert AVI to MP4 (H.264)', 'avi', 'mp4', '{"codec": "libx264", "quality": "medium", "audio_codec": "aac"}', 1),
('preset_mov_to_mp4', 'MOV to MP4', 'Convert QuickTime MOV to MP4', 'mov', 'mp4', '{"codec": "libx264", "quality": "medium"}', 1),
('preset_video_to_gif', 'Video to GIF', 'Convert short video clip to animated GIF', 'mp4', 'gif', '{"fps": 10, "width": 480, "loop": 0}', 1),

-- Audio conversions
('preset_wav_to_mp3', 'WAV to MP3', 'Convert WAV audio to MP3', 'wav', 'mp3', '{"bitrate": "192k", "quality": "high"}', 1),
('preset_flac_to_mp3', 'FLAC to MP3', 'Convert lossless FLAC to MP3', 'flac', 'mp3', '{"bitrate": "320k", "quality": "ultra"}', 1),
('preset_m4a_to_mp3', 'M4A to MP3', 'Convert M4A/AAC to MP3', 'm4a', 'mp3', '{"bitrate": "192k"}', 1);

-- Note: Migration tracking is handled automatically by the backend
-- No manual INSERT into migrations_tracker needed

