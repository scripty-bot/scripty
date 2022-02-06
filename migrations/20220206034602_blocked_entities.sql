CREATE TABLE IF NOT EXISTS blocked_users (
    user_id BIGINT PRIMARY KEY,
    reason TEXT,
    blocked_since TIMESTAMP WITHOUT TIME ZONE
);

CREATE TABLE IF NOT EXISTS blocked_guilds (
    guild_id BIGINT PRIMARY KEY,
    reason TEXT,
    blocked_since TIMESTAMP WITHOUT TIME ZONE
);
