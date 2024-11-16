use std::{fmt, num::ParseIntError};

use serde::{Deserialize, Serialize};

pub type KiaiApiResult<T> = Result<T, KiaiApiError>;

#[derive(Debug)]
pub enum KiaiApiError {
	Reqwest(reqwest::Error),
	BadRequest(BadRequestPayload),
	Forbidden(String),
	Unauthorized(String),
	TooManyRequests(TooManyRequestsPayload),
	Unknown {
		status: reqwest::StatusCode,
		body:   String,
	},
	BadInteger(ParseIntError),
}

impl std::error::Error for KiaiApiError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct BadRequestPayload {
	pub error:   String,
	pub message: String,
	pub issues:  Vec<BadRequestIssue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BadRequestIssue {
	pub code:    String,
	pub message: String,
	pub path:    Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TooManyRequestsPayload {
	pub name:        String,
	pub message:     String,
	pub status:      u16,
	#[serde(rename = "resetAfter")]
	pub reset_after: u64,
}

impl From<reqwest::Error> for KiaiApiError {
	fn from(err: reqwest::Error) -> Self {
		Self::Reqwest(err)
	}
}

impl fmt::Display for KiaiApiError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("KiaiApiError { ")?;
		match self {
			Self::Reqwest(err) => write!(f, "{}", err),
			Self::BadRequest(payload) => write!(f, "BadRequest: {}", payload.message),
			Self::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
			Self::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
			Self::TooManyRequests(payload) => write!(f, "TooManyRequests: {}", payload.message),
			Self::Unknown { status, body } => write!(f, "Unknown: {} {}", status, body),
			Self::BadInteger(e) => write!(f, "Invalid integer received: {}", e),
		}?;
		f.write_str(" }")
	}
}

impl From<ParseIntError> for KiaiApiError {
	fn from(value: ParseIntError) -> Self {
		Self::BadInteger(value)
	}
}
