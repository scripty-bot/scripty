use std::time::Duration;

use serenity::{
	all::{Context, GuildId, Message, MessageFlags},
	builder::{CreateAllowedMentions, CreateAttachment, CreateMessage, EditMessage},
};

pub async fn handle_message(ctx: &Context, msg: Message) {
	if let Some(flags) = msg.flags
		&& flags.contains(MessageFlags::IS_VOICE_MESSAGE)
	{
		if let Some(attachment) = msg.attachments.first() {
			if let Some(duration_secs) = attachment.duration_secs {
				debug!(%msg.id, %duration_secs, "got voice message");
				if !voice_message_enabled_for_guild(msg.guild_id.unwrap()).await {
					debug!(%msg.id, "voice message not enabled for guild");
					return;
				}
				let new_msg = match msg
					.channel_id
					.send_message(
						&ctx,
						CreateMessage::new()
							.content("Downloading voice message...")
							.allowed_mentions(CreateAllowedMentions::new().replied_user(false))
							.reference_message(&msg),
					)
					.await
				{
					Ok(msg) => msg,
					Err(e) => {
						// likely can't send messages, so just ignore
						error!(%msg.id, "failed to send reply: {}", e);
						return;
					}
				};

				let res = match attachment.download().await {
					Ok(waveform) => {
						internal_handle_message(ctx, msg.clone(), new_msg, waveform).await
					}
					Err(e) => {
						error!(%msg.id, "failed to download voice message: {}", e);
						return;
					}
				};

				if let Err(e) = res {
					error!(%msg.id, "failed to handle voice message: {}", e);
					if let Err(e) = msg
						.reply(ctx, format!("failed to handle this voice message: {}", e))
						.await
					{
						error!(%msg.id, "failed to send error message: {}", e)
					}
				}
			}
		} else {
			warn!(%msg.id, "voice message did not contain attachment");
		}
	}
}

async fn internal_handle_message(
	ctx: &Context,
	msg: Message,
	mut new_msg: Message,
	waveform: Vec<u8>,
) -> Result<(), crate::Error> {
	new_msg
		.edit(
			&ctx,
			EditMessage::new().content("Decoding voice message..."),
		)
		.await?;

	debug!(%msg.id, "decoding voice message");
	// start by trying to decode the waveform: it should be 1 channel, 48000Hz, 32Kbps Opus in an OGG container
	let output = scripty_stt::decode_ogg_opus_file(waveform)?;
	// calculate length in seconds (fn above resamples to 16KHz, 1 channel)
	let output_length_secs = output.len() as f64 / 16000.0;

	debug!(%msg.id, "decoded voice message, feeding to speech-to-text");
	// fetch guild language
	let db = scripty_db::get_db();
	let res = sqlx::query!(
		"SELECT language, translate FROM guilds WHERE guild_id = $1",
		msg.guild_id.ok_or_else(crate::Error::expected_guild)?.get() as i64
	)
	.fetch_one(db)
	.await?;
	let lang = res.language;
	let translate = res.translate;

	let stream = scripty_stt::get_stream().await?;
	stream.feed_audio(output)?;
	debug!(%msg.id, "fed audio to speech-to-text, waiting up to {} seconds for result", output_length_secs * 2.0);
	let transcript = stream
		.get_result(
			lang,
			false,
			translate,
			Some(Duration::from_secs_f64(output_length_secs * 2.0)),
		)
		.await?;
	let transcript = transcript.trim();
	let mut msg_builder = EditMessage::new();

	if transcript.is_empty() {
		msg_builder = msg_builder.content("No transcription found");
	} else if transcript.len() > 1950 {
		msg_builder = msg_builder.new_attachment(CreateAttachment::bytes(
			transcript.to_string().into_bytes(),
			"transcript.txt",
		));
	} else {
		// send as a quote
		let mut quote = String::from("Transcript:\n");
		for line in transcript.split_inclusive('\n') {
			quote.push_str("> ");
			quote.push_str(line);
		}
		msg_builder = msg_builder.content(quote);
	}

	debug!(%msg.id, "got result from speech-to-text, sending to channel");
	new_msg.edit(&ctx, msg_builder).await?;

	debug!(%msg.id, "done handling voice message");
	Ok(())
}

pub async fn voice_message_enabled_for_guild(guild: GuildId) -> bool {
	// try to fetch from redis
	let redis_res = scripty_redis::run_transaction::<Option<bool>>("GET", |cmd| {
		cmd.arg(format!("msg_transcript_{}", guild.get()));
	})
	.await
	.unwrap_or_else(|e| {
		error!("failed fetching from redis: {}", e);
		None
	});
	if let Some(enabled) = redis_res {
		return enabled;
	};

	// fall back to DB fetch
	let db = scripty_db::get_db();
	match sqlx::query!(
		"SELECT transcribe_voice_messages FROM guilds WHERE guild_id = $1",
		guild.get() as i64
	)
	.fetch_optional(db)
	.await
	{
		Ok(Some(res)) => {
			let ret = res.transcribe_voice_messages;

			// cache in redis
			if let Err(e) = scripty_redis::run_transaction::<()>("SETEX", |cmd| {
				cmd.arg(format!("msg_transcript_{}", guild.get()))
					.arg(60 * 60 * 3)
					.arg(ret);
			})
			.await
			{
				error!("failed to cache in redis: {}", e);
			}
			false
		}
		Ok(None) => false,
		Err(e) => {
			error!("failed to fetch from DB: {}", e);
			false
		}
	}
}
