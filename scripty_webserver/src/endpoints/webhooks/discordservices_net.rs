use axum::{
	async_trait,
	extract::FromRequestParts,
	http::{request::Parts, StatusCode},
	Json,
};
use scripty_bot_utils::extern_utils::{CreateEmbed, CreateMessage, UserId};
use scripty_botlists::discordservices_net::{Bot, DiscordServicesNetIncomingWebhook};

use crate::errors::WebServerError;

pub struct DiscordServicesNetAuthorization;
#[async_trait]
impl<S> FromRequestParts<S> for DiscordServicesNetAuthorization {
	type Rejection = (StatusCode, &'static str);

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
		let authorization = parts
			.headers
			.get("Authorization")
			.ok_or((StatusCode::UNAUTHORIZED, "No Authorization header provided"))?;
		let authorization = authorization.to_str().map_err(|_| {
			(
				StatusCode::BAD_REQUEST,
				"Authorization header was not valid UTF-8",
			)
		})?;

		let scripty_config::BotListsConfig::FullConfig { token: _, webhook } =
			scripty_config::get_config()
				.bot_lists
				.get("discordservices_net")
				.ok_or((StatusCode::UNAUTHORIZED, "Invalid token"))?
		else {
			return Err((StatusCode::UNAUTHORIZED, "Invalid token"));
		};

		if authorization != webhook {
			Err((StatusCode::UNAUTHORIZED, "Invalid token"))
		} else {
			Ok(Self)
		}
	}
}

pub async fn discordservices_net_incoming_webhook(
	_authorization: DiscordServicesNetAuthorization,
	Json(DiscordServicesNetIncomingWebhook {
		user: Bot { id, .. },
		..
	}): Json<DiscordServicesNetIncomingWebhook>,
) -> Result<(), WebServerError> {
	// check if the user is opted out of notifications
	let opted_out = sqlx::query!(
		"SELECT vote_reminder_disabled FROM users WHERE user_id = $1",
		scripty_utils::hash_user_id(id)
	)
	.fetch_optional(scripty_db::get_db())
	.await?
	.map(|row| row.vote_reminder_disabled)
	.unwrap_or(false);

	// regardless, send them a message
	let cache_http = scripty_bot_utils::extern_utils::get_cache_http();
	let dm_channel = UserId::new(id).create_dm_channel(&cache_http).await?;
	dm_channel
		.send_message(
			&cache_http,
			CreateMessage::new().embed(
				CreateEmbed::new()
					.title("Thanks for voting for Scripty on discordservices.net!")
					.description(if opted_out {
						"You can vote again in 12 hours. You're opted out of reminders, but if you \
						 want to be notified, run `/vote_reminders True`. Thanks for your support!"
					} else {
						"You can vote again in 12 hours. We'll send you a reminder then. If you \
						 don't want to be notified, run `/vote_reminders False`. Thanks for your \
						 support!"
					}),
			),
		)
		.await?;

	// if they're opted in, set up a reminder for 12 hours from now
	sqlx::query!(
		"INSERT INTO vote_reminders (user_id, site_id, next_reminder)
           VALUES ($1, 2, NOW() + INTERVAL '12 hours')
           ON CONFLICT (user_id, site_id)
               DO UPDATE SET next_reminder = NOW() + INTERVAL '12 hours'",
		id as i64
	)
	.execute(scripty_db::get_db())
	.await?;

	Ok(())
}
