#![feature(let_chains)]
#[macro_use]
extern crate scripty_i18n;
#[macro_use]
extern crate tracing;

pub mod cmds;
mod i18n;

pub fn build_commands() -> Vec<poise::Command<Data, Error>> {
	let mut cmds = vec![
		cmds::register_cmds(),
		cmds::help(),
		cmds::join(),
		cmds::data_storage(),
		cmds::ping(),
		cmds::leave(),
		cmds::delete_all_data(),
		cmds::throw_error(),
		cmds::terms_of_service(),
		cmds::user_language(),
		poise::Command {
			subcommands: vec![cmds::block_user(), cmds::block_guild()],
			..cmds::block()
		},
		poise::Command {
			subcommands: vec![cmds::check_guilds(), cmds::hash_user_id()],
			..cmds::admin()
		},
		poise::Command {
			subcommands: vec![cmds::ps_close()],
			..cmds::ps()
		},
		poise::Command {
			subcommands: vec![
				cmds::premium::premium_remove(),
				cmds::premium::premium_claim(),
			],
			..cmds::premium::premium()
		},
		poise::Command {
			subcommands: vec![
				cmds::automod::automod_setup(),
				cmds::automod::automod_add_rule(),
				cmds::automod::automod_list_rules(),
				cmds::automod::automod_remove_rule(),
			],
			..cmds::automod::automod_root()
		},
		poise::Command {
			subcommands: vec![
				cmds::config::config_server_language(),
				cmds::config::config_transcribe_audio(),
				cmds::config::config_transcribe_video(),
				cmds::config::config_transcribe_voice_messages(),
				cmds::config::config_verbose(),
			],
			subcommand_required: true,
			..cmds::config::config_root()
		},
	];
	i18n::localize_commands(&mut cmds);
	cmds
}

use scripty_bot_utils::{Context, Data, Error};
