#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StripeWebhookEvent {
	/// Discord user ID of the user who owns the subscription.
	/// Processed by the server which manages the subscription.
	pub user_id: u64,

	/// Are we in live mode?
	pub live_mode: bool,

	/// The Stripe event type.
	pub event: StripeWebhookEventEnum,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "t", content = "c")]
pub enum StripeWebhookEventEnum {
	/// Fired three days before a subscription's trial period is scheduled to end,
	/// or when a trial is ended immediately (using trial_end=now).
	#[serde(rename = "customer.subscription.trial_will_end")]
	CustomerSubscriptionTrialWillEnd(CustomerSubscriptionTrialWillEnd),

	/// Fired when a customer creates a new subscription to a plan.
	/// Does not mean a charge has occurred yet, so should not be used to provision access.
	#[serde(rename = "customer.subscription.created")]
	CustomerSubscriptionCreated(CustomerSubscriptionCreated),

	/// Fired when a subscription is updated.
	/// For example, when a subscription is upgraded or downgraded.
	/// Does not have to be a change in the plan.
	#[serde(rename = "customer.subscription.updated")]
	CustomerSubscriptionUpdated(CustomerSubscriptionUpdated),

	/// Fired when a subscription is deleted.
	#[serde(rename = "customer.subscription.deleted")]
	CustomerSubscriptionDeleted(CustomerSubscriptionDeleted),

	/// Fired when a customer's payment method expires soon.
	#[serde(rename = "customer.source.expiring")]
	CustomerSourceExpiring(CustomerSourceExpiring),

	/// Fired when a dispute is created.
	#[serde(rename = "charge.dispute.created")]
	ChargeDisputeCreated(ChargeDisputeCreated),
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct CustomerSubscriptionTrialWillEnd {
	/// End timestamp of the trial period. Unix timestamp.
	pub trial_end: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct CustomerSubscriptionCreated {
	/// The tier of the subscription.
	/// 1 = tier 1, 2 = tier 2, 3 = tier 3, etc...
	pub tier: u8,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct CustomerSubscriptionUpdated {
	/// The tier of the subscription.
	/// 1 = tier 1, 2 = tier 2, 3 = tier 3, etc...
	pub tier: u8,

	/// The status of the subscription.
	pub status: SubscriptionStatus,

	/// Whether to cancel the subscription at the end of the current period.
	pub cancel_at_period_end: bool,

	/// End of the current period, as a Unix timestamp.
	/// If the subscription is canceled, this is the time the subscription will end.
	pub current_period_end: u64,

	/// End of the trial period, as a Unix timestamp.
	/// Only present if status is [SubscriptionStatus::Trialing].
	pub trial_end: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct CustomerSubscriptionDeleted {
	/// The tier of the subscription.
	/// 1 = tier 1, 2 = tier 2, 3 = tier 3, etc...
	pub tier: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerSourceExpiring {
	/// Brand of the card
	pub brand: Option<String>,

	/// Last 4 digits of the card number
	pub last4: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct ChargeDisputeCreated {}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
	Active,
	Canceled,
	Incomplete,
	IncompleteExpired,
	PastDue,
	Paused,
	Trialing,
	Unpaid,
}
