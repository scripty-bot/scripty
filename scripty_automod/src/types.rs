use std::collections::HashMap;

#[repr(i16)]
#[derive(Debug, poise::ChoiceParameter, Copy, Clone)]
#[non_exhaustive]
pub enum AutomodRuleType {
	Regular = 1,
}

impl From<i16> for AutomodRuleType {
	fn from(value: i16) -> Self {
		match value {
			1 => AutomodRuleType::Regular,
			_ => panic!("invalid value for AutomodRuleType"),
		}
	}
}

#[repr(i16)]
#[derive(Debug, poise::ChoiceParameter, Copy, Clone)]
pub enum AutomodRuleAction {
	#[name = "Silent delete"]
	SilentDelete        = 1,
	#[name = "Delete and log"]
	DeleteAndLog        = 2,
	#[name = "Delete, log, and remove user from VC"]
	DeleteLogAndKick    = 3,
	#[name = "Delete, log, and silence user"]
	DeleteLogAndSilence = 4,
}

impl From<i16> for AutomodRuleAction {
	fn from(value: i16) -> Self {
		match value {
			1 => AutomodRuleAction::SilentDelete,
			2 => AutomodRuleAction::DeleteAndLog,
			3 => AutomodRuleAction::DeleteLogAndKick,
			4 => AutomodRuleAction::DeleteLogAndSilence,
			_ => panic!("invalid value for AutomodRuleAction"),
		}
	}
}

#[derive(Debug, Clone)]
pub struct AutomodRule {
	pub rule_type:   AutomodRuleType,
	pub rule_data:   String,
	pub rule_action: AutomodRuleAction,
}

#[derive(Debug, Clone)]
pub struct AutomodRuleGroup {
	pub group_id:   i64,
	pub group_name: String,
	pub rules:      Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct AutomodServerConfig {
	pub guild_id:       u64,
	pub internal_id:    i32,
	pub enabled:        bool,
	pub groups:         Vec<AutomodRuleGroup>,
	rules:              Vec<AutomodRule>,
	rule_action_map:    HashMap<String, AutomodRuleAction>,
	rule_array:         Vec<String>,
	pub log_channel_id: u64,
	pub log_recording:  bool,
}

impl AutomodServerConfig {
	pub fn new(
		guild_id: u64,
		internal_id: i32,
		enabled: bool,
		groups: Vec<AutomodRuleGroup>,
		rules: Vec<AutomodRule>,
		log_channel_id: u64,
		log_recording: bool,
	) -> Self {
		let mut rule_action_map = HashMap::new();
		for rule in &rules {
			rule_action_map.insert(rule.rule_data.clone(), rule.rule_action);
		}
		let rule_array = rules.iter().map(|r| r.rule_data.clone()).collect();

		Self {
			guild_id,
			internal_id,
			enabled,
			groups,
			rules,
			rule_action_map,
			rule_array,
			log_channel_id,
			log_recording,
		}
	}

	pub fn add_rule(&mut self, rule: AutomodRule) {
		self.rule_action_map
			.insert(rule.rule_data.clone(), rule.rule_action);
		self.rule_array.push(rule.rule_data.clone());
		self.rules.push(rule);
	}

	pub fn get_action(&self, msg: &str) -> Option<AutomodRuleAction> {
		if !self.enabled {
			return None;
		}

		let m = msg.to_lowercase();
		for rule in &self.rule_array {
			if m.contains(rule) {
				return Some(
					self.rule_action_map
						.get(rule)
						.copied()
						.unwrap_or(AutomodRuleAction::SilentDelete),
				);
			}
		}

		None
	}
}
