-- language: postgres
ALTER TABLE message_store ALTER COLUMN message_content SET NOT NULL;
ALTER TABLE message_store ALTER COLUMN message_content DROP DEFAULT;
