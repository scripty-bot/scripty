use crate::auth::Authentication;
use crate::errors::WebServerError;
use crate::models::*;
use axum::{routing::post, Json};
use scripty_commands::{CreateEmbed, CreateMessage, UserId};
use sqlx::types::time::OffsetDateTime;
use std::num::NonZeroU64;

/// # TRIAL END
pub async fn trial_will_end(
    Json(data): Json<TrialWillEndJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    let cache_http = scripty_commands::get_cache_http();

    match UserId::new(data.discord_id).to_user(cache_http).await {
        Ok(u) => {
            if let Err(e) = u
                .direct_message(
                    &cache_http,
                    CreateMessage::default().embed(
                        CreateEmbed::default()
                            .title("Free Trial Expires Soon")
                            .description(
                                "Your free trial for tier 1 Scripty Premium will expire soon.\
                                See below for the exact expiry timestamp.\n\
                                Please note: unless you log in to the dashboard at https://dash.scripty.org/ \
                                and manually cancel your subscription, you will automatically be charged for \
                                one month of Tier 1 Premium, worth US$5.45.",
                            )
                            .field(
                            "Expiry Timestamp",
                            format!("<t:{0}:F> (<t:{0}:R>)", data.trial_end),
                            true
                            ),
                    ),
                )
                .await
            {
                error!("Error sending DM: {}", e);
            }
        }
        Err(e) => {
            error!("Error fetching user: {}", e);
        }
    }

    Ok(())
}

/// # SUBSCRIPTION CREATE
pub async fn subscription_create(
    Json(_): Json<SubscriptionCreatedJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    Ok(())
}

/// # SUBSCRIPTION UPDATE
pub async fn subscription_update(
    Json(data): Json<SubscriptionUpdatedJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    let hashed_user_id =
        scripty_utils::hash_user_id(NonZeroU64::new(data.discord_id).expect("expected NonZeroU64"));

    let db = scripty_db::get_db();

    let mut embed = CreateEmbed::default();

    let (update_tier, cancel_sub, is_trialing) = match data.status {
        SubscriptionStatus::Trialing => {
            embed = embed
                .title("Trial Started")
                .description(
                    "Your trial to Tier 1 Scripty Premium has begun. This trial will expire in three days. \
                    You will get a DM 24 hours before it expires, that essentially repeats what follows.\n\
                    If you do not cancel your trial before it expires, you will be charged for one month of \
                    Tier 1 Premium, worth US$5.45. You can cancel your trial at any time by logging in to the \
                    dashboard at https://dash.scripty.org/.\n\
                    You may claim this premium in any server that has not had a trial claimed before, by using \
                    `premium claim` in that server.\n\
                    If you have any questions, you may respond to this message for support."
                );

            (true, false, true)
        }
        SubscriptionStatus::Active if !data.cancel_at_period_end => {
            embed = embed
                .title("Subscription Updated")
                .description(format!(
                    "Your subscription has been updated to Tier {1}, and takes effect <t:{0}:F> (<t:{0}:R>.\n\
                    If you had more servers than you were supposed to with this new Premium tier due to a downgrade, \
                    all the servers you have added to Premium have been removed. \
                    You will need to re-add the servers you would like to keep Premium on.\n\
                    If you had fewer servers than you now have access to, you can use `premium claim` to add more servers.\n\
                    If this is a brand new subscription, you can now start using your benefits.\n\
                    If you have any questions, you may respond to this message for support.",
                    data.current_period_start,
                    data.tier
                ));

            (true, false, false)
        }
        SubscriptionStatus::Active if data.cancel_at_period_end => {
            embed = embed.title("Subscription Cancelled").description(format!(
                "Your subscription to Scripty Premium has been cancelled. \
                You, and any servers you have activated Premium on, will lose their benefits <t:{0}:F> (<t:{0}:R>)\n\
                We're sorry to see you go.\n\
                If you have a moment, it'd be great if you could respond to this message telling us why you cancelled.\
                In any case, thank you a lot for supporting Scripty.\n\
                <:meow_heart:1003570104866443274> ~ the Scripty team",
                data.plan_ends_at.unwrap_or(0)
            ));

            (false, true, false)
        }
        SubscriptionStatus::Canceled => {
            embed = embed.title("Subscription Cancelled").description(format!(
                "Your subscription to Scripty Premium has been cancelled. \
                You, and any servers you have activated Premium on, will lose their benefits <t:{0}:F> (<t:{0}:R>)\n\
                We're sorry to see you go.\n\
                If you have a moment, it'd be great if you could respond to this message telling us why you cancelled.\
                In any case, thank you a lot for supporting Scripty.\n\
                <:meow_heart:1003570104866443274> ~ the Scripty team",
                data.plan_ends_at.unwrap_or(0)
            ));

            (false, true, false)
        }
        SubscriptionStatus::PastDue => {
            embed = embed.title("Subscription Past Due").description(format!(
                "Your subscription to Scripty Premium is overdue. You, and any servers you have activated Premium on, \
                have lost their benefits.\n\
                If this is not resolved by <t:{0}:F> (<t:{0}:R>), your subscription will be cancelled. \
                If you no longer wish to pay for Premium, simply log in at https://dash.scripty.org/ and cancel your \
                subscription.",
                data.current_period_start + 259200
            ));
            (false, true, false)
        }
        _ => (false, true, false),
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

    if is_trialing {
        sqlx::query!(
            "UPDATE users SET trial_used = true AND is_trialing = true WHERE user_id = $1",
            hashed_user_id
        )
        .execute(db)
        .await?;
    } else {
        sqlx::query!(
            "UPDATE users SET is_trialing = false WHERE user_id = $1",
            hashed_user_id
        )
        .execute(db)
        .await?;
    }

    if cancel_sub {
        sqlx::query!(
            "UPDATE users SET premium_level = 0 WHERE user_id = $1",
            hashed_user_id
        )
        .execute(db)
        .await?;
    }

    if let Some(expiry_timestamp) = data.plan_ends_at {
        let db = scripty_db::get_db();

        // convert the Unix timestamp in expiry_timestamp to a sqlx::types::time::PrimitiveDateTime
        let expiry_timestamp = OffsetDateTime::from_unix_timestamp(expiry_timestamp as i64)?;

        sqlx::query!(
            "UPDATE users SET premium_expiry = $1 WHERE user_id = $2",
            Some(expiry_timestamp),
            hashed_user_id
        )
        .execute(db)
        .await?;
    }

    let cache_http = scripty_commands::get_cache_http();
    match UserId::new(data.discord_id).to_user(&cache_http).await {
        Ok(u) => {
            if let Err(e) = u
                .direct_message(&cache_http, CreateMessage::default().embed(embed))
                .await
            {
                error!("Error sending DM: {}", e);
            }
        }
        Err(e) => {
            error!("Error fetching user: {}", e);
        }
    };

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

    // expire now by setting premium level to 0
    sqlx::query!(
        "UPDATE users SET premium_level = 0 WHERE user_id = $1",
        hashed_user_id
    )
    .execute(db)
    .await?;

    let cache_http = scripty_commands::get_cache_http();
    match UserId::new(data.discord_id).to_user(&cache_http).await {
        Ok(u) => {
            if let Err(e) = u.direct_message(
                cache_http,
                CreateMessage::default()
                    .embed(
                        CreateEmbed::default()
                            .title("Subscription Cancelled")
                            .description(
                                "As of now, your subscription to Scripty Premium has officially been canceled \
                                and deleted from our systems.\n\
                                We're sorry to see you go. If you have a moment, it'd be great if you could respond to \
                                this message telling us why you cancelled.\n\
                                In any case, thank you a lot for supporting Scripty.\n\
                                <:meow_heart:1003570104866443274> ~ the Scripty team"
                            )
                    )
            ).await {
                error!("Error sending DM: {}", e);
            }
        }
        Err(e) => {
            error!("Error fetching user: {}", e);
        }
    }

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
    Json(_): Json<InvoiceStatusJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    // TODO: DM user

    Ok(())
}

/// # INVOICE PAID
pub async fn invoice_paid(
    Json(_): Json<InvoiceStatusJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    // TODO: DM user

    Ok(())
}

/// # INVOICE PAYMENT FAILED
pub async fn invoice_payment_failed(
    Json(_): Json<InvoiceStatusJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    // TODO: DM user

    Ok(())
}

/// # INVOICE PAYMENT ACTION REQUIRED
pub async fn invoice_payment_action_required(
    Json(_): Json<InvoiceStatusJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    Ok(())
}

/// # INVOICE UPCOMING
pub async fn invoice_upcoming(
    Json(_): Json<InvoiceStatusJson>,
    _: Authentication,
) -> Result<(), WebServerError> {
    // TODO: DM user

    Ok(())
}

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/premium/trial_end", post(trial_will_end))
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
