-- stop errors when a user requests to have their data deleted and themselves banned
ALTER TABLE blocked_users
    DROP CONSTRAINT blocked_users_user_id_fkey;
