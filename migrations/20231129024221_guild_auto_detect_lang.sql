-- Add migration script here
ALTER TABLE guilds ADD COLUMN auto_detect_lang BOOLEAN NOT NULL DEFAULT FALSE;
