ALTER TABLE blocked_users ALTER COLUMN blocked_since TYPE TIMESTAMP WITH TIME ZONE USING blocked_since::timestamp;
ALTER TABLE blocked_guilds ALTER COLUMN blocked_since TYPE TIMESTAMP WITH TIME ZONE USING blocked_since::timestamp;
