-- Add migration script here
ALTER TABLE guilds ADD COLUMN translate BOOLEAN NOT NULL DEFAULT FALSE;
