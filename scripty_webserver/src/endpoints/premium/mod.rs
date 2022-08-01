use crate::auth::Authentication;
use crate::errors::WebServerError;
use crate::models::*;
use axum::{routing::post, Json};

/// # TRIAL END
pub async fn trial_end(
    data: Json<TrialWillEndJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    Ok(())
}

/// # SUBSCRIPTION CREATE
pub async fn subscription_create(
    data: Json<SubscriptionCreatedJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    Ok(())
}

/// # SUBSCRIPTION UPDATE
pub async fn subscription_update(
    data: Json<SubscriptionUpdatedJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    Ok(())
}

/// # SUBSCRIPTION DELETE
pub async fn subscription_delete(
    data: Json<SubscriptionDeletedJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    Ok(())
}

/// # EARLY FRAUD WARNING
pub async fn early_fraud_warning(
    data: Json<EarlyFraudWarningJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    Ok(())
}

/// # INVOICE CREATED
pub async fn invoice_created(
    data: Json<InvoiceStatusJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    Ok(())
}

/// # INVOICE PAID
pub async fn invoice_paid(
    data: Json<InvoiceStatusJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    Ok(())
}

/// # INVOICE PAYMENT FAILED
pub async fn invoice_payment_failed(
    data: Json<InvoiceStatusJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    Ok(())
}

/// # INVOICE PAYMENT ACTION REQUIRED
pub async fn invoice_payment_action_required(
    data: Json<InvoiceStatusJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    Ok(())
}

/// # INVOICE UPCOMING
pub async fn invoice_upcoming(
    data: Json<InvoiceStatusJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    Ok(())
}

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/premium/trial_end", post(trial_end))
        .route("/premium/subscription_create", post(subscription_create))
        .route("/premium/subscription_update", post(subscription_update))
        .route("/premium/subscription_delete", post(subscription_delete))
        .route("/premium/early_fraud_warning", post(early_fraud_warning))
        .route("/premium/invoice_created", post(invoice_created))
        .route("/premium/invoice_paid", post(invoice_paid))
        .route(
            "/premium/invoice_payment_failed",
            post(invoice_payment_failed),
        )
        .route(
            "/premium/invoice_payment_action_required",
            post(invoice_payment_action_required),
        )
        .route("/premium/invoice_upcoming", post(invoice_upcoming))
}
