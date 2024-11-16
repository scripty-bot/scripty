use std::{
	borrow::Cow,
	error::Error as StdError,
	fmt::{Debug, Display, Formatter},
};

use backtrace::Backtrace;
use scripty_stt::{ModelError, OpusSourceError};
use serenity::{http::JsonErrorCode, model::channel::ChannelType, prelude::SerenityError};

use crate::generic_audio_message::GenericMessageError;

pub struct Error {
	bt:             Backtrace,
	pub(super) err: ErrorEnum,
}

impl Debug for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Error").field("err", &self.err).finish()
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorEnum {
	Serenity(serenity::Error),
	InvalidChannelType {
		expected: ChannelType,
		got:      ChannelType,
	},
	Db(sqlx::Error),
	ExpectedGuild,
	Join(scripty_audio_handler::JoinError),
	ManualError,
	Redis(scripty_redis::redis::RedisError),
	RedisPool(scripty_redis::PoolError),
	VoiceMessageDecode(OpusSourceError),
	Transcription(ModelError),
	ExpectedPremiumValue,
	AudioTranscription(GenericMessageError),
	CallAlreadyExists,
	KiaiError(scripty_integrations::kiai::KiaiApiError),
	Custom(String),
}

#[allow(dead_code)]
impl Error {
	#[inline]
	pub fn serenity(err: serenity::Error) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::Serenity(err),
		}
	}

	#[inline]
	pub fn invalid_channel_type(expected: ChannelType, got: ChannelType) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::InvalidChannelType { expected, got },
		}
	}

	#[inline]
	pub fn db(err: sqlx::Error) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::Db(err),
		}
	}

	#[inline]
	pub fn expected_guild() -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::ExpectedGuild,
		}
	}

	#[inline]
	pub fn join(err: scripty_audio_handler::JoinError) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::Join(err),
		}
	}

	#[inline]
	pub fn manual() -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::ManualError,
		}
	}

	#[inline]
	pub fn redis(err: scripty_redis::redis::RedisError) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::Redis(err),
		}
	}

	#[inline]
	pub fn redis_pool(err: scripty_redis::PoolError) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::RedisPool(err),
		}
	}

	#[inline]
	pub fn voice_message_decode(err: OpusSourceError) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::VoiceMessageDecode(err),
		}
	}

	#[inline]
	pub fn transcription(err: ModelError) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::Transcription(err),
		}
	}

	#[inline]
	pub fn expected_premium_value() -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::ExpectedPremiumValue,
		}
	}

	#[inline]
	pub fn audio_transcription(err: GenericMessageError) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::AudioTranscription(err),
		}
	}

	#[inline]
	pub fn custom(err: String) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::Custom(err),
		}
	}

	#[inline]
	pub fn call_already_exists() -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::CallAlreadyExists,
		}
	}

	#[inline]
	pub fn backtrace(&mut self) -> &Backtrace {
		self.bt.resolve();
		&self.bt
	}

	/// Whether a command handler should actually handle this error and note it for the user,
	/// or if it should silently be ignored.
	///
	/// Returns true if the error should be handled, false if it should be ignored.
	pub fn should_handle(&self) -> bool {
		match &self.err {
			ErrorEnum::Serenity(SerenityError::Http(
				serenity::http::HttpError::UnsuccessfulRequest(serenity::http::ErrorResponse {
					error:
						serenity::http::DiscordJsonError {
							code: JsonErrorCode::UnknownInteraction,
							..
						},
					..
				}),
			)) => {
				// ignore unknown interaction errors
				false
			}
			_ => true,
		}
	}

	/// If this is a user error. If it is, this should be handled in a different way to
	/// return a user-friendly error message.
	///
	/// Returns true if this is a user error, false if it is not.
	pub fn is_user_error(&self) -> bool {
		matches!(
			&self.err,
			ErrorEnum::ExpectedGuild
				| ErrorEnum::InvalidChannelType { .. }
				| ErrorEnum::CallAlreadyExists
		)
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		use self::ErrorEnum::*;
		let res: Cow<str> = match &self.err {
			Serenity(e) => format!("Discord/wrapper returned an error: {}", e).into(),
			InvalidChannelType { expected, got } => format!(
				"Got an invalid channel type {:?} when expected {:?}",
				got, expected
			)
			.into(),
			Db(e) => format!("Database returned an error: {:?}", e).into(),
			// _ => "an unknown error happened".into(),
			ExpectedGuild => "expected this to be in a guild".into(),
			Join(e) => format!("failed to join VC: {}", e).into(),
			ManualError => "manual error".into(),
			Redis(e) => format!("Redis returned an error: {}", e).into(),
			RedisPool(e) => format!("Redis pool returned an error: {}", e).into(),
			VoiceMessageDecode(e) => format!(
				"Failed to decode voice message: {}\nNote: this error can be caused by uploading \
				 custom voice message files. Scripty will only attempt to decode those uploaded \
				 with the same format as the official client.",
				e
			)
			.into(),
			Transcription(e) => format!("STT model returned an error: {}", e).into(),
			ExpectedPremiumValue => {
				"Expected a response from Premium service, got none. Try again later.".into()
			}
			Custom(e) => format!("Custom error: {}", e).into(),
			AudioTranscription(e) => format!("Failed to transcribe audio message: {}", e).into(),
			KiaiError(e) => format!("Kiai API error: {}", e).into(),
			CallAlreadyExists => "a call for this channel already exists - not trying to rejoin \
			                      the same channel"
				.into(),
		};
		f.write_str(res.as_ref())
	}
}

impl StdError for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		use self::ErrorEnum::*;
		match &self.err {
			Serenity(e) => Some(e),
			InvalidChannelType { .. } => None,
			Db(e) => Some(e),
			ExpectedGuild => None,
			Join(e) => Some(e),
			ManualError => None,
			Redis(e) => Some(e),
			RedisPool(e) => Some(e),
			VoiceMessageDecode(e) => Some(e),
			Transcription(e) => Some(e),
			ExpectedPremiumValue => None,
			AudioTranscription(e) => Some(e),
			KiaiError(e) => Some(e),
			CallAlreadyExists => None,
			Custom(_) => None,
		}
	}
}

impl From<serenity::Error> for Error {
	#[inline]
	fn from(e: serenity::Error) -> Self {
		Self {
			err: ErrorEnum::Serenity(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<sqlx::Error> for Error {
	#[inline]
	fn from(e: sqlx::Error) -> Self {
		Self {
			err: ErrorEnum::Db(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<scripty_audio_handler::Error> for Error {
	#[inline]
	fn from(e: scripty_audio_handler::Error) -> Self {
		let mut err = match e.kind {
			scripty_audio_handler::ErrorKind::Join(e) => Self::join(e),
			scripty_audio_handler::ErrorKind::Database(e) => Self::db(e),
			scripty_audio_handler::ErrorKind::Serenity(e) => Self::serenity(e),
			scripty_audio_handler::ErrorKind::Redis(scripty_redis::TransactionError::Redis(e)) => {
				Self::redis(e)
			}
			scripty_audio_handler::ErrorKind::Redis(scripty_redis::TransactionError::Deadpool(
				e,
			)) => Self::redis_pool(e),
			scripty_audio_handler::ErrorKind::NoWebhookToken => {
				Self::custom("No webhook token found".to_string())
			}
			scripty_audio_handler::ErrorKind::AlreadyExists => Self::call_already_exists(),
		};
		err.bt = e.backtrace;
		err
	}
}

impl From<scripty_audio_handler::JoinError> for Error {
	#[inline]
	fn from(e: scripty_audio_handler::JoinError) -> Self {
		Self {
			err: ErrorEnum::Join(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<scripty_redis::redis::RedisError> for Error {
	#[inline]
	fn from(e: scripty_redis::redis::RedisError) -> Self {
		Self {
			err: ErrorEnum::Redis(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<scripty_redis::PoolError> for Error {
	#[inline]
	fn from(e: scripty_redis::PoolError) -> Self {
		Self {
			err: ErrorEnum::RedisPool(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<String> for Error {
	#[inline]
	fn from(e: String) -> Self {
		Self {
			err: ErrorEnum::Custom(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<OpusSourceError> for Error {
	#[inline]
	fn from(e: OpusSourceError) -> Self {
		Self {
			err: ErrorEnum::VoiceMessageDecode(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<ModelError> for Error {
	#[inline]
	fn from(e: ModelError) -> Self {
		Self {
			err: ErrorEnum::Transcription(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<GenericMessageError> for Error {
	#[inline]
	fn from(e: GenericMessageError) -> Self {
		Self {
			err: ErrorEnum::AudioTranscription(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<scripty_redis::TransactionError> for Error {
	#[inline]
	fn from(e: scripty_redis::TransactionError) -> Self {
		match e {
			scripty_redis::TransactionError::Deadpool(e) => Self::from(e),
			scripty_redis::TransactionError::Redis(e) => Self::from(e),
		}
	}
}

impl From<scripty_integrations::kiai::KiaiApiError> for Error {
	fn from(e: scripty_integrations::kiai::KiaiApiError) -> Self {
		Self {
			err: ErrorEnum::KiaiError(e),
			bt:  Backtrace::new(),
		}
	}
}
