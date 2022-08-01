#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionDeletedJson {
    pub discord_id: u64,
    pub tier: u8,
    pub status: super::SubscriptionStatus,
    /// Unix timestamp
    pub plan_ends_at: Option<u64>,
}
