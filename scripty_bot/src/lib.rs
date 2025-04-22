//! This crate shall stay as small as possible, and only contain the
//! core code to run the bot. All other code should be in other crates.

mod framework_opts;

#[macro_use]
extern crate tracing;

use std::{str::FromStr, sync::Arc};

use poise::FrameworkBuilder;
use scripty_bot_utils::{Data, globals::CLIENT_DATA, handler};
use serenity::{
	gateway::{ActivityData, TransportCompression, client::ClientBuilder},
	model::user::OnlineStatus,
	secrets::Token,
};

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
	let data = Arc::new(Data::new());
	if CLIENT_DATA.set(data.clone()).is_err() {
		unreachable!("client data set more than once: bug?")
	}

	let songbird = scripty_audio_handler::Songbird::serenity_from_config(
		scripty_audio_handler::get_songbird_config(),
	);
	scripty_audio_handler::set_songbird(songbird.clone());

	let token = Token::from_str(&cfg.tokens.discord).expect("failed to parse token");

	let mut http = serenity::http::HttpBuilder::new(token.clone());
	if let Some(proxy) = &cfg.proxy {
		http = http.proxy(proxy).ratelimiter_disabled(true);
	}
	let http = http.build();
	if let Some(ratelimiter) = &http.ratelimiter {
		ratelimiter.set_ratelimit_callback(Box::new(handler::ratelimit));
	}

	let mut client =
		ClientBuilder::new_with_http(token, Arc::new(http), framework_opts::get_gateway_intents())
			.compression(TransportCompression::None)
			.data(data.clone())
			.framework(framework)
			.voice_manager::<scripty_audio_handler::Songbird>(songbird)
			.event_handler(handler::EventHandler)
			.raw_event_handler(handler::RawEventHandler)
			.status(OnlineStatus::Idle)
			.activity(ActivityData::custom("Starting up..."))
			.await
			.expect("failed to create serenity client");

	data.shard_runners
		.set(client.shard_manager.runners.clone())
		.expect("no other task should set shard manager");

	client.start_autosharded().await.expect("failed to run bot");
}
