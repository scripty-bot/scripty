use serenity::{all::ShardInfo, client::Context, model::prelude::Ready};

use crate::{
	dm_support::DmSupportStatus,
	extern_utils::set_cache_http,
	globals::{CLIENT_CACHE, DM_SUPPORT_GLOBAL},
};

pub async fn ready(
	ctx: Context,
	Ready {
		version,
		user,
		guilds,
		shard,
		..
	}: Ready,
) {
	set_cache_http(ctx.http.clone(), ctx.cache.clone());

	let _ = CLIENT_CACHE.set(ctx.cache.clone());

	let dm_support = DmSupportStatus::new();
	let _ = DM_SUPPORT_GLOBAL.set(dm_support);

	crate::background_tasks::init_background_tasks(ctx);

	if let Some(ShardInfo { id, total }) = shard {
		info!(
			"shard {} of {} ready: logged in as {}, in {} guilds, using API version {}",
			id,
			total,
			user.tag(),
			guilds.len(),
			version
		);
	} else {
		info!(
			"bot ready: logged in as {}, in {} guilds, using API version {}",
			user.tag(),
			guilds.len(),
			version
		);
	}
}
