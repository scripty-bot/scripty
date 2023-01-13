use poise::{FrameworkOptions, PrefixFrameworkOptions};
use scripty_bot_utils::error::handler::on_error;
use scripty_commands::cmds;
use serenity::builder::CreateAllowedMentions;
use serenity::model::id::UserId;
use serenity::prelude::GatewayIntents;

pub fn get_framework_opts() -> FrameworkOptions<scripty_bot_utils::Data, scripty_bot_utils::Error> {
    FrameworkOptions {
        commands: vec![
            cmds::setup(),
            cmds::register_cmds(),
            cmds::help(),
            cmds::join(),
            cmds::data_storage(),
            cmds::donate(),
            cmds::ping(),
            cmds::leave(),
            cmds::delete_all_data(),
            cmds::throw_error(),
            poise::Command {
                subcommands: vec![cmds::user_language(), cmds::guild_language()],
                ..cmds::language()
            },
            poise::Command {
                subcommands: vec![cmds::block_user(), cmds::block_guild()],
                ..cmds::block()
            },
            poise::Command {
                subcommands: vec![cmds::check_guilds()],
                ..cmds::admin()
            },
            poise::Command {
                subcommands: vec![cmds::ps_close()],
                ..cmds::ps()
            },
            poise::Command {
                subcommands: vec![cmds::premium::remove(), cmds::premium::claim()],
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
        ],
        on_error: |error| Box::pin(on_error(error)),
        command_check: Some(scripty_bot_utils::entity_block::check_block),
        pre_command: scripty_bot_utils::handler::pre_command,
        post_command: scripty_bot_utils::handler::post_command,
        // Only support direct user pings by default
        allowed_mentions: Some(
            CreateAllowedMentions::default()
                .empty_roles()
                .empty_users()
                .replied_user(true),
        ),
        prefix_options: PrefixFrameworkOptions {
            prefix: Some("~".to_string()),
            execute_self_messages: false,
            execute_untracked_edits: true,
            mention_as_prefix: true,
            ..Default::default()
        },
        owners: scripty_config::get_config()
            .owners
            .iter()
            .map(|id| UserId::new(*id))
            .collect(),

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
