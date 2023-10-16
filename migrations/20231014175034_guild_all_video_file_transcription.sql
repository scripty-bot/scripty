-- Add migration script here
ALTER TABLE guilds ADD COLUMN transcribe_video_files BOOLEAN NOT NULL DEFAULT false;