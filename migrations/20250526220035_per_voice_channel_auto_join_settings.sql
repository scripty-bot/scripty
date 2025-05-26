ALTER TABLE scripty.public.per_voice_channel_settings
    ADD COLUMN auto_join_enabled BOOLEAN NOT NULL DEFAULT TRUE;
