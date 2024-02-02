use poise::CreateReply;
use scripty_bot_utils::checks::is_guild;
use serenity::all::MessageFlags;

use crate::{Context, Error};

/// Transcribe the replied message.
#[poise::command(prefix_command, check = "is_guild")]
pub async fn transcribe_message(ctx: Context<'_>) -> Result<(), Error> {
	let ctx = match ctx {
		Context::Prefix(pctx) => pctx,
		Context::Application(actx) => {
			actx.send(
				CreateReply::default()
					.content(
						"This command is not available as a slash command. Use the prefix version \
						 instead",
					)
					.ephemeral(true),
			)
			.await?;
			return Ok(());
		}
	};

	let Some(target) = ctx.msg.referenced_message.to_owned() else {
		ctx.send(
			CreateReply::default()
				.reply(true)
				.content("You must reply to a message to transcribe it."),
		)
		.await?;
		return Ok(());
	};
	let mut target = *target;
	// thanks discord
	// message reference guild ID is always None,
	// so we have to snipe it from the real message
	target.guild_id = ctx.msg.guild_id;
	if target.guild_id.is_none() {
		ctx.send(
			CreateReply::default()
				.reply(true)
				.content("You can only transcribe messages in servers."),
		)
		.await?;
		return Ok(());
	}

	if target
		.flags
		.map_or(false, |f| f.contains(MessageFlags::IS_VOICE_MESSAGE))
	{
		scripty_bot_utils::voice_message::handle_message(ctx.serenity_context(), target).await;
	} else {
		scripty_bot_utils::generic_audio_message::handle_message(ctx.serenity_context(), target)
			.await?;
	}

	ctx.msg.delete(&ctx).await?;

	// the message gets sent in the above functions
	Ok(())
}
