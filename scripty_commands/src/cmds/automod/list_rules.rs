use super::{get_next_tier, get_tier_rule_count, AutomodRuleAction, AutomodRuleType};
use crate::{Context, Error};
use poise::CreateReply;
use serenity::builder::CreateEmbed;

#[poise::command(
    prefix_command,
    slash_command,
    guild_only,
    required_permissions = "MANAGE_GUILD",
    rename = "add_rule"
)]
pub async fn automod_list_rules(ctx: Context<'_>) -> Result<(), Error> {}
