use poise::CreateReply;
use serenity::builder::CreateEmbed;

use crate::{Context, Error};

/// Remove an automod rule.
#[poise::command(
	prefix_command,
	slash_command,
	guild_only,
	required_permissions = "MANAGE_GUILD",
	rename = "remove_rule"
)]
pub async fn automod_remove_rule(
	ctx: Context<'_>,
	#[description = "The rule ID to remove."] rule_id: i32,
) -> Result<(), Error> {
	// fetch the current guild's rule count
	let db = scripty_db::get_db();
	let gid = ctx.guild_id().expect("asserted in guild").get();
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(gid)).await;

	let source_id: i32 = match sqlx::query!(
		"SELECT item_id FROM automod_config WHERE guild_id = $1",
		gid as i64
	)
	.fetch_optional(db)
	.await?
	{
		Some(item) => item.item_id,
		None => {
			ctx.send(
				CreateReply::default().embed(
					CreateEmbed::default()
						.title(format_message!(
							resolved_language,
							"automod-remove-rule-embed-failure-title"
						))
						.description(format_message!(
							resolved_language,
							"automod-remove-rule-embed-failure-description-not-setup",
							contextPrefix: ctx.prefix()
						)),
				),
			)
			.await?;

			return Ok(());
		}
	};
	let count = sqlx::query!(
		r#"SELECT COUNT(*) AS "count!" FROM automod_rules WHERE source_id = $1"#,
		source_id
	)
	.fetch_one(db)
	.await?
	.count;

	// try to remove the rule
	if sqlx::query!(
		"DELETE FROM automod_rules WHERE source_id = $1 AND item_id = $2 RETURNING item_id",
		source_id,
		rule_id
	)
	.fetch_optional(db)
	.await?
	.is_none()
	{
		ctx.send(
			CreateReply::default().embed(
				CreateEmbed::default()
					.title(format_message!(
						resolved_language,
						"automod-remove-rule-embed-failure-title"
					))
					.description(format_message!(
						resolved_language,
						"automod-remove-rule-embed-failure-description-invalid-id",
						contextPrefix: ctx.prefix()
					)),
			),
		)
		.await?;

		return Ok(());
	};

	let premium_tier = scripty_premium::get_guild(gid).await.unwrap_or_default();

	// send success message
	ctx.send(
		CreateReply::default().embed(
			CreateEmbed::default()
				.title(format_message!(
					resolved_language,
					"automod-remove-rule-embed-success-title"
				))
				.description(format_message!(
					resolved_language,
					"automod-remove-rule-embed-success-description",
					rulesLeft: count - 1,
					maxRules: scripty_automod::utils::get_tier_rule_count(premium_tier)
				)),
		),
	)
	.await?;

	Ok(())
}
