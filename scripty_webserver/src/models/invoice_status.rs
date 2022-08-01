#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceStatusJson {
    pub discord_id: u64,
    pub status: String,
    pub cost: String,
    /// Unix timestamp
    pub next_attempt: u64,
    pub invoice_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
}
