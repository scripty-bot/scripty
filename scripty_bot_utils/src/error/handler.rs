#![allow(unused_variables)]

use std::{any::Any, borrow::Cow, fmt::Write, time::Duration};

use poise::{
	ApplicationContext,
	BoxFuture,
	CreateReply,
	FrameworkContext,
	FrameworkError,
	MessageDispatchTrigger,
	PartialContext,
	serenity_prelude::StatusCode,
};
use scripty_error::{Error, ErrorEnum};
use serenity::{
	all::{CommandInteraction, DiscordJsonError, InteractionResponseFlags, Message},
	builder::{
		CreateEmbed,
		CreateInteractionResponse,
		CreateInteractionResponseMessage,
		CreateMessage,
	},
	http,
	model::Permissions,
};
use tokio::sync::Mutex;

use crate::{
	Context,
	Data,
	error::{log_error_message, message::send_err_msg},
	types::InvocationData,
};

async fn _on_error(error: FrameworkError<'_, Data, Error>) {
	info!("handling error event");
	#[allow(unreachable_patterns)]
	let res = match error {
		FrameworkError::Command { ctx, error, .. } => handle_command_error(ctx, error).await,
		FrameworkError::ArgumentParse {
			error, input, ctx, ..
		} => handle_argparse_error(ctx, error, input).await,
		FrameworkError::CommandStructureMismatch {
			description, ctx, ..
		} => handle_csm_error(ctx, description).await,
		FrameworkError::CooldownHit {
			remaining_cooldown,
			ctx,
			..
		} => handle_cooldown_hit(ctx, remaining_cooldown).await,
		FrameworkError::MissingBotPermissions {
			missing_permissions,
			ctx,
			..
		} => handle_missing_bot_permission(ctx, missing_permissions).await,
		FrameworkError::MissingUserPermissions {
			missing_permissions,
			ctx,
			..
		} => handle_missing_user_permission(ctx, missing_permissions).await,
		FrameworkError::NotAnOwner { ctx, .. } => handle_not_an_owner(ctx).await,
		FrameworkError::CommandCheckFailed { error, ctx, .. } => {
			handle_command_check_failed(ctx, error).await
		}
		FrameworkError::SubcommandRequired { ctx } => handle_subcommand_required(ctx).await,
		FrameworkError::CommandPanic { ctx, payload, .. } => {
			handle_command_panic(ctx, payload).await
		}
		FrameworkError::GuildOnly { ctx, .. } => handle_guild_only(ctx).await,
		FrameworkError::DmOnly { ctx, .. } => handle_dm_only(ctx).await,
		FrameworkError::NsfwOnly { ctx, .. } => handle_nsfw_only(ctx).await,
		FrameworkError::DynamicPrefix {
			ctx, msg, error, ..
		} => handle_dynamic_prefix_error(ctx, msg, error).await,
		FrameworkError::UnknownCommand {
			framework,
			msg,
			prefix,
			msg_content,
			invocation_data,
			trigger,
			..
		} => {
			handle_unknown_command(
				framework,
				msg,
				prefix,
				msg_content,
				invocation_data,
				trigger,
			)
			.await
		}
		FrameworkError::UnknownInteraction {
			framework,
			interaction,
			..
		} => handle_unknown_interaction(framework, interaction).await,
		FrameworkError::NonCommandMessage {
			framework,
			msg,
			error,
			..
		} => handle_non_command_msg_error(framework, msg, error).await,
		FrameworkError::PermissionFetchFailed { ctx, .. } => {
			handle_permission_fetch_failure(ctx).await
		}
		FrameworkError::__NonExhaustive(_) => {
			unreachable!("__NonExhaustive is not supposed to be used")
		}
	};

	if let Err(e) = res {
		error!("error while handling error event: {}", e);
	}
}

pub fn on_error(error: FrameworkError<'_, Data, Error>) -> BoxFuture<'_, ()> {
	Box::pin(_on_error(error))
}

async fn handle_command_error(ctx: Context<'_>, error: Error) -> Result<(), Error> {
	if !error.should_handle() {
		return Ok(());
	}

	let cmd_name = &ctx.command().qualified_name;

	// if this is a 403 error, it's probably because the bot doesn't have permissions
	match error.peek_inner() {
		ErrorEnum::Serenity(serenity::Error::Http(http::HttpError::UnsuccessfulRequest(
			http::ErrorResponse {
				status_code,
				error: DiscordJsonError { code, message, .. },
				..
			},
		))) if status_code == &StatusCode::FORBIDDEN => {
			send_err_msg(
				ctx,
				format!("Missing permissions for {}!", cmd_name),
				format!(
					"I tried doing something (not sure what) but was not allowed to. Please check \
					 my permissions and try again.\nDiscord error code {:?}, message: `{}`",
					code, message
				),
			)
			.await;
		}
		ErrorEnum::Serenity(serenity::Error::Http(http::HttpError::UnsuccessfulRequest(
			http::ErrorResponse {
				status_code,
				error: DiscordJsonError { .. },
				..
			},
		))) if status_code == &StatusCode::BAD_GATEWAY => {
			send_err_msg(
				ctx,
				"Discord broke!",
				"There's nothing we can do about this. Try again in a few minutes. (502 Bad \
				 Gateway)",
			)
			.await;
		}

		_ if error.is_user_error() => {
			send_err_msg(
				ctx,
				format!("Invalid use of {}", cmd_name),
				error.to_string(),
			)
			.await;
		}

		ref e => {
			send_err_msg(
				ctx,
				format!("An error happened while processing {}", cmd_name),
				format!(
					"```\n{:?}\n```\nThis has been automatically reported. Please do not attempt \
					 to repeatedly use this command.",
					e
				),
			)
			.await;

			log_error_message(
				&ctx,
				error,
				Some(format!("running command {}", ctx.command().name)),
			)
			.await;
		}
	}

	Ok(())
}
async fn handle_argparse_error(
	ctx: Context<'_>,
	error: Box<dyn std::error::Error + Send + Sync>,
	input: Option<String>,
) -> Result<(), Error> {
	send_err_msg(
		ctx,
		format!(
			"Invalid arguments while parsing {}",
			ctx.command().qualified_name
		),
		match input {
			Some(input) => {
				format!(
					"Failed to parse `{}` because `{}`\n**Hint:** if you're trying to mention a \
					 channel with prefix commands, use its ID, as they are the most reliable way \
					 of doing so.",
					input, error
				)
			}
			None => format!("{}", error),
		},
	)
	.await;

	Ok(())
}

async fn handle_csm_error(
	ctx: ApplicationContext<'_, Data, Error>,
	description: &str,
) -> Result<(), Error> {
	let mut args = String::new();
	for param in &ctx.command.parameters {
		if param.required {
			write!(&mut args, "<{}> ", param.name).expect("failed to format string: this is a bug");
		} else {
			write!(&mut args, "[{}] ", param.name).expect("failed to format string: this is a bug");
		}
	}

	let msg = CreateMessage::default().embed(
		CreateEmbed::default()
			.title(format!(
				"Invalid structure from Discord while parsing {}",
				ctx.command.qualified_name
			))
			.color((255, 0, 0))
			.description(format!(
				"{}\n\n**Note**: this is a Discord error\nThe only fix for this is to wait for \
				 Discord to propagate slash commands, which can take up to one hour.\nIf you do \
				 not want to wait for this hour, you should use the prefix commands: run this \
				 command with `~{} {}`.",
				description, ctx.command.qualified_name, args
			)),
	);

	let response = ctx
		.interaction
		.channel_id
		.send_message(ctx.http(), msg.clone())
		.await;
	if let Err(e) = response {
		warn!("failed to send message while handling error: {}", e);
		let response = ctx
			.interaction
			.user
			.id
			.direct_message(ctx.http(), msg)
			.await;
		if let Err(e) = response {
			error!("failed to DM user while handling error: {}", e)
		}
	}

	Ok(())
}

async fn handle_cooldown_hit(ctx: Context<'_>, remaining_cooldown: Duration) -> Result<(), Error> {
	send_err_msg(
		ctx,
		format!("Cooldown hit on {}", ctx.command().qualified_name),
		format!(
			"{:.2} seconds remaining on cooldown",
			remaining_cooldown.as_secs_f32()
		),
	)
	.await;

	Ok(())
}

async fn handle_missing_bot_permission(
	ctx: Context<'_>,
	missing_permissions: Permissions,
) -> Result<(), Error> {
	send_err_msg(
		ctx,
		format!("I am missing perms to run {}", ctx.command().qualified_name),
		format!("Permissions missing: {}", missing_permissions),
	)
	.await;

	Ok(())
}

async fn handle_missing_user_permission(
	ctx: Context<'_>,
	missing_permissions: Option<Permissions>,
) -> Result<(), Error> {
	send_err_msg(
		ctx,
		format!(
			"You are missing perms to run {}",
			ctx.command().qualified_name
		),
		match missing_permissions {
			Some(p) => Cow::from(format!("Permissions missing: {}", p)),
			None => Cow::from("I'm not sure what permissions you're missing."),
		},
	)
	.await;

	Ok(())
}

async fn handle_not_an_owner(ctx: Context<'_>) -> Result<(), Error> {
	send_err_msg(
		ctx,
		format!(
			"You are missing perms to run {}",
			ctx.command().qualified_name
		),
		"Not an owner of this bot",
	)
	.await;

	Ok(())
}

async fn handle_command_check_failed(ctx: Context<'_>, error: Option<Error>) -> Result<(), Error> {
	send_err_msg(
		ctx,
		format!("A precondition for {} failed", ctx.command().qualified_name),
		match error {
			Some(e) => Cow::from(format!("{}", e)),
			None => Cow::from("no reason provided"),
		},
	)
	.await;

	Ok(())
}

async fn handle_subcommand_required(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;

	if let Err(e) = ctx
		.send(
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
		.await
	{
		error!("failed to send message: {:?}", e);
	}

	Ok(())
}

async fn handle_command_panic(ctx: Context<'_>, payload: Option<String>) -> Result<(), Error> {
	error!("Command panicked: {:?}", payload);

	Ok(())
}

async fn handle_guild_only(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;

	if let Err(e) = ctx
		.send(
			CreateReply::default().ephemeral(true).embed(
				CreateEmbed::new()
					.title(format_message!(
						resolved_language,
						"guild-only-command-invoked-title"
					))
					.description(format_message!(
						resolved_language,
						"guild-only-command-invoked-description"
					)),
			),
		)
		.await
	{
		error!("failed to send message: {:?}", e);
	}

	Ok(())
}

async fn handle_dm_only(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;

	if let Err(e) = ctx
		.send(
			CreateReply::default().ephemeral(true).embed(
				CreateEmbed::new()
					.title(format_message!(
						resolved_language,
						"dm-only-command-invoked-title"
					))
					.description(format_message!(
						resolved_language,
						"dm-only-command-invoked-description"
					)),
			),
		)
		.await
	{
		error!("failed to send message: {:?}", e);
	}

	Ok(())
}

async fn handle_nsfw_only(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language = &ctx
		.invocation_data::<InvocationData>()
		.await
		.ok_or_else(Error::expected_interaction_context)?
		.resolved_language;

	if let Err(e) = ctx
		.send(
			CreateReply::default().ephemeral(true).embed(
				CreateEmbed::new()
					.title(format_message!(
						resolved_language,
						"nsfw-only-command-invoked-title"
					))
					.description(format_message!(
						resolved_language,
						"nsfw-only-command-invoked-description"
					)),
			),
		)
		.await
	{
		error!("failed to send message: {:?}", e);
	}

	Ok(())
}

async fn handle_dynamic_prefix_error(
	ctx: PartialContext<'_, Data, Error>,
	msg: &Message,
	error: Error,
) -> Result<(), Error> {
	error!("Error while fetching prefix: {}", error);

	Ok(())
}

async fn handle_unknown_command(
	framework: FrameworkContext<'_, Data, Error>,
	msg: &Message,
	prefix: &str,
	msg_content: &str,
	invocation_data: &Mutex<Box<dyn Any + Send + Sync>>,
	trigger: MessageDispatchTrigger,
) -> Result<(), Error> {
	let bot_user = framework
		.bot_id()
		.to_user(&framework.serenity_context)
		.await?;

	if !msg_content.trim().is_empty()
		|| msg.mentions.len() != 1
		|| !msg.mentions.contains(&bot_user)
		|| trigger != MessageDispatchTrigger::MessageCreate
	{
		return Ok(());
	}
	match msg
		.channel_id
		.say(
			&framework.serenity_context.http,
			"To get started with Scripty, you can run </join:1179256548241973269> in a channel \
			 where you want transcriptions sent to.",
		)
		.await
	{
		Ok(_) => {}
		Err(serenity::Error::Http(e)) if e.status_code() == Some(StatusCode::FORBIDDEN) => {
			// DM user
			if let Err(e) = msg
				.author
				.id
				.direct_message(
					&framework.serenity_context.http,
					CreateMessage::new().content(format!(
						"I don't have permission to send messages in <#{}>! To get started with \
						 Scripty, you can run </join:1179256548241973269> in a channel where I do \
						 have permission to send messages.",
						msg.channel_id.get()
					)),
				)
				.await
			{
				warn!("failed to DM user: {:?}", e);
			}
		}
		Err(e) => {
			error!("failed to send message: {:?}", e);
		}
	}

	Ok(())
}

async fn handle_unknown_interaction(
	framework: FrameworkContext<'_, Data, Error>,
	interaction: &CommandInteraction,
) -> Result<(), Error> {
	error!("Unknown interaction: {:?}", interaction);
	interaction
		.create_response(
			&framework.serenity_context.http,
			CreateInteractionResponse::Message(
				CreateInteractionResponseMessage::new()
					.content("Internal error: unknown interaction")
					.flags(InteractionResponseFlags::EPHEMERAL),
			),
		)
		.await?;

	Ok(())
}

async fn handle_non_command_msg_error(
	framework: FrameworkContext<'_, Data, Error>,
	msg: &Message,
	error: Error,
) -> Result<(), Error> {
	error!("Error in non-command message handler: {:?}", error);

	Ok(())
}

async fn handle_permission_fetch_failure(ctx: Context<'_>) -> Result<(), Error> {
	if ctx
		.reply(
			"Couldn't fetch user permissions, this is a Discord problem likely. Try again later.",
		)
		.await
		.is_err()
	{
		let _ = ctx
			.author()
			.id
			.direct_message(
				&ctx,
				CreateMessage::default().content(
					"Couldn't send a message in the channel where you invoked me, so DMing you \
					 instead. I couldn't fetch your user permissions for that channel. This is \
					 likely a Discord problem. Try again later.",
				),
			)
			.await;
	}

	Ok(())
}
