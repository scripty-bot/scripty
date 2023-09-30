use std::num::NonZeroU64;

use axum::{routing::post, Json};
use scripty_bot_utils::extern_utils::{CreateEmbed, CreateEmbedFooter, CreateMessage, UserId};
use sqlx::types::time::OffsetDateTime;

use crate::{
	auth::Authentication,
	errors::WebServerError,
	models::{
		CustomerSubscriptionCreated,
		StripeWebhookEvent,
		StripeWebhookEventEnum,
		SubscriptionStatus,
	},
};

pub async fn stripe_webhook(
	Authentication {
		user_id: auth_user_id,
		..
	}: Authentication,
	Json(StripeWebhookEvent {
		user_id,
		live_mode,
		event,
	}): Json<StripeWebhookEvent>,
) -> Result<(), WebServerError> {
	if auth_user_id != 0 {
		return Err(WebServerError::AuthenticationFailed(3));
	}

	let mut embed = CreateEmbed::default();
	if !live_mode {
		// add a field to the footer of the embed to show that this is a test webhook
		embed = embed.footer(CreateEmbedFooter::new(
			"test data, not real | if you're a user and seeing this, this is a bug",
		));
	}

	let should_fire_webhook = match event {
		StripeWebhookEventEnum::CustomerSourceExpiring(evt) => {
			embed = embed.title("Card Expiring").description(format!(
                "Heads up: your card ({} **{}) is expiring at the end of this month. Please update your card information. \
                You can head over to the dashboard at https://dash.scripty.org/ to do so. If you don't want to continue \
                paying for Scripty Premium, consider cancelling your subscription at the same site.\n\
                Thanks!\n\n~ the Scripty team",
                evt.brand.unwrap_or_else(|| "Unknown".to_string()),
                evt.last4.unwrap_or_else(|| "Unknown".to_string())
            ));

			true
		}
		StripeWebhookEventEnum::CustomerSubscriptionCreated(CustomerSubscriptionCreated {
			tier,
			is_trial,
			trial_end,
		}) => {
			if is_trial {
				// prepare embed
				embed = embed.title("Trial Started").description(format!(
					"Your trial for Tier {0} Scripty Premium has begun. It will end <t:{1}:F> (<t:{1}:R>), at \
                        which point you will be charged for the next billing period.\n\
                         Note we do not offer refunds under any circumstances, so if you do not want to pay for \
                         this, please cancel your subscription before the end date at https://dash.scripty.org/.\n
                         If you have any questions, you may respond to this message.\n\n\
                         Thanks!\n\
                         ~ the Scripty team",
					tier,
					trial_end.unwrap_or(0)
				));

				// update user in DB
				let hashed_user_id = scripty_utils::hash_user_id(
					NonZeroU64::new(user_id).expect("expected non-zero discord ID"),
				);
				let db = scripty_db::get_db();
				sqlx::query!(
					r#"
INSERT INTO users
    (user_id, premium_level, premium_expiry)
VALUES
    ($1, 1, to_timestamp($2))
ON CONFLICT
    ON CONSTRAINT users_pkey
    DO UPDATE
    SET
        premium_level = 1,
        premium_expiry = to_timestamp($2)
"#,
					hashed_user_id,
					trial_end.unwrap_or(0) as f64
				)
				.execute(db)
				.await?;
				true
			} else {
				false
			}
		}
		StripeWebhookEventEnum::CustomerSubscriptionDeleted(_) => {
			embed = embed.title("Subscription Cancelled").description(
				"Your subscription to Scripty Premium has officially been cancelled. \
                If you would like to reactivate it, head to https://dash.scripty.org/.\n\
                Thank you for trying out Scripty Premium! We appreciate your support.\n\
                ~ the Scripty team",
			);

			// update the user's status in the database
			let hashed_user_id = scripty_utils::hash_user_id(
				NonZeroU64::new(user_id).expect("expected non-zero discord ID"),
			);
			let db = scripty_db::get_db();
			sqlx::query!(
				r#"
INSERT INTO users
    (user_id, premium_level, premium_expiry, is_trialing)
VALUES
    ($1, 0, NULL, false)
ON CONFLICT
    ON CONSTRAINT users_pkey
    DO UPDATE
    SET
        premium_level = 0,
        premium_expiry = NULL,
        is_trialing = false"#,
				hashed_user_id
			)
			.execute(db)
			.await?;

			true
		}
		StripeWebhookEventEnum::CustomerSubscriptionTrialWillEnd(evt) => {
			embed = embed.title("Trial Ending").description(format!(
                "Please note that your free trial to Scripty Premium will end <t:{0}:F> (<t:{0}:R>).\n\
                At the timestamp marked above, you will be charged for the next period of Scripty Premium.\
                We do not offer refunds under any circumstances, so if you do not want to pay for this,\
                please cancel your subscription before the end date at https://dash.scripty.org/.\n\n\
                Thanks for using Scripty! ~ the Scripty team",
                evt.trial_end.unwrap_or(0)
            ));

			true
		}
		StripeWebhookEventEnum::CustomerSubscriptionUpdated(evt) => {
			let tier = evt.tier;

			// check the status of the subscription
			match evt.status {
				SubscriptionStatus::Active => {
					if evt.is_length_change && evt.is_tier_change {
						embed = embed.title("Subscription Updated").description(format!(
							"Your subscription to Scripty Premium has been updated to Tier {0}, and will take effect \
							<t:{1}:F> (<t:{1}:R>).\n\
							If you have any questions, you may respond to this message for support.\n\
							Thanks for supporting Scripty!\n\n\
							~ the Scripty team",
							tier,
							evt.current_period_start
						));
					} else if evt.cancel_at_period_end {
						embed = embed.title("Subscription Cancelled").description(format!(
                            "Your subscription to Scripty Premium has been cancelled. You, and any servers you have \
                            activated Premium on, will lose their benefits <t:{0}:F> (<t:{0}:R>)\n\
                            We're sorry to see you go.\n\
                            If you have a moment, it'd be great if you could respond to this message telling us why you \
                            cancelled. In any case, thank you a lot for supporting Scripty.\n\n\
                            <:meow_heart:1003570104866443274> ~ the Scripty team",
                            evt.current_period_end
                        )).footer(CreateEmbedFooter::new("https://xkcd.com/2257/"));
					} else if evt.is_new {
						embed = embed.title("Subscription Started").description(format!(
							"Your subscription to Scripty Premium has started, and takes effect <t:{0}:F> (<t:{0}:R>).\n\
							If you have any questions, you may respond to this message for support.\n\
							Thanks for supporting Scripty!\n\n\
							~ the Scripty team",
							evt.current_period_start
						));
					} else if evt.is_renewal {
						embed = embed.title("Subscription Renewed").description(
							"Your subscription to Scripty Premium has successfully renewed! Thank you for your continued support.\n\
							If you did not want to renew your subscription, you may cancel it at https://dash.scripty.org/ to prevent \
							further charges, and reply to this message requesting a refund. A human will get back to you.\n\n\
							If you have any questions, you may respond to this message for support.\n\n\
							~ the Scripty team"
						);
					} else if evt.is_length_change {
						embed = embed.title("Subscription Length Changed").description(format!(
							"The length of your subscription has successfully been changed! It now ends on <t:{0}:F> (<t:{0}:R>).\n\
							If you have any questions, you may respond to this message for support.\n\
							Thanks for supporting Scripty!\n\n\
							~ the Scripty team",
							evt.current_period_end
						));
					} else if evt.is_tier_change {
						embed = embed.title("Tier Changed").description(format!(
                            "Your subscription has been updated to Tier {1}, and takes effect <t:{0}:F> (<t:{0}:R>).\n\
                            If you had more servers than you were supposed to with this new Premium tier due to a downgrade, \
                            all the servers you have added to Premium have been removed. You will need to re-add the \
                            servers you would like to keep Premium on.\n\
                            If you had fewer servers than you now have access to, you can use `/premium claim` to add more servers. \
                            If you have any questions, you may respond to this message for support.\n\n\
                            ~ the Scripty team",
                            evt.current_period_end,
                            tier
                        ));
					} else if evt.trial_finished {
						embed = embed.title("Trial Upgraded").description(format!(
							"Your subscription of Scripty Premium has been upgraded from a trial to a paid subscription, and \
							we have attempted to charge you for a full {1} of Tier {0}.\n\
							If you have any questions, you may respond to this message for support.\n\
							Thanks for supporting Scripty!\n\n\
							~ the Scripty team",
							tier,evt.interval
						));
					} else {
						embed = embed.title("Subscription Update").description(format!(
							"I am unable to determine what has changed about your subscription. You may want to review the following \
							information and contact support if you believe this is an error.\n\n\
							**Current Subscription End Date**: <t:{0}:F> (<t:{0}:R>)\n\
							**Current Subscription Tier**: {1}\n\
							**Current Subscription Status**: {2}\n\
							**Cancelling at Period End**: {3}\n\
							**Current Trial End**: {4}\n\
							**Is New**: {5}\n\
							**Is Renewal**: {6}\n\
							**Is Length Change**: {7}\n\
							**Is Tier Change**: {8}\n\
							If you're unsure what this means, you may respond to this message for support, \
							and we will dig into it for you.",
							evt.current_period_end,
							tier,
							evt.status,
							evt.cancel_at_period_end,
							evt.trial_end.map_or_else(|| "None".to_string(), |t| format!("<t:{0}:F> (<t:{0}:R>)", t)),
							evt.is_new,
							evt.is_renewal,
							evt.is_length_change,
							evt.is_tier_change
						));
					}

					// update the tier in the database
					let hashed_user_id = scripty_utils::hash_user_id(
						NonZeroU64::new(user_id).expect("expected non-zero discord ID"),
					);
					let db = scripty_db::get_db();
					sqlx::query!(
						r#"
INSERT INTO users
    (user_id, premium_level, premium_expiry, is_trialing)
VALUES
    ($1, $2, $3, false)
ON CONFLICT
    ON CONSTRAINT users_pkey
    DO UPDATE
    SET
        premium_level = $2,
        premium_expiry = $3,
        is_trialing = false
"#,
						hashed_user_id,
						tier as i16,
						OffsetDateTime::from_unix_timestamp(evt.current_period_end as i64)?,
					)
					.execute(db)
					.await?;

					true
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
                    ).footer(CreateEmbedFooter::new("https://xkcd.com/2257/"));

					// remove the user's premium from the db
					let hashed_user_id = scripty_utils::hash_user_id(
						NonZeroU64::new(user_id).expect("expected non-zero discord ID"),
					);
					let db = scripty_db::get_db();

					sqlx::query!(
						r#"
INSERT INTO users
    (user_id, premium_level, premium_expiry, is_trialing)
VALUES
    ($1, 0, NULL, false)
ON CONFLICT
    ON CONSTRAINT users_pkey
    DO UPDATE
    SET
        premium_level = 0,
        premium_expiry = NULL,
        is_trialing = false
"#,
						hashed_user_id
					)
					.execute(db)
					.await?;

					true
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
						NonZeroU64::new(user_id).expect("expected non-zero discord ID"),
					);
					let db = scripty_db::get_db();

					sqlx::query!(
						r#"
INSERT INTO users
    (user_id, premium_level, premium_expiry, is_trialing)
VALUES
    ($1, 0, NULL, false)
ON CONFLICT
    ON CONSTRAINT users_pkey
    DO UPDATE
    SET
        premium_level = 0,
        premium_expiry = NULL,
        is_trialing = false
        "#,
						hashed_user_id
					)
					.execute(db)
					.await?;

					true
				}
				SubscriptionStatus::Trialing => {
					// prepare embed
					embed = embed.title("Trial Started").description(format!(
                        "Your trial for Tier {0} Scripty Premium has begun. It will end <t:{1}:F> (<t:{1}:R>), at \
                        which point you will be charged for a {2} of the next tier.\n\
                         Note we do not offer refunds under any circumstances, so if you do not want to pay for \
                         this, please cancel your subscription before the end date at https://dash.scripty.org/.\n
                         If you have any questions, you may respond to this message.\n\n\
                         Thanks!\n\
                         ~ the Scripty team",
                        tier,
                        evt.trial_end.unwrap_or(0),
						evt.interval
                    ));

					// update user in DB
					let hashed_user_id = scripty_utils::hash_user_id(
						NonZeroU64::new(user_id).expect("expected non-zero discord ID"),
					);
					let db = scripty_db::get_db();
					sqlx::query!(
						r#"
INSERT INTO users
    (user_id, premium_level, premium_expiry)
VALUES
    ($1, 1, now() + INTERVAL '1 day')
ON CONFLICT
    ON CONSTRAINT users_pkey
    DO UPDATE
    SET
        premium_level = 1,
        premium_expiry = now() + INTERVAL '1 day'
"#,
						hashed_user_id
					)
					.execute(db)
					.await?;

					true
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
                    ).footer(CreateEmbedFooter::new("this state should never be shown to a real user"));

					// cancel the subscription
					let hashed_user_id = scripty_utils::hash_user_id(
						NonZeroU64::new(user_id).expect("expected non-zero discord ID"),
					);
					let db = scripty_db::get_db();
					sqlx::query!(
						r#"
INSERT INTO users
    (user_id, premium_level, premium_expiry, is_trialing)
VALUES
    ($1, 0, NULL, false)
ON CONFLICT
    ON CONSTRAINT users_pkey
    DO UPDATE
    SET
        premium_level = 0,
        premium_expiry = NULL,
        is_trialing = false
        "#,
						hashed_user_id
					)
					.execute(db)
					.await?;

					true
				}
				SubscriptionStatus::Incomplete => {
					// this status is a grace period for newly created subscriptions, where they are not charged yet
					// so do nothing here
					false
				}
				SubscriptionStatus::IncompleteExpired => {
					embed = embed.title("Subscription Expired").description(
                        "Your subscription to Scripty Premium failed to activate, likely due to a missing payment method.\n\
                        If you would still like to activate it again, head to the dashboard at https://dash.scripty.org.\n\
                        Otherwise, no action is needed.\n\n\
                        Thanks for using Scripty! ~ the Scripty team"
                    );

					true
				}
				SubscriptionStatus::Paused => {
					embed = embed.title("Subscription Paused").description(
                        "Your subscription to Scripty Premium has been paused. \
                        You will not be charged until you resume it. You can resume it at any time at https://dash.scripty.org/.\n\n\
                        Thanks for using Scripty! ~ the Scripty team"
                    );

					true
				}
			}
		}
		StripeWebhookEventEnum::ChargeDisputeCreated(_) => {
			// prepare embed
			embed = embed.title("Dispute Created").description(
				"Your payment for Scripty Premium has been disputed. \n\
				This means that your Premium has been revoked, and you have been banned from the bot. \
				You can appeal this ban by responding to this message. If you appeal successfully, \
				there will still be a $30 fee to cover the cost of the dispute that must be paid before \
				you can use Scripty again.\n\n\
				~ the Scripty team",
			);

			// ban the user
			let hashed_user_id = scripty_utils::hash_user_id(
				NonZeroU64::new(user_id).expect("expected non-zero discord ID"),
			);
			let db = scripty_db::get_db();
			sqlx::query!(
				r#"
INSERT INTO blocked_users
	(user_id, reason, blocked_since)
VALUES
	($1, 'disputed payment', now())
ON CONFLICT
	DO NOTHING
				"#,
				hashed_user_id
			)
			.execute(db)
			.await?;

			true
		}
	};

	if should_fire_webhook {
		debug!("sending DM to user for premium event");
		let cache_http = scripty_bot_utils::extern_utils::get_cache_http();

		let dm_channel = UserId::from(user_id).create_dm_channel(cache_http).await?;
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
