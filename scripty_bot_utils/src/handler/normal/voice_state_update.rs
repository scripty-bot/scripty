use std::time::Duration;

use scripty_audio_handler::get_voice_channel_id;
use serenity::{
	all::{ChannelId, VoiceState},
	client::Context,
};

pub async fn voice_state_update(ctx: &Context, _old: &Option<VoiceState>, new: &VoiceState) {
	let Some(guild_id) = new.guild_id else {
		warn!("no guild id in voice_state_update");
		return;
	};

	if let Some(cid) = get_voice_channel_id(guild_id).await {
		let own_user_id = ctx.cache.current_user().id;

		// GuildRef forces a block here to prevent hold over await
		{
			let guild = match guild_id.to_guild_cached(&ctx.cache) {
				Some(g) => g,
				None => {
					warn!("guild id {} not found in cache", guild_id);
					return;
				}
			};

			// iterate through voice states in the guild
			// if there are any more than 1 in this channel, return
			// if there are 0, leave the channel
			let mut user_count = 0;
			for (_, vs) in guild.voice_states.iter() {
				// is the voice state in the channel we're in, and is it not us?
				if !(vs.channel_id == Some(cid) || vs.user_id != own_user_id) {
					continue;
				}
				// is the user a bot? if so, they don't count
				if guild
					.members
					.get(&vs.user_id)
					.map_or(false, |m| m.user.bot())
				{
					continue;
				}
				user_count += 1;
			}
			if user_count > 0 {
				debug!(
					"not leaving voice channel {} in guild {} ({} users)",
					cid, guild_id, user_count
				);
				return;
			}
		}

		// if we get here, we are the only one in the channel
		// so we should leave
		debug!(
			"leaving voice channel {} in guild {} (we're last user)",
			cid, guild_id
		);
		if let Err(e) = scripty_audio_handler::disconnect_from_vc(ctx, guild_id).await {
			error!("error disconnecting from voice channel: {:?}", e);
		};
	} else {
		debug!("not in a voice channel in guild {}", guild_id);

		// check if the guild has active premium
		let Some(_) = scripty_premium::get_guild(guild_id.get()).await else {
			// it does not, so we don't need to do anything
			return;
		};

		// does the guild have automod enabled?
		let db = scripty_db::get_db();
		let Some(resp) = (match sqlx::query!(
			"SELECT enabled, auto_join_voice, log_channel_id FROM automod_config WHERE guild_id = \
			 $1",
			guild_id.get() as i64
		)
		.fetch_optional(db)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				error!("error fetching automod config: {:?}", e);
				return;
			}
		}) else {
			// automod is not set up, so we don't need to do anything
			debug!(
				"automod not set up in guild {}, not continuing with join",
				guild_id
			);
			return;
		};
		if !(resp.enabled && resp.auto_join_voice) {
			// automod is not enabled, so we don't need to do anything
			debug!(
				"automod not enabled in guild {}, not continuing with join",
				guild_id
			);
			return;
		};

		let log_channel_id = ChannelId::new(resp.log_channel_id as u64);

		// is the target user a bot?
		let target_user = match new.user_id.to_user(&ctx).await {
			Ok(u) => u,
			Err(e) => {
				error!("error fetching user: {:?}", e);
				return;
			}
		};
		if target_user.bot() {
			debug!("user {} is a bot, not continuing with join", target_user.id);
			return;
		};

		// now we need to check the voice channel the user is joining
		// discord doesn't give us the channel id, so we need to get it from the guild's voice states
		let vs = {
			let guild = match guild_id.to_guild_cached(&ctx.cache) {
				Some(g) => g,
				None => {
					warn!("guild id {} not found in cache", guild_id);
					return;
				}
			};

			// fetch the user's voice state
			match guild.voice_states.get(&new.user_id) {
				Some(vs) => vs.clone(), // a relatively cheap clone, only one string internally
				None => {
					warn!("user id {} not found in guild voice states", new.user_id);
					return;
				}
			}
		};
		let Some(voice_channel_id) = vs.channel_id else {
			warn!("user id {} not in a voice channel", new.user_id);
			return;
		};

		// join the channel
		debug!(
			"joining voice channel {} in guild {} as guild has auto join enabled",
			voice_channel_id, guild_id
		);
		if let Err(e) = scripty_audio_handler::connect_to_vc(
			ctx.clone(),
			guild_id,
			log_channel_id,
			voice_channel_id,
			None,
			false,
			false,
		)
		.await
		{
			error!("error joining voice channel: {:?}", e);
			// fire a message to the log channel
			let _ = log_channel_id
				.say(
					&ctx.http,
					format!(
						"Failed to join voice channel due to auto-join error: {}\nYou may want to \
						 report this in our support server.",
						e
					),
				)
				.await;
		}
		// wait 1500ms as an additional buffer
		const FIFTEEN_HUNDRED_MS: Duration = Duration::from_millis(1500);
		tokio::time::sleep(FIFTEEN_HUNDRED_MS).await;
	};
}
