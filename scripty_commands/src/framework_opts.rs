use crate::cmds;
use crate::error::on_error;
use poise::{FrameworkOptions, PrefixFrameworkOptions};
use serenity::builder::{CreateAllowedMentions, ParseValue};
use serenity::prelude::GatewayIntents;

pub fn get_framework_opts() -> FrameworkOptions<crate::Data, crate::Error> {
    FrameworkOptions {
        commands: vec![
            cmds::setup(),
            cmds::register_cmds(),
            cmds::help(),
            cmds::join(),
            cmds::data_storage(),
            cmds::donate(),
            cmds::ping(),
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
        ],
        on_error: |error| Box::pin(on_error(error)),
        command_check: Some(crate::entity_block::check_block),
        pre_command: crate::handler::pre_command,
        post_command: crate::handler::post_command,
        allowed_mentions: Some({
            let mut f = CreateAllowedMentions::default();
            // Only support direct user pings by default
            f.empty_parse().parse(ParseValue::Users);
            f
        }),
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
            .map(|id| serenity::model::id::UserId(*id))
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
