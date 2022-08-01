#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionUpdatedJson {
    pub discord_id: u64,
    pub tier: u8,
    pub status: super::SubscriptionStatus,
    /// Unix timestamp
    pub cancel_at: Option<u64>,
    pub current_period_start: u64,
}
