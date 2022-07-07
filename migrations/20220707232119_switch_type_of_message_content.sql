-- language: postgres
ALTER TABLE message_store DROP COLUMN text;
ALTER TABLE message_store ADD COLUMN message_content bytea;
