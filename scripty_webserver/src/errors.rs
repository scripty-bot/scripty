use std::fmt::{Display, Formatter};

use axum::{
	http::{header::CONTENT_TYPE, HeaderValue, StatusCode},
	response::{IntoResponse, Response},
};
use serde::Serialize;
use time::error::ComponentRange;

/// All possible errors that can occur when handling a request.
#[derive(Debug)]
pub enum WebServerError {
	/// Token provided in the request was invalid.
	///
	/// Code `1`, sub-code is the inner integer of this variant.
	///
	/// Sub-code `1`: No token was provided in the `Authorization` header.
	/// Sub-code `2`: The token was not valid UTF-8.
	/// Sub-code `3`: The token was not a valid token.
	AuthenticationFailed(i32),

	/// Bot cache was unavailable at request time.
	///
	/// Code `2`, no sub-code.
	CacheUnavailable,

	/// The database returned an error.
	///
	/// Code `3`, no sub-code.
	DatabaseError(Option<sqlx::Error>),

	/// Parsing an integer failed.
	///
	/// Code `5`, no sub-code.
	ParseIntError,

	/// Serenity returned an error.
	///
	/// Code `6`, no sub-code.
	SerenityError,
}

impl From<scripty_bot_utils::extern_utils::CacheNotInitializedError> for WebServerError {
	fn from(_: scripty_bot_utils::extern_utils::CacheNotInitializedError) -> Self {
		WebServerError::CacheUnavailable
	}
}

impl From<scripty_bot_utils::extern_utils::SerenityError> for WebServerError {
	fn from(_: scripty_bot_utils::extern_utils::SerenityError) -> Self {
		WebServerError::SerenityError
	}
}

impl From<sqlx::Error> for WebServerError {
	fn from(e: sqlx::Error) -> Self {
		WebServerError::DatabaseError(Some(e))
	}
}

impl From<ComponentRange> for WebServerError {
	fn from(_: ComponentRange) -> Self {
		WebServerError::DatabaseError(None)
	}
}

impl From<std::num::ParseIntError> for WebServerError {
	fn from(_: std::num::ParseIntError) -> Self {
		WebServerError::ParseIntError
	}
}

impl Display for WebServerError {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			WebServerError::AuthenticationFailed(_) => write!(f, "Authentication failed"),
			WebServerError::CacheUnavailable => write!(f, "Cache unavailable"),
			WebServerError::DatabaseError(Some(e)) => write!(f, "Database error: {:?}", e),
			WebServerError::DatabaseError(None) => write!(f, "Database error"),
			WebServerError::ParseIntError => write!(f, "Parse int error"),
			WebServerError::SerenityError => write!(f, "Serenity error"),
		}
	}
}

/// Private helper type for error serialization
#[derive(Serialize)]
struct ErrorJson {
	code:     u32,
	/// -1 means no sub-code
	sub_code: i32,
}

impl IntoResponse for WebServerError {
	fn into_response(self) -> Response {
		warn!("web server error: {:?}", self);

		let (body, code) = match self {
			WebServerError::AuthenticationFailed(sub_code) => {
				(ErrorJson { code: 1, sub_code }, StatusCode::FORBIDDEN)
			}
			WebServerError::CacheUnavailable => (
				ErrorJson {
					code:     2,
					sub_code: -1,
				},
				StatusCode::INTERNAL_SERVER_ERROR,
			),
			WebServerError::DatabaseError(_) => (
				ErrorJson {
					code:     3,
					sub_code: -1,
				},
				StatusCode::INTERNAL_SERVER_ERROR,
			),
			WebServerError::ParseIntError => (
				ErrorJson {
					code:     5,
					sub_code: -1,
				},
				StatusCode::BAD_REQUEST,
			),
			WebServerError::SerenityError => (
				ErrorJson {
					code:     6,
					sub_code: -1,
				},
				StatusCode::INTERNAL_SERVER_ERROR,
			),
		};

		let bytes = match serde_json::to_vec(&body) {
			Ok(bytes) => bytes,
			Err(e) => {
				warn!("Error serializing error response: {}", e);
				return (
					StatusCode::INTERNAL_SERVER_ERROR,
					[(CONTENT_TYPE, HeaderValue::from_static("text/plain"))],
					e.to_string(),
				)
					.into_response();
			}
		};
		(code, bytes).into_response()
	}
}
