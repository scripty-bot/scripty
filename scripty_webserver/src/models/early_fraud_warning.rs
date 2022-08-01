#[derive(Debug, Serialize, Deserialize)]
pub struct EarlyFraudWarningJson {
    pub charge_id: String,
    pub reason: String,
    pub actionable: bool,
}

/// one of card_never_received, fraudulent_card_application, made_with_counterfeit_card, made_with_lost_card,
/// made_with_stolen_card, misc, unauthorized_use_of_card
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EarlyFraudWarningReason {
    CardNeverReceived,
    FraudulentCardApplication,
    MadeWithCounterfeitCard,
    MadeWithLostCard,
    MadeWithStolenCard,
    Misc,
    UnauthorizedUseOfCard,
}
