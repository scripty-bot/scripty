use crate::{Context, Error};

/// Leave any current voice call.
#[poise::command(prefix_command, slash_command, guild_cooldown = 15, guild_only)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;

	let _typing = ctx.defer_or_broadcast().await;
	let guild_id = {
		let guild = ctx.guild().ok_or_else(Error::expected_guild)?;
		guild.id
	};

	scripty_audio_handler::disconnect_from_vc(ctx.serenity_context(), guild_id).await?;

	ctx.say(format_message!(resolved_language, "leave-success"))
		.await?;

	Ok(())
}
