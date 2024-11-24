use poise::CreateReply;
use serenity::builder::CreateAttachment;

use crate::{Context, Error};

/// Output some data useful for debugging Scripty
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn debug(ctx: Context<'_>) -> Result<(), Error> {
	let guild_id = ctx.guild_id().ok_or_else(Error::expected_guild)?;
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(guild_id.get())).await;
	let state = scripty_audio_handler::get_internal_state(&guild_id);
	if let Some(state) = state {
		ctx.send(
			CreateReply::new()
				.content(format_message!(resolved_language, "debug-info-message"))
				.attachment(CreateAttachment::bytes(
					format!("{:?}", state),
					Cow::Borrowed("debug_info.txt"),
				)),
		)
		.await?;
	} else {
		ctx.send(
			CreateReply::new().content(format_message!(resolved_language, "debug-not-in-call")),
		)
		.await?;
	}

	Ok(())
}
