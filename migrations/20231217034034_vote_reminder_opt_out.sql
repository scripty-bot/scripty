-- Add migration script here
ALTER TABLE users ADD COLUMN vote_reminder_disabled BOOLEAN NOT NULL DEFAULT FALSE;
