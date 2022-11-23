-- Add migration script here
-- language: postgresql

-- add automod table
CREATE TABLE automod_config (
    item_id SERIAL PRIMARY KEY,
    guild_id bigint NOT NULL REFERENCES guilds (guild_id) ON DELETE CASCADE UNIQUE,
    enabled boolean NOT NULL DEFAULT false,
    log_channel_id bigint NOT NULL,

    -- if rule action is 2 or 3, log a recording of the message?
    log_recording boolean NOT NULL DEFAULT false,

    -- automatically join a voice channel if a user joins a voice channel?
    auto_join_voice boolean NOT NULL DEFAULT true
);

CREATE TABLE automod_rules (
    item_id SERIAL PRIMARY KEY,
    source_id bigint NOT NULL REFERENCES automod_config (item_id) ON DELETE CASCADE,

    -- rule type: currently only one type is supported, added for future expansion
    -- 1 for regular text match
    rule_type SMALLINT NOT NULL,

    -- rule data: any attached data to the rule, such as text to match
    rule_data TEXT NOT NULL,

    -- rule action: what to do when the rule is triggered
    -- 1 for silent delete
    -- 2 for delete and log
    -- 3 for delete, log, and remove user from voice
    rule_action SMALLINT NOT NULL
);
