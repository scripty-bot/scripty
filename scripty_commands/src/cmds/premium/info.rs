use poise::CreateReply;
use scripty_bot_utils::checks::is_guild;
use scripty_premium::PremiumTierList;
use scripty_utils::hash_user_id;
use serenity::builder::CreateEmbed;

use crate::{Context, Error};

/// Check your Premium status in this server.
#[poise::command(prefix_command, slash_command, check = "is_guild", rename = "info")]
pub async fn premium_info(ctx: Context<'_>) -> Result<(), Error> {
	let guild_id = ctx.guild_id().ok_or_else(Error::expected_guild)?;
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(guild_id.get())).await;
	ctx.defer().await?;
	let db = scripty_db::get_db();
	let premium_level = scripty_premium::get_guild(guild_id.get())
		.await
		.unwrap_or(PremiumTierList::None);

	let mut embed_builder = CreateEmbed::new()
		.title(format_message!(
			resolved_language,
			"premium-info-embed-title"
		))
		.field(
			format_message!(resolved_language, "premium-info-embed-current-tier"),
			premium_level,
			false,
		)
		.field(
			format_message!(resolved_language, "premium-info-embed-max-users"),
			premium_level.max_users().to_string(),
			false,
		)
		.field(
			format_message!(resolved_language, "premium-info-embed-max-duration"),
			premium_level.max_duration().to_string(),
			false,
		)
		.field(
			format_message!(resolved_language, "premium-info-embed-max-file-length"),
			premium_level.max_file_length().to_string(),
			false,
		);

	if premium_level == PremiumTierList::None {
		let res = sqlx::query!(
			r#"SELECT trial_used, premium_owner_id IS NULL AS "premium_owner_id_is_null!" FROM 
			 guilds WHERE guild_id = $1"#,
			guild_id.get() as i64
		)
		.fetch_optional(db)
		.await?;
		let trial_used = res.as_ref().map_or(false, |row| row.trial_used);
		if !trial_used {
			embed_builder = embed_builder.field(
				format_message!(
					resolved_language,
					"premium-info-embed-trial-available-title"
				),
				format_message!(
					resolved_language,
					"premium-info-embed-trial-available-description"
				),
				false,
			);
		}

		if let Some(res) = res
			&& res.premium_owner_id_is_null
		{
			let user_has_premium = sqlx::query!(
				r#"SELECT coalesce(premium_level != 0, false) AS "has_premium!" FROM users WHERE user_id = $1"#,
				hash_user_id(ctx.author().id.get())
			)
			.fetch_optional(db)
			.await?
			.map_or(false, |row| row.has_premium);
			if user_has_premium {
				let claim_command = "`/premium claim`";
				embed_builder = embed_builder.field(
					format_message!(
						resolved_language,
						"premium-info-embed-manage-subscription-user-has-unclaimed-title"
					),
					format_message!(
						resolved_language,
						"premium-info-embed-manage-subscription-user-has-unclaimed-description",
						claimCommand: claim_command
					),
					false,
				);
			}
		}
	}

	let embed_builder = if premium_level == PremiumTierList::None {
		embed_builder.description(format_message!(
			resolved_language,
			"premium-info-embed-description-no-subscription"
		))
	} else {
		embed_builder.description(format_message!(
			resolved_language,
			"premium-info-embed-description-has-subscription"
		))
	};

	ctx.send(CreateReply::new().embed(embed_builder)).await?;

	Ok(())
}
