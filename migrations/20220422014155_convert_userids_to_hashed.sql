-- user IDs are now going to be hashed with sha512
-- targets:
--  - audio_store (source_id)
--  - message_store (author_id)
--  - blocked_users (user_id)
--  - users (user_id)

-- the existing type is bigint, which is difficult to migrate to bytea
-- so just drop and recreate the two columns
ALTER TABLE audio_store DROP COLUMN source_id;
ALTER TABLE message_store DROP COLUMN author_id;
ALTER TABLE blocked_users DROP COLUMN user_id;
ALTER TABLE users DROP COLUMN user_id;

-- recreate the columns
ALTER TABLE users ADD COLUMN user_id bytea NOT NULL PRIMARY KEY;
ALTER TABLE audio_store ADD COLUMN source_id bytea NOT NULL REFERENCES users(user_id) ON DELETE CASCADE;
ALTER TABLE message_store ADD COLUMN author_id bytea NOT NULL REFERENCES users(user_id) ON DELETE CASCADE;
ALTER TABLE blocked_users ADD COLUMN user_id bytea NOT NULL REFERENCES users(user_id) ON DELETE CASCADE PRIMARY KEY;
