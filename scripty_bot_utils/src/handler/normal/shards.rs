use std::num::NonZeroU16;

use serenity::all::{Context, ShardStageUpdateEvent};

pub async fn shards_ready(ctx: &Context, total_shards: &NonZeroU16) {
	info!(
		"all {} shards have received ready event",
		total_shards.get()
	);

	crate::background_tasks::init_background_tasks(ctx.clone());
}

pub async fn shard_stage_update(
	_: &Context,
	ShardStageUpdateEvent { new, old, shard_id }: &ShardStageUpdateEvent,
) {
	info!(%shard_id, "shard {} transitioned from {} to {}", shard_id, old, new);
}
