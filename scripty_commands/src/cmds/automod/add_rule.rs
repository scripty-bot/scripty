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
pub async fn automod_add_rule(
    ctx: Context<'_>,

    #[description = "The type of rule to add. See `/automod rule_help` for more info."]
    rule_type: AutomodRuleType,

    #[description = "The rule content to add."] content: String,

    #[description = "The action to take when the rule is triggered."] action: AutomodRuleAction,
) -> Result<(), Error> {
    let resolved_language =
        scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

    // fetch the current guild's rule count
    let db = scripty_db::get_db();
    let gid = ctx.guild_id().expect("asserted in guild").0;
    let item_id: i32 = match sqlx::query!(
        "SELECT item_id FROM automod_config WHERE guild_id = $1",
        gid.get() as i64
    )
    .fetch_optional(db)
    .await?
    {
        Some(item) => item.item_id,
        None => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title(format_message!(
                            resolved_language,
                            "automod-add-rule-embed-failure-title"
                        ))
                        .description(format_message!(
                            resolved_language,
                            "automod-add-rule-embed-failure-description-not-setup"
                        )),
                ),
            )
            .await?;

            return Ok(());
        }
    };

    let count = sqlx::query!(
        r#"SELECT COUNT(*) AS "count!" FROM automod_rules WHERE source_id = $1"#,
        item_id
    )
    .fetch_optional(db)
    .await?
    .map(|x| x.count);

    let count: i64 = match count {
        Some(c) => c,
        None => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title(format_message!(
                            resolved_language,
                            "automod-add-rule-embed-failure-title"
                        ))
                        .description(format_message!(
                            resolved_language,
                            "automod-add-rule-embed-failure-description-not-setup"
                        )),
                ),
            )
            .await?;

            return Ok(());
        }
    };

    let premium_tier = scripty_premium::get_guild(gid).await.unwrap_or_default();
    let max_rules = get_tier_rule_count(premium_tier);

    let past_limit = count >= max_rules;
    let is_t6 = premium_tier == scripty_premium::PremiumTierList::Tier6;
    let is_not_none = premium_tier != scripty_premium::PremiumTierList::None;

    if past_limit {
        let translation_string = if !is_t6 && is_not_none {
            // tiers 1-5
            let next_tier = get_next_tier(premium_tier);
            let next_tier_rules = get_tier_rule_count(next_tier);

            format_message!(
                resolved_language,
                "automod-add-rule-embed-failure-description-max-rules",
                tier: format!("{}", premium_tier),
                maxRules: max_rules,
                nextTier: format!("{}", next_tier),
                nextTierMaxRules: next_tier_rules
            )
        } else if is_t6 && is_not_none {
            // tier 6
            format_message!(
                resolved_language,
                "automod-add-rule-embed-failure-description-premium-limit-hard-cap",
                hardCap: 123000
            )
        } else {
            // tier 0
            format_message!(
                resolved_language,
                "automod-add-rule-embed-failure-description-free-limit",
                tier: format!("{}", premium_tier),
                maxRules: max_rules
            )
        };

        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::default()
                    .title(format_message!(
                        resolved_language,
                        "automod-add-rule-embed-failure-title"
                    ))
                    .description(translation_string),
            ),
        )
        .await?;

        return Ok(());
    }

    let is_regular_rule = matches!(rule_type, AutomodRuleType::Regular);

    // check rule type
    if !is_not_none && !is_regular_rule {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::default()
                    .title(format_message!(
                        resolved_language,
                        "automod-add-rule-embed-failure-title"
                    ))
                    .description(format_message!(
                        resolved_language,
                        "automod-add-rule-embed-failure-description-free-locked-type"
                    )),
            ),
        )
        .await?;

        return Ok(());
    }

    // all checks passed, add the rule
    let rule_id = sqlx::query!(
        "INSERT INTO automod_rules (source_id, rule_type, rule_data, rule_action) VALUES ($1, $2, $3, $4) RETURNING item_id",
        item_id,
        rule_type as i16,
        content,
        action as i16
    ).fetch_one(db).await?.item_id;

    let extra_details = if !is_not_none {
        format_message!(
            resolved_language,
            "automod-add-rule-embed-extra-details-free-limit"
        )
    } else {
        "".to_string()
    };

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title(format_message!(
                    resolved_language,
                    "automod-add-rule-embed-success-title",
                    ruleId: rule_id
                ))
                .description(format_message!(
                    resolved_language,
                    "automod-add-rule-embed-success-description",
                    rulesLeft: max_rules - count - 1,
                    maxRules: max_rules,
                    extraDetails: extra_details
                )),
        ),
    )
    .await?;

    Ok(())
}
