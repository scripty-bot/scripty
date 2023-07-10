use scripty_premium::PremiumTierList;

pub fn get_next_tier(current_tier: PremiumTierList) -> PremiumTierList {
	match current_tier {
		PremiumTierList::None => PremiumTierList::Tier1,
		PremiumTierList::Tier1 => PremiumTierList::Tier2,
		PremiumTierList::Tier2 => PremiumTierList::Tier3,
		PremiumTierList::Tier3 => PremiumTierList::Tier4,
		PremiumTierList::Tier4 => PremiumTierList::Tier5,
		PremiumTierList::Tier5 => PremiumTierList::Tier6,
		PremiumTierList::Tier6 => PremiumTierList::Tier6,
	}
}

pub fn get_tier_rule_count(tier: PremiumTierList) -> i64 {
	// the equation is roughly 500x^3 + 2500x^2 - 500x
	match tier {
		PremiumTierList::None => 25,
		PremiumTierList::Tier1 => 500,
		PremiumTierList::Tier2 => 5000,
		PremiumTierList::Tier3 => 16500,
		PremiumTierList::Tier4 => 38000,
		PremiumTierList::Tier5 => 72500,
		PremiumTierList::Tier6 => 123000,
	}
}
