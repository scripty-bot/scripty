-- Add migration script here
ALTER TABLE guilds ADD COLUMN transcript_only_role BIGINT DEFAULT NULL;
