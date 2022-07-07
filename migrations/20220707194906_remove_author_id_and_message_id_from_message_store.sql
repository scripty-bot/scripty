-- language: postgres
-- drop two identifying columns
ALTER TABLE message_store DROP COLUMN message_id;
ALTER TABLE message_store DROP COLUMN author_id;
-- replace the message_id column with a primary key that is independent of message IDs
ALTER TABLE message_store ADD COLUMN message_id BIGSERIAL PRIMARY KEY;
