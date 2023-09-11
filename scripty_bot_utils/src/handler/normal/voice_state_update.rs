use scripty_audio_handler::{get_songbird, get_voice_channel_id};
use serenity::{all::VoiceState, client::Context};

pub async fn voice_state_update(ctx: Context, _: Option<VoiceState>, new: VoiceState) {
	let Some(gid) = new.guild_id else {
		warn!("no guild id in voice_state_update");
		return;
	};
	let Some(cid) = get_voice_channel_id(&ctx, gid).await else {
		debug!("not in a voice channel in guild {}", gid);
		return;
	};

	let own_user_id = ctx.cache.current_user().id;

	// GuildRef forces a block here to prevent hold over await
	{
		let guild = match gid.to_guild_cached(&ctx) {
			Some(g) => g,
			None => {
				warn!("guild id {} not found in cache", gid);
				return;
			}
		};

		// iterate through voice states in the guild
		// if there are any more than 1 in this channel, return
		// if there are 0, leave the channel
		for (_, vs) in guild.voice_states.iter() {
			if vs.channel_id == Some(cid) && vs.user_id != own_user_id {
				return;
			}
		}
	}

	// if we get here, we are the only one in the channel
	// so we should leave
	debug!(
		"leaving voice channel {} in guild {} (we're last user)",
		cid, gid
	);
	if let Err(e) = scripty_audio_handler::disconnect_from_vc(&ctx, gid).await {
		error!("error disconnecting from voice channel: {:?}", e);
	};
}
