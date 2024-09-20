-- Add migration script here
ALTER TABLE guilds
    ALTER COLUMN translate SET DEFAULT FALSE;