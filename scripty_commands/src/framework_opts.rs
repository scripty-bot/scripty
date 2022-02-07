use crate::error::on_error;
use poise::FrameworkOptions;
use serenity::builder::{CreateAllowedMentions, ParseValue};

pub fn get_framework_opts() -> FrameworkOptions<crate::Data, crate::Error> {
    FrameworkOptions {
        commands: Vec::new(),
        on_error: |error| Box::pin(on_error(error)),
        listener: |_, _, _, _| Box::pin(async { Ok(()) }),
        pre_command: |_| Box::pin(async {}),
        post_command: |_| Box::pin(async {}),
        command_check: Some(crate::entity_block::check_block),
        allowed_mentions: Some({
            let mut f = CreateAllowedMentions::default();
            // Only support direct user pings by default
            f.empty_parse().parse(ParseValue::Users);
            f
        }),
        prefix_options: Default::default(),
        owners: Default::default(),
    }
}
