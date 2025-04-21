mod cache;
mod message;
mod ready;
mod resume;
mod shards;
mod voice_state_update;

use serenity::gateway::client::{Context, EventHandler as SerenityEventHandler, FullEvent};

pub struct EventHandler;

#[async_trait]
impl SerenityEventHandler for EventHandler {
	async fn dispatch(&self, ctx: &Context, event: &FullEvent) {
		match event {
			FullEvent::CacheReady { guilds, .. } => cache::cache_ready(ctx, guilds).await,
			FullEvent::ShardsReady { total_shards, .. } => {
				shards::shards_ready(ctx, total_shards).await
			}
			FullEvent::Message {
				new_message: msg, ..
			} => message::message(ctx, msg).await,
			FullEvent::Ready {
				data_about_bot: ready,
				..
			} => ready::ready(ctx, ready).await,
			FullEvent::Resume { .. } => resume::resume(ctx).await,
			FullEvent::ShardStageUpdate { event, .. } => {
				shards::shard_stage_update(ctx, event).await
			}
			FullEvent::VoiceStateUpdate { new, .. } => {
				voice_state_update::voice_state_update(ctx, new).await
			}
			_ => {}
		}
	}
}
