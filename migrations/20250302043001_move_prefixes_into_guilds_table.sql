DROP TABLE prefixes;

ALTER TABLE guilds
    ADD COLUMN prefix varchar(8) DEFAULT NULL;
