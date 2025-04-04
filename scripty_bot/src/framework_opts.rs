use std::{sync::Arc, time::Duration};

use poise::{EditTracker, FrameworkOptions, PrefixFrameworkOptions};
use scripty_bot_utils::{
	error::handler::on_error,
	handler::{post_command, pre_command},
};
use scripty_commands::build_commands;
use serenity::{builder::CreateAllowedMentions, model::id::UserId, prelude::GatewayIntents};

pub fn get_framework_opts() -> FrameworkOptions<scripty_bot_utils::Data, scripty_bot_utils::Error> {
	let commands = build_commands();

	FrameworkOptions {
		commands,
		on_error,
		pre_command,
		post_command,
		command_check: Some(scripty_bot_utils::entity_block::check_block),
		allowed_mentions: Some(
			CreateAllowedMentions::default()
				.empty_roles()
				.empty_users()
				.replied_user(true),
		),
		prefix_options: PrefixFrameworkOptions {
			dynamic_prefix: Some(scripty_bot_utils::prefix_handling::dynamic_prefix),
			execute_self_messages: false,
			execute_untracked_edits: true,
			mention_as_prefix: true,
			edit_tracker: Some(Arc::new(EditTracker::for_timespan(Duration::from_secs(
				120,
			)))),
			..Default::default()
		},
		owners: scripty_config::get_config()
			.owners
			.iter()
			.map(|id| UserId::new(*id))
			.collect(),
		skip_checks_for_owners: true,

		..Default::default()
	}
}

pub fn get_gateway_intents() -> GatewayIntents {
	GatewayIntents::GUILDS
		| GatewayIntents::GUILD_MEMBERS
		| GatewayIntents::GUILD_WEBHOOKS
		| GatewayIntents::GUILD_VOICE_STATES
		| GatewayIntents::GUILD_MESSAGES
		| GatewayIntents::DIRECT_MESSAGES
		| GatewayIntents::MESSAGE_CONTENT
}
