use crate::auth::Authentication;
use crate::errors::WebServerError;
use crate::models::*;
use axum::{routing::post, Json};
use sqlx::types::time::OffsetDateTime;
use std::num::NonZeroU64;

/// # TRIAL END
pub async fn trial_end(
    Json(data): Json<TrialWillEndJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    // TODO: DM user

    Ok(())
}

/// # SUBSCRIPTION CREATE
pub async fn subscription_create(
    Json(data): Json<SubscriptionCreatedJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    let hashed_user_id =
        scripty_utils::hash_user_id(NonZeroU64::new(data.discord_id).expect("expected NonZeroU64"));

    let update_tier = match data.status {
        SubscriptionStatus::Trialing => {
            // alter the user in the DB
            let db = scripty_db::get_db();

            sqlx::query!(
                "UPDATE users SET trial_used = true AND is_trialing = true WHERE user_id = $1",
                hashed_user_id
            )
            .execute(db)
            .await?;
            true
        }
        SubscriptionStatus::Active => {
            // alter the user in the DB
            let db = scripty_db::get_db();

            sqlx::query!(
                "UPDATE users SET is_trialing = false WHERE user_id = $1",
                hashed_user_id
            )
            .execute(db)
            .await?;
            true
        }
        _ => {
            // alter the user in the DB
            let db = scripty_db::get_db();

            sqlx::query!(
                "UPDATE users SET is_trialing = false WHERE user_id = $1",
                hashed_user_id
            )
            .execute(db)
            .await?;
            false
        }
    };

    if update_tier {
        let tier = data.tier;
        let db = scripty_db::get_db();
        sqlx::query!(
            "UPDATE users SET premium_level = $1 WHERE user_id = $2",
            tier as i16,
            hashed_user_id
        )
        .execute(db)
        .await?;
    }

    // TODO: DM user

    Ok(())
}

/// # SUBSCRIPTION UPDATE
pub async fn subscription_update(
    Json(data): Json<SubscriptionUpdatedJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    let hashed_user_id =
        scripty_utils::hash_user_id(NonZeroU64::new(data.discord_id).expect("expected NonZeroU64"));

    let update_tier = match data.status {
        SubscriptionStatus::Trialing => {
            // alter the user in the DB
            let db = scripty_db::get_db();

            sqlx::query!(
                "UPDATE users SET trial_used = true AND is_trialing = true WHERE user_id = $1",
                hashed_user_id
            )
            .execute(db)
            .await?;
            true
        }
        SubscriptionStatus::Active => {
            // alter the user in the DB
            let db = scripty_db::get_db();

            sqlx::query!(
                "UPDATE users SET is_trialing = false WHERE user_id = $1",
                hashed_user_id
            )
            .execute(db)
            .await?;
            true
        }
        _ => {
            // alter the user in the DB
            let db = scripty_db::get_db();

            sqlx::query!(
                "UPDATE users SET is_trialing = false WHERE user_id = $1",
                hashed_user_id
            )
            .execute(db)
            .await?;
            false
        }
    };

    if update_tier {
        let tier = data.tier;
        let db = scripty_db::get_db();
        sqlx::query!(
            "UPDATE users SET premium_level = $1 WHERE user_id = $2",
            tier as i16,
            hashed_user_id
        )
        .execute(db)
        .await?;
    }

    if let Some(expiry_timestamp) = data.plan_ends_at {
        let db = scripty_db::get_db();

        // convert the Unix timestamp in expiry_timestamp to a sqlx::types::time::PrimitiveDateTime
        let expiry_timestamp = OffsetDateTime::from_unix_timestamp(expiry_timestamp as i64);

        sqlx::query!(
            "UPDATE users SET premium_expiry = $1 WHERE user_id = $2",
            Some(expiry_timestamp),
            hashed_user_id
        )
        .execute(db)
        .await?;
    }

    // TODO: DM user

    Ok(())
}

/// # SUBSCRIPTION DELETE
pub async fn subscription_delete(
    Json(data): Json<SubscriptionDeletedJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    let hashed_user_id =
        scripty_utils::hash_user_id(NonZeroU64::new(data.discord_id).expect("expected NonZeroU64"));
    let db = scripty_db::get_db();

    if let Some(expiry_timestamp) = data.plan_ends_at {
        // convert the Unix timestamp in expiry_timestamp to a sqlx::types::time::PrimitiveDateTime
        let expiry_timestamp = OffsetDateTime::from_unix_timestamp(expiry_timestamp as i64);

        sqlx::query!(
            "UPDATE users SET premium_expiry = $1 WHERE user_id = $2",
            Some(expiry_timestamp),
            hashed_user_id
        )
        .execute(db)
        .await?
    } else {
        // expire now by setting premium level to 0
        sqlx::query!(
            "UPDATE users SET premium_level = 0 WHERE user_id = $1",
            hashed_user_id
        )
        .execute(db)
        .await?
    };

    // TODO: send DM to the user on Discord

    Ok(())
}

/// # EARLY FRAUD WARNING
pub async fn early_fraud_warning(
    Json(_): Json<EarlyFraudWarningJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    Ok(())
}

/// # INVOICE CREATED
pub async fn invoice_created(
    Json(data): Json<InvoiceStatusJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    // TODO: DM user

    Ok(())
}

/// # INVOICE PAID
pub async fn invoice_paid(
    Json(data): Json<InvoiceStatusJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    // TODO: DM user

    Ok(())
}

/// # INVOICE PAYMENT FAILED
pub async fn invoice_payment_failed(
    Json(data): Json<InvoiceStatusJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    // TODO: DM user

    Ok(())
}

/// # INVOICE PAYMENT ACTION REQUIRED
pub async fn invoice_payment_action_required(
    Json(data): Json<InvoiceStatusJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    // TODO: DM user

    Ok(())
}

/// # INVOICE UPCOMING
pub async fn invoice_upcoming(
    Json(data): Json<InvoiceStatusJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    // TODO: DM user

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
