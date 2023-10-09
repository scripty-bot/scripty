use serenity::all::{Context, GuildId, Message, MessageFlags};

pub async fn handle_message(ctx: Context, msg: Message) {
	if let Some(flags) = msg.flags && flags.contains(MessageFlags::IS_VOICE_MESSAGE) {
			if let Some(attachment) = msg.attachments.first() {
				if let Some(duration_secs) = attachment.duration_secs {
					debug!(%msg.id, %duration_secs, "got voice message");

					if !voice_message_enabled_for_guild(msg.guild_id.unwrap()).await {
						debug!(%msg.id, "voice message not enabled for guild");
						return;
					}

					let res = match attachment.download().await {
						Ok(waveform) => internal_handle_message(&ctx, msg.clone(), waveform).await,
						Err(e) => {
							error!(%msg.id, "failed to download voice message: {}", e);
							return
						}
					};

					if let Err(e) = res {
						error!(%msg.id, "failed to handle voice message: {}", e);
						if let Err(e) = msg.reply(ctx, format!("failed to handle this voice message: {}", e)).await { error!(%msg.id, "failed to send error message: {}", e)}
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
	waveform: Vec<u8>,
) -> Result<(), crate::Error> {
	debug!(%msg.id, "decoding voice message");
	// start by trying to decode the waveform: it should be 1 channel, 48000Hz,32Kbps Opus in an OGG container
	let output = scripty_audio::decode_ogg_opus_file(waveform)?;

	debug!(%msg.id, "decoded voice message, feeding to speech-to-text");
	// fetch guild language
	let db = scripty_db::get_db();
	let lang = sqlx::query!(
		"SELECT language FROM guilds WHERE guild_id = $1",
		msg.guild_id.ok_or_else(crate::Error::expected_guild)?.get() as i64
	)
	.fetch_one(db)
	.await?
	.language;

	let stream = scripty_audio::get_stream(&lang, false).await?;
	stream.feed_audio(output)?;
	let res = stream.get_result().await?.result;

	debug!(%msg.id, "got result from speech-to-text, sending to channel");
	msg.reply(ctx, res).await?;

	debug!(%msg.id, "done handling voice message");
	Ok(())
}

pub async fn voice_message_enabled_for_guild(guild: GuildId) -> bool {
	// try to fetch from redis
	let redis_res = scripty_redis::run_transaction::<Option<bool>>("GET", |cmd| {
		cmd.arg(format!("msg_transcript_{}", guild.get()));
	})
	.await
	.map_or_else(
		|e| {
			error!("failed fetching from redis: {}", e);
			None
		},
		|r| r,
	);
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
