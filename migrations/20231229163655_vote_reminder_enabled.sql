-- Add migration script here
ALTER TABLE users DROP COLUMN vote_reminder_disabled;
ALTER TABLE users ADD COLUMN vote_reminder_enabled boolean NOT NULL DEFAULT true;
