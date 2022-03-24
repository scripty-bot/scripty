-- change the type of column language in users table from varchar(5) to text
ALTER TABLE users ALTER COLUMN language TYPE text;
-- change the type of column language in guilds table from varchar(5) to text
ALTER TABLE guilds ALTER COLUMN language TYPE text;
-- change the default value of column language in guilds table to 'en'
ALTER TABLE guilds ALTER COLUMN language SET DEFAULT 'en';