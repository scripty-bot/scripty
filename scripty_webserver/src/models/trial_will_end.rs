#[derive(Debug, Serialize, Deserialize)]
pub struct TrialWillEndJson {
    pub discord_id: u64,
    /// Unix timestamp in seconds
    pub trial_end: u64,
}
