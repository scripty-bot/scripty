use std::{borrow::Cow, fmt::Write};

use poise::{
	serenity_prelude::StatusCode,
	BoxFuture,
	CreateReply,
	FrameworkError,
	MessageDispatchTrigger,
};
use serenity::{
	all::{DiscordJsonError, InteractionResponseFlags},
	builder::{
		CreateEmbed,
		CreateInteractionResponse,
		CreateInteractionResponseMessage,
		CreateMessage,
	},
	http,
};

use crate::{
	error::{error_type::ErrorEnum, log_error_message, message::send_err_msg},
	types::InvocationData,
	Data,
	Error,
};

async fn _on_error(error: FrameworkError<'_, Data, Error>) {
	info!("handling error event");
	#[allow(unreachable_patterns)]
	match error {
		FrameworkError::Command { error, ctx, .. } => {
			if !error.should_handle() {
				return;
			}

			let cmd_name = &ctx.command().qualified_name;

			// if this is a 403 error, it's probably because the bot doesn't have permissions
			match error.err {
				ErrorEnum::Serenity(serenity::Error::Http(
					http::HttpError::UnsuccessfulRequest(http::ErrorResponse {
						status_code,
						error: DiscordJsonError { code, message, .. },
						..
					}),
				)) if status_code == http::StatusCode::FORBIDDEN => {
					send_err_msg(
						ctx,
						format!("Missing permissions for {}!", cmd_name),
						format!(
							"I tried doing something (not sure what) but was not allowed to. \
							 Please check my permissions and try again.\nDiscord error code {:?}, \
							 message: `{}`",
							code, message
						),
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
							"```\n{:?}\n```\nThis has been automatically reported. Please do not \
							 attempt to repeatedly use this command.",
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
		}
		FrameworkError::ArgumentParse {
			error, input, ctx, ..
		} => {
			send_err_msg(
				ctx,
				format!(
					"Invalid arguments while parsing {}",
					ctx.command().qualified_name
				),
				match input {
					Some(input) => {
						format!(
							"Failed to parse `{}` because `{}`\n**Hint:** if you're trying to \
							 mention a channel with prefix commands, use its ID, as they are the \
							 most reliable way of doing so.",
							input, error
						)
					}
					None => format!("{}", error),
				},
			)
			.await;
		}
		FrameworkError::CommandStructureMismatch {
			description, ctx, ..
		} => {
			let mut args = String::new();
			for param in &ctx.command.parameters {
				if param.required {
					write!(&mut args, "<{}> ", param.name)
						.expect("failed to format string: this is a bug");
				} else {
					write!(&mut args, "[{}] ", param.name)
						.expect("failed to format string: this is a bug");
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
						"{}\n\n**Note**: this is a Discord error\nThe only fix for this is to \
						 wait for Discord to propagate slash commands, which can take up to one \
						 hour.\nIf you do not want to wait for this hour, you should use the \
						 prefix commands: run this command with `~{} {}`.",
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
		}
		FrameworkError::CooldownHit {
			remaining_cooldown,
			ctx,
			..
		} => {
			send_err_msg(
				ctx,
				format!("Cooldown hit on {}", ctx.command().qualified_name),
				format!(
					"{:.2} seconds remaining on cooldown",
					remaining_cooldown.as_secs_f32()
				),
			)
			.await;
		}
		FrameworkError::MissingBotPermissions {
			missing_permissions,
			ctx,
			..
		} => {
			send_err_msg(
				ctx,
				format!("I am missing perms to run {}", ctx.command().qualified_name),
				format!("Permissions missing: {}", missing_permissions),
			)
			.await;
		}
		FrameworkError::MissingUserPermissions {
			missing_permissions,
			ctx,
			..
		} => {
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
		}
		FrameworkError::NotAnOwner { ctx, .. } => {
			send_err_msg(
				ctx,
				format!(
					"You are missing perms to run {}",
					ctx.command().qualified_name
				),
				"Not an owner of this bot",
			)
			.await;
		}
		FrameworkError::CommandCheckFailed { error, ctx, .. } => {
			send_err_msg(
				ctx,
				format!("A precondition for {} failed", ctx.command().qualified_name),
				match error {
					Some(e) => Cow::from(format!("{:?}", e.err)),
					None => Cow::from("no reason provided"),
				},
			)
			.await;
		}
		FrameworkError::EventHandler { error, event, .. } => {
			error!(?event, "Error in event handler: {:?}", error);
		}
		FrameworkError::SubcommandRequired { ctx } => {
			let resolved_language = scripty_i18n::get_resolved_language(
				ctx.author().id.get(),
				ctx.guild_id().map(|g| g.get()),
			)
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
		}
		FrameworkError::CommandPanic { payload, .. } => {
			error!("Command panicked: {:?}", payload);
		}
		FrameworkError::GuildOnly { ctx, .. } => {
			let resolved_language = scripty_i18n::get_resolved_language(
				ctx.author().id.get(),
				ctx.guild_id().map(|g| g.get()),
			)
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
		}
		FrameworkError::DmOnly { ctx, .. } => {
			let resolved_language = scripty_i18n::get_resolved_language(
				ctx.author().id.get(),
				ctx.guild_id().map(|g| g.get()),
			)
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
		}
		FrameworkError::NsfwOnly { ctx, .. } => {
			let resolved_language = &ctx
				.invocation_data::<InvocationData>()
				.await
				.expect("invocation data should be populated in pre_command hook")
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
		}
		FrameworkError::DynamicPrefix { error, .. } => {
			error!("Error while fetching prefix: {}", error);
		}
		FrameworkError::UnknownCommand {
			msg,
			msg_content,
			framework,
			trigger,
			..
		} => {
			if !msg_content.trim().is_empty()
				|| msg.mentions.len() != 1
				|| !msg.mentions.contains(
					&framework
						.bot_id()
						.to_user(&framework.serenity_context)
						.await
						.expect("should always be able to get bot user"),
				) || trigger != MessageDispatchTrigger::MessageCreate
			{
				return;
			}
			match msg
				.channel_id
				.say(
					&framework.serenity_context.http,
					"To get started with Scripty, you can run </join:1179256548241973269> in a \
					 channel where you want transcriptions sent to.",
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
								"I don't have permission to send messages in <#{}>! To get \
								 started with Scripty, you can run </join:1179256548241973269> in \
								 a channel where I do have permission to send messages.",
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
		}
		FrameworkError::UnknownInteraction {
			framework,
			interaction,
			..
		} => {
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
				.await
				.expect("failed to send response");
		}
		FrameworkError::NonCommandMessage { error, .. } => {
			error!("Error in non-command message handler: {:?}", error);
		}
		FrameworkError::__NonExhaustive(_) => {
			unreachable!("__NonExhaustive is not supposed to be used")
		}
	}
}

#[inline]
pub fn on_error(error: FrameworkError<'_, Data, Error>) -> BoxFuture<'_, ()> {
	Box::pin(_on_error(error))
}
