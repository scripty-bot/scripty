-- Add migration script here
CREATE TABLE vote_reminders (
    user_id BIGINT NOT NULL,
    site_id SMALLINT NOT NULL,
    next_reminder TIMESTAMP NOT NULL,
    
    PRIMARY KEY (user_id, site_id)
);
