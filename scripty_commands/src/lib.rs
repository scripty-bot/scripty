#![feature(let_chains)]
#![feature(duration_millis_float)]
#[macro_use]
extern crate scripty_i18n;
#[macro_use]
extern crate tracing;

mod cmds;
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
		cmds::user_language(),
		cmds::vote_reminder(),
		cmds::transcribe_message(),
		cmds::transcribe_message_ctx_menu(),
		cmds::debug(),
		cmds::config::config_root(),
		cmds::entity_block::block_root(),
		cmds::admin::admin_root(),
		cmds::dm_support::ps_root(),
		cmds::premium::premium_root(),
		cmds::automod::automod_root(),
	];
	i18n::localize_commands(&mut cmds);
	cmds
}

use scripty_bot_utils::{Context, Data, Error};
