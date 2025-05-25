CREATE TABLE per_voice_channel_settings
(
    channel_id     BIGINT NOT NULL PRIMARY KEY,
    target_channel BIGINT NULL DEFAULT NULL
);
