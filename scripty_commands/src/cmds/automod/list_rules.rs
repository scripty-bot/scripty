use scripty_utils::do_paginate;

use crate::{Context, Error};

#[poise::command(
	prefix_command,
	slash_command,
	guild_only,
	required_permissions = "MANAGE_GUILD",
	rename = "list_rules"
)]
pub async fn automod_list_rules(ctx: Context<'_>) -> Result<(), Error> {
	// fetch the current guild's rule count
	let db = scripty_db::get_db();
	let gid = ctx.guild_id().expect("asserted in guild").0;
	let resolved_language = scripty_i18n::get_resolved_language(ctx.author().id.0, Some(gid)).await;

	let rules: Vec<_> = sqlx::query!(
        "SELECT item_id, rule_type, rule_action, rule_data FROM automod_rules WHERE source_id = (SELECT item_id FROM automod_config WHERE guild_id = $1) ORDER BY item_id ASC",
        gid.get() as i64
    ).fetch_all(db).await?;

	if rules.is_empty() {
		ctx.say(format_message!(
			resolved_language,
			"automod-list-rules-no-rules"
		))
		.await?;
		return Ok(()); // no rules
	}

	let formatted_rules = rules
		.into_iter()
		.map(|x| {
			(
				format_message!(resolved_language, "automod-list-rules-embed-field-name", ruleId: x.item_id),
				format_message!(
					resolved_language,
					"automod-list-rules-embed-field-value",
					ruleType: x.rule_type,
					ruleAction: x.rule_action,
					ruleContent: x.rule_data
				),
			)
		})
		.collect::<Vec<_>>();

	do_paginate(
		ctx.serenity_context(),
		ctx.channel_id(),
		formatted_rules,
		format_message!(resolved_language, "automod-list-rules-embed-title"),
		None,
		None,
		Some(ctx.author().id),
	)
	.await?;

	Ok(())
}
