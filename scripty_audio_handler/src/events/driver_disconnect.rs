use std::{borrow::Cow, sync::Arc};

use serenity::{
	builder::{CreateAttachment, CreateMessage, ExecuteWebhook},
	gateway::client::Context,
	model::{
		id::{ChannelId, ThreadId, UserId},
		webhook::Webhook,
	},
};
use songbird::{events::context_data::DisconnectReason, id::GuildId, model::CloseCode};

use crate::{
	connect_to_vc,
	types::{SeenUsers, TranscriptResults},
};

pub async fn driver_disconnect(
	guild_id: GuildId,
	reason: Option<DisconnectReason>,
	ctx: Context,
	webhook: Arc<Webhook>,
	channel_id: ChannelId,
	voice_channel_id: ChannelId,
	thread_id: Option<ThreadId>,
	transcript_results: TranscriptResults,
	seen_users: SeenUsers,
	ephemeral: bool,
) {
	debug!(?guild_id, "handler disconnected");
	let (should_reconnect, reason) = match reason {
		Some(DisconnectReason::AttemptDiscarded) => {
			warn!(?guild_id, "reconnection failed due to another request");
			(false, None)
		}
		Some(DisconnectReason::Internal) => {
			error!(?guild_id, "disconnected due to songbird internal error");
			(true, Some("library internal error".into()))
		}
		Some(DisconnectReason::Io) => {
			warn!(?guild_id, "host IO error caused disconnection");
			(true, Some("host IO error".into()))
		}
		Some(DisconnectReason::ProtocolViolation) => {
			error!(
				?guild_id,
				"disconnected due to songbird and discord disagreeing on protocol"
			);
			(
				true,
				Some(
					"library and discord disagreed on protocol (this is a bug, please report!)"
						.into(),
				),
			)
		}
		Some(DisconnectReason::TimedOut) => {
			warn!(?guild_id, "timed out waiting for connection");
			(true, Some("timed out waiting for connection".into()))
		}
		Some(DisconnectReason::Requested) => {
			debug!(?guild_id, "requested disconnection");
			(false, None)
		}
		Some(DisconnectReason::WsClosed(None)) => {
			debug!(?guild_id, "voice session WebSocket closed without reason");
			(
				true,
				Some("discord closed connection without reason".into()),
			)
		}
		Some(DisconnectReason::WsClosed(Some(code))) => check_ws_close_err(code, guild_id),
		Some(_) => {
			warn!(?guild_id, "disconnected for unknown reason");
			(true, Some("disconnected for unknown reason".into()))
		}
		None => {
			debug!("requested disconnection from {}", guild_id);
			(false, None)
		}
	};

	if should_reconnect {
		debug!(?guild_id, "scheduling reconnect");
		// retry connection in 30 seconds
		let record_transcriptions = transcript_results.is_some();
		let webhook2 = webhook.clone();
		let ctx2 = ctx.clone();
		let ctx3 = ctx.clone();
		tokio::spawn(async move {
			debug!(?guild_id, "sleeping 30 seconds");
			tokio::time::sleep(std::time::Duration::from_secs(30)).await;
			debug!(?guild_id, "attempting reconnect");

			if let Err(e) = connect_to_vc(
				ctx2,
				serenity::all::GuildId::new(guild_id.get()),
				channel_id,
				voice_channel_id,
				thread_id,
				record_transcriptions,
				ephemeral,
			)
			.await && let Err(e) = webhook2
				.execute(
					&ctx3.http,
					false,
					ExecuteWebhook::default().content(format!("Failed to reconnect due to: {}", e)),
				)
				.await
			{
				debug!(
					?guild_id,
					"failed to notify user about reconnect failure: {}", e
				);
			}
		});
	}

	if let Some(reason) = reason {
		debug!(?guild_id, "giving user reason for disconnection");
		if let Err(e) = webhook
			.execute(
				&ctx.http,
				false,
				ExecuteWebhook::default().content(format!(
					"I had an issue ({}) and disconnected from the voice chat. {}",
					reason,
					if should_reconnect {
						"I'll try reconnecting in 30 seconds."
					} else {
						""
					}
				)),
			)
			.await
		{
			debug!(
				?guild_id,
				"failed to notify user about disconnection: {}", e
			);
		}
	}

	// send all users the results of their transcriptions
	if let (Some(transcript_results), Some(seen_users)) = (transcript_results, seen_users) {
		let final_text_output = transcript_results
			.read()
			.unwrap_or_else(|poisoned| {
				warn!(
					%guild_id,
					"transcript results poisoned, may be inconsistent"
				);
				poisoned.into_inner()
			})
			.join("\n");
		let attachment = CreateAttachment::bytes(final_text_output.into_bytes(), "transcript.txt");
		let message = CreateMessage::new().add_file(attachment.clone()).content(
			"This transcript was automatically sent to all users who spoke in the voice chat.",
		);
		for user in seen_users.iter() {
			if let Err(e) = UserId::new(*user)
				.direct_message(&ctx.http, message.clone())
				.await
			{
				debug!(?guild_id, "failed to send transcript to {}: {}", *user, e);
			}
		}

		// send the transcript to the channel
		if let Err(e) = webhook
			.execute(
				&ctx.http,
				false,
				ExecuteWebhook::new()
					.content(
						"This transcript was automatically sent to all users who spoke in the \
						 voice chat.",
					)
					.add_file(attachment),
			)
			.await
		{
			debug!(?guild_id, "failed to send transcript to channel: {}", e);
		}
	}
}

fn check_ws_close_err<'a>(reason: CloseCode, guild_id: GuildId) -> (bool, Option<Cow<'a, str>>) {
	match reason {
		CloseCode::UnknownOpcode => {
			error!(?guild_id, "voice session WebSocket closed: unknown opcode");
			(
				true,
				Some("discord closed connection due to unknown opcode".into()),
			)
		}
		CloseCode::InvalidPayload => {
			error!(?guild_id, "voice session WebSocket closed: invalid payload");
			(
				true,
				Some("discord closed connection due to an invalid payload".into()),
			)
		}
		CloseCode::NotAuthenticated => {
			error!(
				?guild_id,
				"voice session WebSocket closed: not authenticated"
			);
			(
				true,
				Some("discord closed connection due to not being authenticated".into()),
			)
		}
		CloseCode::AuthenticationFailed => {
			error!(
				?guild_id,
				"voice session WebSocket closed: failed to authenticate"
			);
			(
				true,
				Some("discord closed connection due to failing to authenticate".into()),
			)
		}
		CloseCode::AlreadyAuthenticated => {
			error!(
				?guild_id,
				"voice session WebSocket closed: already authenticated"
			);
			(
				true,
				Some("discord closed connection due to already being authenticated".into()),
			)
		}
		CloseCode::SessionInvalid => {
			error!(
				?guild_id,
				"voice session WebSocket closed: session no longer valid"
			);
			(true, Some("discord invalidated session".into()))
		}
		CloseCode::SessionTimeout => {
			error!(
				?guild_id,
				"voice session WebSocket closed: session timed out"
			);
			(true, Some("session timed out".into()))
		}
		CloseCode::ServerNotFound => {
			warn!(
				?guild_id,
				"voice session WebSocket closed: server not found"
			);
			(true, Some("voice server couldn't be found".into()))
		}
		CloseCode::UnknownProtocol => {
			warn!(
				?guild_id,
				"voice session WebSocket closed: protocol unrecognized"
			);
			(true, Some("discord didn't recognize protocol".into()))
		}
		CloseCode::Disconnected => {
			debug!(
				?guild_id,
				"voice session WebSocket closed: kicked/removed/deleted from channel"
			);
			(false, None)
		}
		CloseCode::VoiceServerCrash => {
			warn!(
				?guild_id,
				"voice session WebSocket closed: voice server crashed"
			);
			(true, Some("discord voice server crashed".into()))
		}
		CloseCode::UnknownEncryptionMode => {
			warn!(
				?guild_id,
				"voice session WebSocket closed: encryption scheme unrecognized"
			);
			(
				true,
				Some("discord didn't recognize encryption scheme".into()),
			)
		}
	}
}
