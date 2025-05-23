use axum::{
	Json,
	extract::FromRequestParts,
	http::{StatusCode, request::Parts},
};
use scripty_bot_utils::extern_utils::{CreateEmbed, CreateEmbedFooter, CreateMessage, UserId};
use scripty_botlists::wumpus_store::IncomingWebhook;

use crate::errors::WebServerError;

pub struct WumpusStoreAuthorization;
impl<S> FromRequestParts<S> for WumpusStoreAuthorization
where
	S: Send + Sync,
{
	type Rejection = (StatusCode, &'static str);

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
		debug!("got webhook request for wumpus.store hook");

		let authorization = parts
			.headers
			.get("Authorization")
			.ok_or((StatusCode::UNAUTHORIZED, "No Authorization header provided"))?
			.to_str()
			.map_err(|_| {
				(
					StatusCode::BAD_REQUEST,
					"Authorization header was not valid UTF-8",
				)
			})?
			.trim();

		let Some(webhook_config) = scripty_config::get_config().bot_lists.get("wumpus_store")
		else {
			warn!("couldn't find valid configuration for wumpus.store (got none)");
			return Err((StatusCode::UNAUTHORIZED, "Invalid token"));
		};

		let scripty_config::BotListsConfig::FullConfig { token: _, webhook } = webhook_config
		else {
			warn!(
				"couldn't find valid configuration for wumpus.store (got single key, need double)"
			);
			return Err((StatusCode::UNAUTHORIZED, "Invalid token"));
		};
		let webhook = webhook.as_str();

		if authorization == webhook {
			Ok(Self)
		} else {
			debug!(
				"webhook request had invalid authorization header: got <{}>",
				authorization
			);
			Err((StatusCode::UNAUTHORIZED, "Invalid token"))
		}
	}
}

pub async fn wumpus_store_incoming_webhook(
	_authorization: WumpusStoreAuthorization,
	Json(IncomingWebhook {
		webhook_test,
		user_id,
		..
	}): Json<IncomingWebhook>,
) -> Result<(), WebServerError> {
	// check if the user is opted out of notifications
	let reminders_enabled = sqlx::query!(
		"SELECT vote_reminder_enabled FROM users WHERE user_id = $1",
		&scripty_utils::hash_user_id(user_id)
	)
	.fetch_optional(scripty_db::get_db())
	.await?
	.map(|row| row.vote_reminder_enabled)
	.unwrap_or(true);

	if !reminders_enabled {
		return Ok(());
	}

	// send them a message
	let cache_http = scripty_bot_utils::extern_utils::get_cache_http();
	UserId::new(user_id)
		.direct_message(
			&cache_http.http,
			CreateMessage::new().embed(
				CreateEmbed::new()
					.title("Thanks for voting for Scripty on wumpus.store!")
					.description(
						"You can vote again in 12 hours. We'll send you a reminder then. If you \
						 don't want to be notified, run `/vote_reminders False`. Thanks for your \
						 support!",
					)
					.footer(CreateEmbedFooter::new(if webhook_test {
						"Webhook test"
					} else {
						""
					})),
			),
		)
		.await?;

	// if they're opted in, set up a reminder for 12 hours from now

	sqlx::query!(
		"INSERT INTO vote_reminders (user_id, site_id, next_reminder)
           VALUES ($1, 3, NOW() + INTERVAL '12 hours')
           ON CONFLICT (user_id, site_id)
               DO UPDATE SET next_reminder = NOW() + INTERVAL '12 hours'",
		user_id as i64
	)
	.execute(scripty_db::get_db())
	.await?;

	Ok(())
}
