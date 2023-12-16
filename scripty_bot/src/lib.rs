//! This crate shall stay as small as possible, and only contain the
//! core code to run the bot. All other code should be in other crates.

mod framework_opts;

#[macro_use]
extern crate tracing;

use poise::FrameworkBuilder;
use scripty_audio_handler::SerenityInit;
use scripty_bot_utils::{
	extern_utils::set_cache_http,
	globals::{CLIENT_CACHE, CLIENT_DATA},
	handler,
	Data,
};
use serenity::{all::OnlineStatus, gateway::ActivityData};

pub async fn entrypoint() {
	// fetch the config
	let cfg = scripty_config::get_config();

	// initialize the blocked entity list
	info!("fetching blocked entities");
	scripty_bot_utils::entity_block::init_blocked()
		.await
		.expect("failed to init blocked entities");

	// initialize the framework
	let framework = FrameworkBuilder::default()
		.setup(move |ctx, _, c| {
			Box::pin(async move {
				set_cache_http(ctx.http.clone(), ctx.cache.clone());

				CLIENT_DATA
					.set(Data {
						shard_manager: c.shard_manager().clone(),
					})
					.expect("user data setup called more than once: bug?");
				CLIENT_CACHE
					.set(ctx.cache.clone())
					.expect("user data setup called more than once: bug?");

				let sm = c.shard_manager().clone();
				tokio::spawn(async move {
					tokio::signal::ctrl_c()
						.await
						.expect("failed to listen for ctrl+c");
					sm.shutdown_all().await;
				});

				Ok(Data {
					shard_manager: c.shard_manager().clone(),
				})
			})
		})
		.options(framework_opts::get_framework_opts())
		.build();

	let mut client = serenity::Client::builder(&cfg.token, framework_opts::get_gateway_intents())
		.framework(framework)
		.event_handler(handler::BotEventHandler)
		.raw_event_handler(handler::RawEventHandler)
		.register_songbird_from_config(scripty_audio_handler::get_songbird())
		.status(OnlineStatus::Idle)
		.activity(ActivityData::custom("Starting up..."))
		.await
		.expect("failed to create serenity client");

	client.start_autosharded().await.expect("failed to run bot");
}
