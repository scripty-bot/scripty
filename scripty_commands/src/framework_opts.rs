use crate::cmds;
use crate::error::on_error;
use poise::{FrameworkOptions, PrefixFrameworkOptions};
use serenity::builder::{CreateAllowedMentions, ParseValue};

pub fn get_framework_opts() -> FrameworkOptions<crate::Data, crate::Error> {
    FrameworkOptions {
        commands: vec![
            cmds::setup(),
            cmds::register_cmds(),
            cmds::help(),
            cmds::join(),
            cmds::train_storage(),
            cmds::donate(),
        ],
        on_error: |error| Box::pin(on_error(error)),
        command_check: Some(crate::entity_block::check_block),
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
        ..Default::default()
    }
}
