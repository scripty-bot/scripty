use poise::CreateReply;
use scripty_premium::PremiumTierList;
use serenity::{builder::CreateEmbed, model::channel::GuildChannel};

use crate::{Context, Error};

/// Get started with Scripty's automod.
#[poise::command(
	prefix_command,
	slash_command,
	guild_only,
	required_permissions = "MANAGE_GUILD",
	rename = "setup"
)]
pub async fn automod_setup(
	ctx: Context<'_>,

	#[description = "The channel to send automod logs to"]
	#[channel_types("Text")]
	target_channel: GuildChannel,

	#[description = "Should a recording of offending speech be sent to the target channel? Defaults to false."]
	log_recording: Option<bool>,

	#[description = "Should the bot automatically join voice if a user joins? Defaults to true."]
	auto_join: Option<bool>,
) -> Result<(), Error> {
	let log_recording = log_recording.unwrap_or(false);
	let auto_join = auto_join.unwrap_or(true);

	let guild_id = ctx.guild_id().expect("asserted in guild").0;

	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.0, Some(guild_id)).await;

	let db = scripty_db::get_db();

	let premium_tier = scripty_premium::get_guild(ctx.guild_id().unwrap().0).await;
	let extra = if let Some(PremiumTierList::None) = premium_tier {
		format_message!(resolved_language, "automod-setup-embed-complete-free-limit")
	} else {
		"".to_string()
	};

	sqlx::query!(
		"INSERT INTO automod_config 
            (guild_id, enabled, log_channel_id, log_recording, auto_join_voice)
        VALUES 
            ($1, true, $2, $3, $4)
        ",
		guild_id.get() as i64,
		target_channel.id.get() as i64,
		log_recording,
		auto_join
	)
	.execute(db)
	.await?;

	ctx.send(
		CreateReply::default().embed(
			CreateEmbed::default()
				.title(format_message!(
					resolved_language,
					"automod-setup-embed-complete-title"
				))
				.description(format_message!(
					resolved_language,
					"automod-setup-embed-complete-description",
					extraDetails: extra,
					contextPrefix: ctx.prefix()
				)),
		),
	)
	.await?;

	Ok(())
}
