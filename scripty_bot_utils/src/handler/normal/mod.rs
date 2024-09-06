mod cache;
mod message;
mod ready;
mod resume;
mod voice_state_update;

use serenity::{
	gateway::client::{Context, EventHandler as SerenityEventHandler},
	model::{
		channel::Message,
		event::ResumedEvent,
		gateway::Ready,
		id::GuildId,
		voice::VoiceState,
	},
};

pub struct EventHandler;

#[async_trait]
impl SerenityEventHandler for EventHandler {
	async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
		cache::cache_ready(ctx, guilds).await
	}

	async fn message(&self, ctx: Context, msg: Message) {
		message::message(ctx, msg).await
	}

	async fn ready(&self, ctx: Context, ready: Ready) {
		ready::ready(ctx, ready).await
	}

	async fn resume(&self, ctx: Context, _: ResumedEvent) {
		resume::resume(ctx).await
	}

	async fn voice_state_update(&self, ctx: Context, _old: Option<VoiceState>, new: VoiceState) {
		voice_state_update::voice_state_update(ctx, new).await
	}
}
