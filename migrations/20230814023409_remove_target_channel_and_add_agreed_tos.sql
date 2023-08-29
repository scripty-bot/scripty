-- Add migration script here
ALTER TABLE guilds DROP COLUMN target_channel;
ALTER TABLE guilds ADD COLUMN agreed_tos boolean NOT NULL DEFAULT false;
