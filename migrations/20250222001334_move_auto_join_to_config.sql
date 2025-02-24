ALTER TABLE guilds
    ADD COLUMN auto_join BOOLEAN NOT NULL DEFAULT FALSE;

-- automigrate data from automod_config to guilds
-- default false covers the other cases
UPDATE guilds
SET auto_join = true
WHERE guild_id IN (SELECT guild_id FROM automod_config WHERE auto_join_voice = true);

ALTER TABLE automod_config
    DROP COLUMN auto_join_voice;

-- clean up any servers that used automod only for the auto join function
DELETE
FROM automod_config AS r
WHERE NOT EXISTS (SELECT source_id FROM automod_rules WHERE source_id = r.item_id);
