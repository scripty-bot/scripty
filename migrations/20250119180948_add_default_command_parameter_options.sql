CREATE TABLE IF NOT EXISTS default_join_settings
(
    guild_id              BIGINT                NOT NULL PRIMARY KEY REFERENCES guilds,
    record_transcriptions BOOLEAN DEFAULT FALSE NOT NULL,
    target_channel        BIGINT  DEFAULT NULL,
    new_thread            BOOLEAN DEFAULT FALSE NOT NULL,
    ephemeral             BOOLEAN DEFAULT FALSE NOT NULL
);
