use scripty_bot_utils::{Context, Error};

#[poise::command(prefix_command, hide_in_help, owners_only)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
	let msg = ctx.say("shutting down").await?;

	// iterate over all active voice connections and notify
	let songbird = scripty_audio_handler::get_songbird();
	for (guild_id, call) in songbird.iter() {
		// leave the call
		call.lock().await.leave().await?;

		// notify the guild
	}

	Ok(())
}
