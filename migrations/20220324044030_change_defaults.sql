-- remove not null constraint on column "target_channel" in table "guilds"
ALTER TABLE guilds ALTER COLUMN target_channel DROP NOT NULL;
-- remove default value on column "target_channel" in table "guilds"
ALTER TABLE guilds ALTER COLUMN target_channel DROP DEFAULT;

-- set default value on column "language" in table "guilds" to "en"
ALTER TABLE guilds ALTER COLUMN language SET DEFAULT 'en';

-- set default value on column "be_verbose" in table "guilds" to false
ALTER TABLE guilds ALTER COLUMN be_verbose SET DEFAULT false;

-- set default value on column "premium_level" in table "guilds" to 0
ALTER TABLE guilds ALTER COLUMN premium_level SET DEFAULT 0;
