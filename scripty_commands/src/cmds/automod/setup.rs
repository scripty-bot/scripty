use poise::CreateReply;
use scripty_premium::PremiumTierList;
use serenity::{
	builder::CreateEmbed,
	model::{channel::GuildChannel, mention::Mentionable, permissions::Permissions},
};

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

	#[description = "Should a recording of offending speech be sent to the target channel? \
	                 Defaults to false."]
	log_recording: Option<bool>,
) -> Result<(), Error> {
	let log_recording = log_recording.unwrap_or(false);

	let guild_id = ctx.guild_id().expect("asserted in guild").get();

	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(guild_id)).await;

	// filter and see if we have permissions to send messages, embed links, and attach files
	let target_permissions = {
		let bot_id = ctx.framework().bot_id();
		let bot_member = ctx
			.guild_id()
			.expect("asserted in guild")
			.member(&ctx, bot_id)
			.await?;
		ctx.guild()
			.expect("asserted in guild")
			.user_permissions_in(&target_channel, &bot_member)
	};
	let required_permissions =
		Permissions::SEND_MESSAGES | Permissions::EMBED_LINKS | Permissions::ATTACH_FILES;
	if !target_permissions.contains(required_permissions) {
		let missing_permissions = (!target_permissions) & required_permissions;
		ctx.say(format_message!(
			resolved_language,
			"automod-setup-invalid-channel-permissions",
			channelMention: target_channel.mention().to_string(),
			missingPermissions: missing_permissions.to_string()
		))
		.await?;
		return Ok(());
	}

	let db = scripty_db::get_db();

	let premium_tier = scripty_premium::get_guild(ctx.guild_id().unwrap().get()).await;
	let extra = if let Some(PremiumTierList::None) = premium_tier {
		format_message!(resolved_language, "automod-setup-embed-complete-free-limit")
	} else {
		String::new()
	};

	match sqlx::query!(
		"INSERT INTO automod_config 
            (guild_id, enabled, log_channel_id, log_recording)
        VALUES 
            ($1, true, $2, $3)
		ON CONFLICT (guild_id) DO UPDATE SET
			log_channel_id = $2,
			log_recording = $3
        ",
		guild_id as i64,
		target_channel.id.get() as i64,
		log_recording,
	)
	.execute(db)
	.await
	{
		Ok(_) => {}
		Err(sqlx::Error::Database(e)) if e.code() == Some("23503".into()) => {
			// guild not found, insert it
			sqlx::query!("INSERT INTO guilds (guild_id) VALUES ($1)", guild_id as i64)
				.execute(db)
				.await?;

			return ctx.rerun().await;
		}
		Err(e) => return Err(e.into()),
	}

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
