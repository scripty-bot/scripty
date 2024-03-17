use std::{
	fmt,
	fmt::{Display, Formatter},
};

use backtrace::Backtrace;
use songbird::error::JoinError;

pub struct Error {
	pub kind:      ErrorKind,
	pub backtrace: Backtrace,
}

#[derive(Debug)]
pub enum ErrorKind {
	Join(JoinError),
	Database(sqlx::Error),
	Redis(scripty_redis::TransactionError),
	Serenity(serenity::Error),
	NoWebhookToken,
}

impl Error {
	pub fn no_webhook_token() -> Self {
		Self {
			kind:      ErrorKind::NoWebhookToken,
			backtrace: Backtrace::new_unresolved(),
		}
	}

	pub fn is_timed_out(&self) -> bool {
		matches!(self.kind, ErrorKind::Join(JoinError::TimedOut))
	}

	pub fn is_dropped(&self) -> bool {
		matches!(self.kind, ErrorKind::Join(JoinError::Dropped))
	}
}

impl From<JoinError> for Error {
	#[inline]
	fn from(e: JoinError) -> Self {
		Self {
			kind:      ErrorKind::Join(e),
			backtrace: Backtrace::new_unresolved(),
		}
	}
}

impl From<sqlx::Error> for Error {
	#[inline]
	fn from(e: sqlx::Error) -> Self {
		Self {
			kind:      ErrorKind::Database(e),
			backtrace: Backtrace::new_unresolved(),
		}
	}
}

impl From<serenity::Error> for Error {
	#[inline]
	fn from(e: serenity::Error) -> Self {
		Self {
			kind:      ErrorKind::Serenity(e),
			backtrace: Backtrace::new_unresolved(),
		}
	}
}

impl From<scripty_redis::TransactionError> for Error {
	#[inline]
	fn from(e: scripty_redis::TransactionError) -> Self {
		Self {
			kind:      ErrorKind::Redis(e),
			backtrace: Backtrace::new_unresolved(),
		}
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match &self.kind {
			ErrorKind::Join(e) => write!(f, "JoinError: {}", e),
			ErrorKind::Database(e) => write!(f, "DatabaseError: {}", e),
			ErrorKind::Serenity(e) => write!(f, "SerenityError: {}", e),
			ErrorKind::NoWebhookToken => write!(f, "No webhook token found"),
			ErrorKind::Redis(e) => write!(f, "RedisError: {}", e),
		}
	}
}

impl std::error::Error for Error {}

impl fmt::Debug for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("Error").field("kind", &self.kind).finish()
	}
}
