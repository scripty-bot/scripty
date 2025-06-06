use poise::CreateReply;
use scripty_bot_utils::file_transcripts::transcribe_generic_message;
use serenity::{
	builder::{CreateAllowedMentions, CreateMessage},
	model::channel::Message,
};

use crate::{Context, Error};

/// Transcribe the replied message.
#[poise::command(prefix_command, guild_only)]
pub async fn transcribe_message(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|x| x.get()))
			.await;

	let ctx = match ctx {
		Context::Prefix(pctx) => pctx,
		Context::Application(actx) => {
			actx.send(
				CreateReply::default()
					.content(format_message!(
						resolved_language,
						"transcribe-message-not-slash-command"
					))
					.ephemeral(true),
			)
			.await?;
			return Ok(());
		}
	};

	let Some(target) = ctx.msg.referenced_message.clone() else {
		ctx.send(CreateReply::default().reply(true).content(format_message!(
			resolved_language,
			"transcribe-message-needs-reply"
		)))
		.await?;
		return Ok(());
	};
	let mut target_msg = *target;
	// thanks discord
	// message reference guild ID is always None,
	// so we have to snipe it from the real message
	target_msg.guild_id = ctx.msg.guild_id;

	let reply_msg = ctx
		.channel_id()
		.send_message(
			ctx.http(),
			CreateMessage::new()
				.content(format_message!(
					resolved_language,
					"transcribe-message-initial-reply"
				))
				.reference_message(&target_msg)
				.allowed_mentions(
					CreateAllowedMentions::new()
						.everyone(false)
						.replied_user(false),
				),
		)
		.await?;
	ctx.msg.delete(ctx.http(), None).await?;

	transcribe_generic_message(
		target_msg,
		(ctx.serenity_context().clone(), reply_msg).into(),
		Some(ctx.author().id),
		resolved_language,
	)
	.await?;

	Ok(())
}

#[poise::command(
	context_menu_command = "Transcribe Message",
	install_context = "Guild|User",
	interaction_context = "Guild|BotDm|PrivateChannel",
	user_cooldown = 15
)]
pub async fn transcribe_message_ctx_menu(
	ctx: Context<'_>,
	target_msg: Message,
) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|x| x.get()))
			.await;

	let reply_msg = ctx
		.reply(format_message!(
			resolved_language,
			"transcribe-message-initial-reply"
		))
		.await?;

	transcribe_generic_message(
		target_msg,
		(ctx, reply_msg).into(),
		Some(ctx.author().id),
		resolved_language,
	)
	.await
}
