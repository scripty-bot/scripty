-- Add migration script here
ALTER TABLE guilds ADD COLUMN kiai_enabled BOOLEAN NOT NULL DEFAULT FALSE;
