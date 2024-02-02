//! This crate shall stay as small as possible, and only contain the
//! core code to run the bot. All other code should be in other crates.

mod framework_opts;

#[macro_use]
extern crate tracing;

use std::sync::Arc;

use poise::FrameworkBuilder;
use scripty_bot_utils::{globals::CLIENT_DATA, handler, Data};
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
		.options(framework_opts::get_framework_opts())
		.build();
	CLIENT_DATA
		.set(Data {
			shard_manager: framework.shard_manager().clone(),
		})
		.expect("user data setup called more than once: bug?");
	let sm = framework.shard_manager().clone();
	tokio::spawn(async move {
		tokio::signal::ctrl_c()
			.await
			.expect("failed to listen for ctrl+c");
		sm.shutdown_all().await;
	});

	let mut client =
		serenity::Client::builder(&cfg.tokens.discord, framework_opts::get_gateway_intents())
			.data(Arc::new(Data {
				shard_manager: framework.shard_manager().clone(),
			}))
			.framework(framework)
			.event_handler(handler::BotEventHandler)
			.raw_event_handler(handler::RawEventHandler)
			.status(OnlineStatus::Idle)
			.activity(ActivityData::custom("Starting up..."))
			.await
			.expect("failed to create serenity client");

	client.start_autosharded().await.expect("failed to run bot");
}
