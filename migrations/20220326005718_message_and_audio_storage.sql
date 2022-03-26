-- add new table named message_store
-- columns:
--  message_id: bigint primary key not null
--  author_id: bigint not null references users.user_id on delete cascade
--  text: text not null
CREATE TABLE IF NOT EXISTS message_store (
  message_id bigint primary key not null,
  author_id bigint not null,
  text text not null
);
-- add index on message_store.author_id
CREATE INDEX IF NOT EXISTS message_store_author_id_idx ON message_store (author_id);

-- add new table named audio_store
-- columns:
--  id: bigint primary key not null
--  source_id: bigint not null references users.user_id on delete cascade
--  audio_data: bytea not null comment 'audio data: 16 bit mono 16KHz WAV'
--  transcript: text not null
--  transcript_language: text not null
CREATE TABLE IF NOT EXISTS audio_store (
  id bigint primary key not null,
  source_id bigint not null,
  audio_data bytea not null,
  transcript text not null,
  transcript_language text not null
);
COMMENT ON COLUMN audio_store.audio_data IS 'audio data: 16 bit mono 16KHz WAV';
-- add index on audio_store.id
CREATE INDEX IF NOT EXISTS audio_store_id_idx ON audio_store (id);
-- add index on audio_store.source_id
CREATE INDEX IF NOT EXISTS audio_store_source_id_idx ON audio_store (source_id);

-- add new table named audio_transcript_verification
-- columns:
--  id: bigint primary key not null references audio_store.id on delete cascade
--  audio_matches_transcript: boolean not null
--  updated_transcript: text (comment 'user-provided transcript if audio_matches_transcript is false')
CREATE TABLE IF NOT EXISTS audio_transcript_verification (
  id bigint primary key not null,
  audio_matches_transcript boolean not null,
  updated_transcript text
);
COMMENT ON COLUMN audio_transcript_verification.updated_transcript IS
    'user-provided transcript if audio_matches_transcript is false';
-- add index on audio_transcript_verification.id
CREATE INDEX IF NOT EXISTS audio_transcript_verification_id_idx ON audio_transcript_verification (id);
