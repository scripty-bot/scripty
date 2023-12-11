mod auto_detect_lang;
mod language;
mod transcribe_audio;
mod transcribe_only_role;
mod transcribe_video;
mod transcribe_voice_messages;
mod translate;
mod verbose;

pub use auto_detect_lang::config_auto_detect_lang;
pub use language::config_server_language;
use poise::CreateReply;
use scripty_bot_utils::{checks::is_guild, Context, Error};
use serenity::builder::CreateEmbed;
pub use transcribe_audio::config_transcribe_audio;
pub use transcribe_only_role::config_transcribe_only_role;
pub use transcribe_video::config_transcribe_video;
pub use transcribe_voice_messages::config_transcribe_voice_messages;
pub use translate::config_translate;
pub use verbose::config_verbose;

/// Configure Scripty's settings
#[poise::command(
	prefix_command,
	slash_command,
	check = "is_guild",
	required_permissions = "MANAGE_GUILD",
	rename = "config"
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
