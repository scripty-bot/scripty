-- language: psql
-- alter column id on table audio_store to be auto-incrementing
ALTER TABLE audio_store DROP COLUMN id;
ALTER TABLE audio_store ADD COLUMN id SERIAL PRIMARY KEY;
