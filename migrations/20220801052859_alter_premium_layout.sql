-- language: postgres

-- drop unnecessary columns now
ALTER TABLE users DROP COLUMN github_account;
ALTER TABLE users DROP COLUMN used_servers;

ALTER TABLE guilds DROP COLUMN premium_level;


-- add new columns
ALTER TABLE users ADD COLUMN trial_used BOOLEAN DEFAULT FALSE NOT NULL;
ALTER TABLE guilds ADD COLUMN trial_used BOOLEAN DEFAULT FALSE NOT NULL;

ALTER TABLE users ADD COLUMN premium_expiry TIMESTAMP DEFAULT NULL;

ALTER TABLE guilds ADD COLUMN premium_owner_id BYTEA DEFAULT NULL REFERENCES users(user_id) ON DELETE SET NULL;
