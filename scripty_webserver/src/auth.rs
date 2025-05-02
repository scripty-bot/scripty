use axum::{extract::FromRequestParts, http::request::Parts};

use crate::errors::WebServerError;

/// A type that handles authentication.
///
/// Set it as the type of an argument to a server endpoint handler to enable authentication for that endpoint.
pub struct Authentication {
	/// The token used for auth.
	#[allow(dead_code)]
	pub token:   String,
	/// The user ID that was authenticated.
	///
	/// If this is `0`, a global auth token was used.
	pub user_id: u64,
}

impl<S> FromRequestParts<S> for Authentication
where
	S: Send + Sync,
{
	type Rejection = WebServerError;

	async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
		// fetch config
		let config = scripty_config::get_config();

		// get token from header
		let token = req
			.headers
			.get("Authorization")
			.ok_or(WebServerError::AuthenticationFailed(1))?;
		// convert to string
		let token = token
			.to_str()
			.map_err(|_| WebServerError::AuthenticationFailed(2))?
			.to_string();

		// check token
		if config.api_tokens.contains(&token) {
			// global token was used
			return Ok(Authentication { token, user_id: 0 });
		}

		// TODO: add support for user tokens
		// right now, only global tokens are supported
		Err(WebServerError::AuthenticationFailed(3))
	}
}
