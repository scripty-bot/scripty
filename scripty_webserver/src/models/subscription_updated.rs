#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionUpdatedJson {
    pub discord_id: u64,
    pub tier: u8,
    pub status: super::SubscriptionStatus,
    /// Unix timestamp
    pub plan_ends_at: Option<u64>,
    pub current_period_start: u64,
    #[serde(default)]
    pub cancel_at_period_end: bool,
}
