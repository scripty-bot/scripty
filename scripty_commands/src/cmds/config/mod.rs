mod auto_detect_lang;
mod auto_join;
mod default_settings;
mod kiai_enabled;
mod language;
mod prefix;
mod transcribe_audio;
mod transcribe_only_role;
mod transcribe_video;
mod transcribe_voice_messages;
mod translate;
mod verbose;

use poise::CreateReply;
use scripty_bot_utils::Context;
use scripty_error::Error;
use serenity::builder::CreateEmbed;

/// Configure Scripty's settings
#[poise::command(
	prefix_command,
	slash_command,
	guild_only,
	required_permissions = "MANAGE_GUILD",
	rename = "config",
	subcommands(
		"auto_detect_lang::config_auto_detect_lang",
		"auto_join::config_auto_join",
		"default_settings::config_default_settings",
		"kiai_enabled::config_enable_kiai",
		"language::config_server_language",
		"prefix::config_prefix",
		"transcribe_audio::config_transcribe_audio",
		"transcribe_only_role::config_transcribe_only_role",
		"transcribe_video::config_transcribe_video",
		"transcribe_voice_messages::config_transcribe_voice_messages",
		"translate::config_translate",
		"verbose::config_verbose"
	),
	subcommand_required
)]
pub async fn config_root(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;

	ctx.send(
		CreateReply::default().ephemeral(true).embed(
			CreateEmbed::new()
				.title(format_message!(
					resolved_language,
					"root-command-invoked-title"
				))
				.description(format_message!(
					resolved_language,
					"root-command-invoked-description"
				)),
		),
	)
	.await?;

	Ok(())
}
