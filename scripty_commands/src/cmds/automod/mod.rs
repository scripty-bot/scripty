mod add_rule;
mod list_rules;
mod remove_rule;
mod root;
mod setup;

pub use add_rule::automod_add_rule;
pub use list_rules::automod_list_rules;
pub use remove_rule::automod_remove_rule;
pub use root::automod_root;
pub use setup::automod_setup;

#[repr(i16)]
#[derive(Debug, poise::ChoiceParameter)]
#[non_exhaustive]
pub enum AutomodRuleType {
	Regular = 1,
}

#[repr(i16)]
#[derive(Debug, poise::ChoiceParameter)]
pub enum AutomodRuleAction {
	#[name = "Silent delete"]
	SilentDelete     = 1,
	#[name = "Delete and log"]
	DeleteAndLog     = 2,
	#[name = "Delete, log, and remove user from VC"]
	DeleteLogAndKick = 3,
}

fn get_next_tier(
	current_tier: scripty_premium::PremiumTierList,
) -> scripty_premium::PremiumTierList {
	match current_tier {
		scripty_premium::PremiumTierList::None => scripty_premium::PremiumTierList::Tier1,
		scripty_premium::PremiumTierList::Tier1 => scripty_premium::PremiumTierList::Tier2,
		scripty_premium::PremiumTierList::Tier2 => scripty_premium::PremiumTierList::Tier3,
		scripty_premium::PremiumTierList::Tier3 => scripty_premium::PremiumTierList::Tier4,
		scripty_premium::PremiumTierList::Tier4 => scripty_premium::PremiumTierList::Tier5,
		scripty_premium::PremiumTierList::Tier5 => scripty_premium::PremiumTierList::Tier6,
		scripty_premium::PremiumTierList::Tier6 => scripty_premium::PremiumTierList::Tier6,
	}
}

fn get_tier_rule_count(tier: scripty_premium::PremiumTierList) -> i64 {
	// equation is roughly 500x^3 + 2500x^2 - 500x
	match tier {
		scripty_premium::PremiumTierList::None => 25,
		scripty_premium::PremiumTierList::Tier1 => 500,
		scripty_premium::PremiumTierList::Tier2 => 5000,
		scripty_premium::PremiumTierList::Tier3 => 16500,
		scripty_premium::PremiumTierList::Tier4 => 38000,
		scripty_premium::PremiumTierList::Tier5 => 72500,
		scripty_premium::PremiumTierList::Tier6 => 123000,
	}
}
