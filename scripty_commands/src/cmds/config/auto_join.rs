use scripty_bot_utils::Context;
use scripty_error::Error;
use serenity::model::{
	channel::{ChannelType, GuildChannel},
	mention::Mentionable,
};

/// Should Scripty automatically join a voice channel when someone joins it?
#[poise::command(
	prefix_command,
	slash_command,
	guild_only,
	required_permissions = "MANAGE_GUILD",
	rename = "auto_join"
)]
pub async fn config_auto_join(
	ctx: Context<'_>,
	#[description = "Defaults to false"] auto_join: bool,
	#[description = "Optionally set a channel this setting should apply to. If not set, the \
	                 server setting is modified."]
	#[channel_types("Voice", "Stage")]
	modify_channel: Option<GuildChannel>,
) -> Result<(), Error> {
	let guild_id = ctx
		.guild_id()
		.map(|g| g.get())
		.ok_or_else(Error::expected_guild)?;
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(guild_id)).await;
	let db = scripty_db::get_db();

	if let Some(ref modify_channel) = modify_channel
		&& !matches!(
			modify_channel.base.kind,
			ChannelType::Voice | ChannelType::Stage
		) {
		ctx.say(format_message!(
			resolved_language,
			"config-auto-join-modify-channel-must-be-vc",
			modifyChannelMention: modify_channel.mention().to_string()
		))
		.await?;
		return Ok(());
	}

	// check if the user needs to set a default channel first
	if auto_join {
		let target_channel_set = sqlx::query!(
			"SELECT target_channel FROM default_join_settings WHERE guild_id = $1",
			guild_id as i64
		)
		.fetch_optional(db)
		.await?
		.and_then(|row| row.target_channel)
		.is_some();

		if !target_channel_set {
			ctx.say(format_message!(
				resolved_language,
				"config-auto-join-needs-target-channel",
				contextPrefix: ctx.prefix()
			))
			.await?;
			return Ok(());
		}
	}

	let fmt_string = if let Some(modify_channel) = modify_channel {
		sqlx::query!(
			"INSERT INTO per_voice_channel_settings (channel_id, auto_join_enabled)
				VALUES ($1, $2)
				ON CONFLICT
				    ON CONSTRAINT per_voice_channel_settings_pkey
				    DO UPDATE SET auto_join_enabled = $2",
			modify_channel.id.get() as i64,
			auto_join
		)
		.execute(db)
		.await?;

		format_message!(
			resolved_language,
			if auto_join {
				"config-auto-join-enabled-channel"
			} else {
				"config-auto-join-disabled-channel"
			},
			modifyChannelMention: modify_channel.mention().to_string()
		)
	} else {
		sqlx::query!(
			"INSERT INTO guilds (guild_id, auto_join) VALUES ($1, $2) ON CONFLICT ON CONSTRAINT \
			 guilds_pkey DO UPDATE SET auto_join = $2",
			guild_id as i64,
			auto_join
		)
		.execute(db)
		.await?;

		format_message!(
			resolved_language,
			if auto_join {
				"config-auto-join-enabled"
			} else {
				"config-auto-join-disabled"
			}
		)
	};

	ctx.say(fmt_string).await?;

	Ok(())
}
