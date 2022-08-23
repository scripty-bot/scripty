use crate::auth::Authentication;
use crate::errors::WebServerError;
use axum::{routing::post, Json};
use scripty_commands::{CreateEmbed, CreateEmbedFooter, CreateMessage, UserId};
use sqlx::types::time::OffsetDateTime;
use std::num::NonZeroU64;
use stripe::{EventObject, EventType, SubscriptionStatus, WebhookEvent};

pub async fn stripe_webhook(
    Json(WebhookEvent {
        event_type,
        data,
        livemode,
        ..
    }): Json<WebhookEvent>,
    Authentication { user_id, .. }: Authentication,
) -> Result<(), WebServerError> {
    if user_id != 0 {
        return Err(WebServerError::AuthenticationFailed(3));
    }

    let mut embed = CreateEmbed::default();
    if !livemode {
        // add a field to the footer of the embed to show that this is a test webhook
        embed = embed.footer(
            CreateEmbedFooter::default()
                .text("test data, not real | if you're a user and seeing this, this is a bug"),
        );
    }

    let target_user_id = match event_type {
        EventType::ChargeDisputeCreated => {
            // right now we don't process these, as the Python server needs work to do this
            None
        }
        EventType::CustomerSourceExpiring => {
            // process the data as a Card object
            let card = if let EventObject::Card(card) = data.object {
                card
            } else {
                warn!("missing card data in CustomerSourceExpiring event");
                return Err(WebServerError::MissingData);
            };
            let discord_id = if let Some(customer) = card.customer {
                // the Python server does hacky stuff and replaces all customer objects with their Discord snowflake ID
                let mut cid = customer.id().as_str().to_string();
                cid.remove_matches("cus_");
                cid.parse::<u64>()?
            } else {
                // the python server should always have a customer ID in here, so panic
                panic!("missing customer ID in CustomerSourceExpiring event");
            };

            embed = embed.title("Card Expiring").description(format!(
                "Heads up: your card ({} **{}) is expiring at the end of this month. Please update your card information. \
                You can head over to the dashboard at https://dash.scripty.org/ to do so. If you don't want to continue \
                paying for Scripty Premium, consider cancelling your subscription at the same site.\n\
                Thanks!\n\n~ the Scripty team",
                card.brand.unwrap_or_else(|| "Unknown".to_string()),
                card.last4.unwrap_or_else(|| "Unknown".to_string())
            ));

            Some(discord_id)
        }
        EventType::CustomerSubscriptionCreated => {
            // really we don't need to do anything here
            None
        }
        // TODO: rest of these enum variants
        EventType::CustomerSubscriptionDeleted => {
            // process the data as a Subscription object
            let subscription = if let EventObject::Subscription(subscription) = data.object {
                subscription
            } else {
                warn!("missing subscription data in CustomerSubscriptionDeleted event");
                return Err(WebServerError::MissingData);
            };

            let mut cid = subscription.customer.id().as_str().to_string();
            cid.remove_matches("cus_");
            let discord_id = cid.parse::<u64>()?;

            embed = embed.title("Subscription Cancelled").description(
                "Your subscription to Scripty Premium has officially been cancelled. \
                If you would like to reactivate it, head to https://dash.scripty.org/.\n\
                Thank you for trying out Scripty Premium! We appreciate your support.\n\
                ~ the Scripty team",
            );

            // update the user's status in the database
            let hashed_user_id = scripty_utils::hash_user_id(
                NonZeroU64::new(discord_id).expect("expected non-zero discord ID"),
            );
            let db = scripty_db::get_db();
            sqlx::query!(
                "UPDATE users SET premium_level = 0, premium_expiry = NULL WHERE user_id = $1",
                hashed_user_id
            )
            .execute(db)
            .await?;

            Some(discord_id)
        }
        EventType::CustomerSubscriptionTrialWillEnd => {
            // process the data as a Subscription object
            let subscription = if let EventObject::Subscription(subscription) = data.object {
                subscription
            } else {
                warn!("missing subscription data in CustomerSubscriptionDeleted event");
                return Err(WebServerError::MissingData);
            };

            let mut cid = subscription.customer.id().as_str().to_string();
            cid.remove_matches("cus_");
            let discord_id = cid.parse::<u64>()?;

            embed = embed.title("Trial Ending").description(format!(
                "Please note that your free trial to Scripty Premium will end <t:{0}:F> (<t:{0}:R>).\n\
                At the timestamp marked above, you will be charged for a full month of Scripty Premium.\
                We do not offer refunds under any circumstances, so if you do not want to pay for this,\
                please cancel your subscription before the end date at https://dash.scripty.org/.\n\n\
                Thanks for using Scripty! ~ the Scripty team",
                subscription.trial_end.unwrap_or(0)
            ));

            Some(discord_id)
        }
        EventType::CustomerSubscriptionUpdated => {
            // process the data as a Subscription object
            let subscription = if let EventObject::Subscription(subscription) = data.object {
                subscription
            } else {
                warn!("missing subscription data in CustomerSubscriptionDeleted event");
                return Err(WebServerError::MissingData);
            };

            let mut cid = subscription.customer.id().as_str().to_string();
            cid.remove_matches("cus_");
            let discord_id = cid.parse::<u64>()?;

            let tier = *scripty_config::get_config()
                .premium
                .tier_map
                .get(
                    subscription
                        .items
                        .data
                        .get(0)
                        .expect("should be at least one subscription item")
                        .price
                        .as_ref()
                        .expect("item should have price")
                        .product
                        .as_ref()
                        .expect("item should have product")
                        .id()
                        .as_str(),
                )
                .ok_or(WebServerError::MissingData)?;

            // check the status of the subscription
            let retval = match subscription.status {
                SubscriptionStatus::Active => {
                    // check if subscription.cancel_at_period_end is set: if it is, then the user has cancelled their subscription:
                    // in that case, we should update the premium_expiry field in the database (expiry timestamp is current_period_end)
                    // if it's not set, then check if subscription.trial_end is set: if it is, then the user has started their trial
                    // if neither are set, then very likely a tier change has happened, which we'll update regardless later on
                    if subscription.cancel_at_period_end {
                        // update the expiry timestamp to the current period end
                        embed = embed.title("Subscription Cancelled").description(format!(
                            "Your subscription to Scripty Premium has been cancelled. You, and any servers you have \
                            activated Premium on, will lose their benefits <t:{0}:F> (<t:{0}:R>)\n\
                            We're sorry to see you go.\n\
                            If you have a moment, it'd be great if you could respond to this message telling us why you \
                            cancelled. In any case, thank you a lot for supporting Scripty.\n\n\
                            <:meow_heart:1003570104866443274> ~ the Scripty team",
                            subscription.current_period_end
                        )).footer(CreateEmbedFooter::default().text("https://xkcd.com/2257/"));

                        // update the expiry timestamp to the current period end
                        let expiry =
                            OffsetDateTime::from_unix_timestamp(subscription.current_period_end)?;
                        let hashed_user_id = scripty_utils::hash_user_id(
                            NonZeroU64::new(discord_id).expect("expected non-zero discord ID"),
                        );
                        let db = scripty_db::get_db();
                        sqlx::query!(
                            "UPDATE users SET premium_expiry = $1 WHERE user_id = $2",
                            expiry,
                            hashed_user_id
                        )
                        .execute(db)
                        .await?;
                    } else {
                        // update the expiry timestamp to the current period end
                        embed = embed.title("Tier Changed").description(format!(
                            "Your subscription has been updated to Tier {1}, and takes effect <t:{0}:F> (<t:{0}:R>).\n\
                            If you had more servers than you were supposed to with this new Premium tier due to a downgrade,\
                            all the servers you have added to Premium have been removed. You will need to re-add the \
                            servers you would like to keep Premium on.\n\
                            If you had fewer servers than you now have access to, you can use premium claim to add more servers.\
                            If you have any questions, you may respond to this message for support.",
                            subscription.current_period_end,
                            tier
                        ));
                    }

                    // update the tier in the database
                    let hashed_user_id = scripty_utils::hash_user_id(
                        NonZeroU64::new(discord_id).expect("expected non-zero discord ID"),
                    );
                    let db = scripty_db::get_db();
                    sqlx::query!(
                        "UPDATE users SET premium_level = $1 WHERE user_id = $2",
                        tier as i16,
                        hashed_user_id
                    )
                    .execute(db)
                    .await?;

                    Some(discord_id)
                }
                SubscriptionStatus::Canceled => {
                    // prepare the user's message
                    embed = embed.title("Subscription Cancelled").description(
                        "Your subscription to Scripty Premium has been finally cancelled. You, and any servers you have \
                        activated Premium on, have lost their benefits.\n\
                        We're sorry to see you go.\n\
                        If you have a moment, it'd be great if you could respond to this message telling us why you \
                        cancelled. In any case, thank you a lot for supporting Scripty.\n\n\
                        <:meow_heart:1003570104866443274> ~ the Scripty team".to_string()
                    ).footer(CreateEmbedFooter::default().text("https://xkcd.com/2257/"));

                    // remove the user's premium from the db
                    let hashed_user_id = scripty_utils::hash_user_id(
                        NonZeroU64::new(discord_id).expect("expected non-zero discord ID"),
                    );
                    let db = scripty_db::get_db();

                    sqlx::query!("UPDATE users SET premium_level = 0, premium_expiry = NULL WHERE user_id = $1", hashed_user_id)
                        .execute(db)
                        .await?;

                    Some(discord_id)
                }
                SubscriptionStatus::PastDue => {
                    // prepare the message
                    embed = embed.title("Subscription Past Due").description(
                        "Your subscription to Scripty Premium is overdue. You, and any servers you have activated Premium \
                        on, have lost their benefits.\n\
                        Your subscription will be cancelled sometime soon if this is not resolved.\n\
                        If you no longer wish to pay for Premium, simply log in at https://dash.scripty.org/ and cancel your subscription. \
                        However, if you would like to continue your subscription and continue your access, you can continue \
                        this subscription at the same site.\n\n\
                        Thanks for using Scripty! ~ the Scripty team".to_string()
                    );

                    // remove the user's premium
                    let hashed_user_id = scripty_utils::hash_user_id(
                        NonZeroU64::new(discord_id).expect("expected non-zero discord ID"),
                    );
                    let db = scripty_db::get_db();

                    sqlx::query!("UPDATE users SET premium_level = 0, premium_expiry = NULL WHERE user_id = $1", hashed_user_id)
                        .execute(db)
                        .await?;

                    Some(discord_id)
                }
                SubscriptionStatus::Trialing => {
                    // prepare embed
                    embed = embed.title("Trial Started").description(format!(
                        "Your trial for Tier {0} Scripty Premium has begun. It will end <t:{1}:F> (<t:{1}:R>), at \
                        which point you will be charged for a full month of the next tier.\n\
                         Note we do not offer refunds under any circumstances, so if you do not want to pay for \
                         this, please cancel your subscription before the end date at https://dash.scripty.org/.\n
                         If you have any questions, you may respond to this message.\n\n\
                         Thanks!\n\
                         ~ the Scripty team",
                        tier,
                        subscription.trial_end.unwrap_or(0)
                    ));

                    Some(discord_id)
                }
                SubscriptionStatus::Unpaid => {
                    // prepare embed
                    embed = embed.title("Subscription Unpaid").description(
                        "Your subscription to Scripty Premium is unpaid. You, and any servers you have activated Premium \
                        on, have lost their benefits.\n\
                        If you no longer wish to pay for Premium, simply log in at https://dash.scripty.org/ and cancel your subscription. \
                        However, if you would like to continue your subscription and continue your access, you can continue \
                        this subscription at the same site.\n\n\
                        Thanks for using Scripty! ~ the Scripty team"
                    ).footer(CreateEmbedFooter::default().text("this state should never be shown to a real user"));

                    // cancel the subscription
                    let hashed_user_id = scripty_utils::hash_user_id(
                        NonZeroU64::new(discord_id).expect("expected non-zero discord ID"),
                    );
                    let db = scripty_db::get_db();
                    sqlx::query!("UPDATE users SET premium_level = 0, premium_expiry = NULL WHERE user_id = $1", hashed_user_id)
                        .execute(db)
                        .await?;

                    Some(discord_id)
                }
                SubscriptionStatus::Incomplete => {
                    // this status is a grace period for newly created subscriptions, where they are not charged yet
                    // so do nothing here
                    None
                }
                SubscriptionStatus::IncompleteExpired => {
                    embed = embed.title("Subscription Expired").description(
                        "Your subscription to Scripty Premium failed to activate, likely due to a missing payment method.\n\
                        If you would still like to activate it again, head to the dashboard at https://dash.scripty.org.\n\
                        Otherwise, no action is needed.\n\n\
                        Thanks for using Scripty! ~ the Scripty team"
                    );

                    Some(discord_id)
                }
            };

            // update user in DB
            let hashed_user_id = scripty_utils::hash_user_id(
                NonZeroU64::new(discord_id).expect("expected non-zero discord ID"),
            );
            let db = scripty_db::get_db();
            sqlx::query!(
                "UPDATE users SET premium_level = $1 WHERE user_id = $2",
                tier as i16,
                hashed_user_id
            )
            .execute(db)
            .await?;

            retval
        }
        EventType::RadarEarlyFraudWarningCreated => None,
        _ => None,
    };

    if let Some(target_user_id) = target_user_id {
        debug!("sending DM to user for premium event");
        let cache_http = scripty_commands::get_cache_http();

        let dm_channel = UserId::from(target_user_id)
            .create_dm_channel(cache_http)
            .await?;
        dm_channel
            .send_message(cache_http, CreateMessage::default().embed(embed))
            .await?;
    } else {
        debug!("not sending DM to user");
    }

    Ok(())
}

pub fn router() -> axum::Router {
    axum::Router::new().route("/premium/stripe_webhook", post(stripe_webhook))
}
