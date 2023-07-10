use crate::types::{AutomodRule, AutomodServerConfig};

pub async fn get_guild_config(guild_id: u64) -> Result<Option<AutomodServerConfig>, sqlx::Error> {
	let db = scripty_db::get_db();
	let mut cfg = match sqlx::query!(
		"SELECT * FROM automod_config WHERE guild_id = $1",
		guild_id as i64
	)
	.fetch_optional(db)
	.await?
	.map(|row| {
		AutomodServerConfig::new(
			row.guild_id as u64,
			row.item_id,
			row.enabled,
			vec![],
			vec![],
			row.log_channel_id as u64,
			row.log_recording,
			row.auto_join_voice,
		)
	}) {
		Some(cfg) => cfg,
		None => return Ok(None),
	};

	// fetch rules
	// TODO: groups are currently not supported
	let res = sqlx::query!(
		"SELECT * FROM automod_rules WHERE source_id = $1",
		cfg.internal_id
	)
	.fetch_all(db)
	.await?;

	for rule in res {
		cfg.add_rule(AutomodRule {
			rule_type:   rule.rule_type.into(),
			rule_data:   rule.rule_data,
			rule_action: rule.rule_action.into(),
		});
	}
	Ok(Some(cfg))
}
