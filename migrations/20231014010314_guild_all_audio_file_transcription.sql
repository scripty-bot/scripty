-- Add migration script here
ALTER TABLE guilds ADD COLUMN transcribe_audio_files BOOLEAN NOT NULL DEFAULT FALSE;