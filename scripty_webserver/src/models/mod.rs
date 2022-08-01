mod early_fraud_warning;
mod invoice_status;
mod subscription_created;
mod subscription_deleted;
mod subscription_updated;
mod trial_will_end;

pub use early_fraud_warning::EarlyFraudWarningJson;
pub use invoice_status::InvoiceStatusJson;
pub use subscription_created::SubscriptionCreatedJson;
pub use subscription_deleted::SubscriptionDeletedJson;
pub use subscription_updated::SubscriptionUpdatedJson;
pub use trial_will_end::TrialWillEndJson;

/// incomplete, incomplete_expired, trialing, active, past_due, canceled, or unpaid
///
/// JSON format
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Incomplete,
    IncompleteExpired,
    Trialing,
    Active,
    PastDue,
    Canceled,
    Unpaid,
}
