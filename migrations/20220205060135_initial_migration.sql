CREATE TABLE IF NOT EXISTS guilds (
    guild_id BIGINT PRIMARY KEY NOT NULL,
    target_channel BIGINT,
    -- later scripty will have language cfgs
    language CHAR(5),
    be_verbose BOOLEAN,
    premium_level SMALLINT
);

CREATE TABLE IF NOT EXISTS users (
    user_id BIGINT PRIMARY KEY NOT NULL,
    github_account TEXT,
    premium_level SMALLINT NOT NULL DEFAULT 0,
    used_servers SMALLINT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS prefixes (
    guild_id BIGINT PRIMARY KEY NOT NULL,
    prefix VARCHAR(10)
);

CREATE TABLE IF NOT EXISTS channels (
    channel_id BIGINT PRIMARY KEY NOT NULL,
    webhook_id BIGINT NOT NULL,
    webhook_token TEXT NOT NULL
);
