-- Add migration script here
-- language: postgresql

-- add a unique constraint to table automod_rules, on columns rule_type, rule_data, rule_action, and source_id
ALTER TABLE automod_rules ADD CONSTRAINT unique_rule UNIQUE (rule_type, rule_data, rule_action, source_id);
