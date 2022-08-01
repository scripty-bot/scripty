#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionCreatedJson {
    pub discord_id: u64,
    pub tier: u8,
    pub status: super::SubscriptionStatus,
}
