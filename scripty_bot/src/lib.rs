//! This crate shall stay as small as possible, and only contain the
//! core code to run the bot. All other code should be in other crates.

mod framework_opts;

#[macro_use]
extern crate tracing;

use std::sync::{Arc, OnceLock};

use poise::FrameworkBuilder;
use scripty_bot_utils::{globals::CLIENT_DATA, handler, Data};
use serenity::{all::OnlineStatus, client::ClientBuilder, gateway::ActivityData};

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
		.options(framework_opts::get_framework_opts())
		.build();
	let data = Arc::new(Data {
		shard_manager: OnceLock::new(),
	});
	CLIENT_DATA
		.set(data.clone())
		.expect("user data setup called more than once: bug?");

	let songbird = scripty_audio_handler::Songbird::serenity_from_config(
		scripty_audio_handler::get_songbird_config(),
	);
	scripty_audio_handler::set_songbird(songbird.clone());

	let mut http = serenity::http::HttpBuilder::new(&cfg.tokens.discord);
	if let Some(proxy) = &cfg.proxy {
		http = http.proxy(proxy).ratelimiter_disabled(true);
	}

	let mut client = ClientBuilder::new_with_http(
		Arc::new(http.build()),
		framework_opts::get_gateway_intents(),
	)
	.data(data.clone())
	.framework(framework)
	.voice_manager::<scripty_audio_handler::Songbird>(songbird)
	.event_handler(handler::BotEventHandler)
	.raw_event_handler(handler::RawEventHandler)
	.status(OnlineStatus::Idle)
	.activity(ActivityData::custom("Starting up..."))
	.await
	.expect("failed to create serenity client");

	data.shard_manager
		.set(client.shard_manager.clone())
		.expect("no other task should set shard manager");

	client.start_autosharded().await.expect("failed to run bot");
}
