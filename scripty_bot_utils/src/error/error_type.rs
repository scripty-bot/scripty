use std::{
	borrow::Cow,
	error::Error as StdError,
	fmt::{Debug, Display, Formatter},
};

use backtrace::Backtrace;
use http::header::ToStrError;
use scripty_error::{FileTranscriptError, SttServerError};
use scripty_stt::OpusSourceError;
use serenity::{http::JsonErrorCode, model::channel::ChannelType, prelude::SerenityError};

pub struct Error {
	bt:             Backtrace,
	pub(super) err: ErrorEnum,
}

impl Debug for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Error")
			.field("err", &self.err)
			.finish_non_exhaustive()
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorEnum {
	Serenity(serenity::Error),
	InvalidChannelType {
		expected: Vec<ChannelType>,
		got:      ChannelType,
	},
	Db(sqlx::Error),
	ExpectedGuild,
	ExpectedChannel,
	Join(scripty_audio_handler::JoinError),
	Reqwest(reqwest::Error),
	ToStr(ToStrError),
	ManualError,
	Redis(scripty_redis::redis::RedisError),
	RedisPool(scripty_redis::PoolError),
	VoiceMessageDecode(OpusSourceError),
	Transcription(SttServerError),
	ExpectedPremiumValue,
	AudioTranscription(FileTranscriptError),
	CallAlreadyExists,
	KiaiError(scripty_integrations::kiai::KiaiApiError),
	NoGuildDefaults,
	BadDiscordState,
	BackgroundTaskFailure(tokio::task::JoinError),
	MissingInteractionContext,
	MissingGlobalData,
	Custom(String),
}

#[allow(dead_code)]
impl Error {
	pub fn serenity(err: serenity::Error) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::Serenity(err),
		}
	}

	pub fn invalid_channel_type(expected: Vec<ChannelType>, got: ChannelType) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::InvalidChannelType { expected, got },
		}
	}

	pub fn db(err: sqlx::Error) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::Db(err),
		}
	}

	pub fn expected_guild() -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::ExpectedGuild,
		}
	}

	pub fn expected_channel() -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::ExpectedChannel,
		}
	}

	pub fn join(err: scripty_audio_handler::JoinError) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::Join(err),
		}
	}

	pub fn reqwest(err: reqwest::Error) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::Reqwest(err),
		}
	}

	pub fn to_str(err: ToStrError) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::ToStr(err),
		}
	}

	pub fn manual() -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::ManualError,
		}
	}

	pub fn redis(err: scripty_redis::redis::RedisError) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::Redis(err),
		}
	}

	pub fn redis_pool(err: scripty_redis::PoolError) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::RedisPool(err),
		}
	}

	pub fn voice_message_decode(err: OpusSourceError) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::VoiceMessageDecode(err),
		}
	}

	pub fn transcription(err: SttServerError) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::Transcription(err),
		}
	}

	pub fn expected_premium_value() -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::ExpectedPremiumValue,
		}
	}

	pub fn audio_transcription(err: FileTranscriptError) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::AudioTranscription(err),
		}
	}

	pub fn missing_interaction_context() -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::MissingInteractionContext,
		}
	}

	pub fn missing_global_data() -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::MissingGlobalData,
		}
	}

	pub fn custom(err: String) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::Custom(err),
		}
	}

	pub fn call_already_exists() -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::CallAlreadyExists,
		}
	}

	pub fn no_guild_defaults() -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::NoGuildDefaults,
		}
	}

	pub fn background_task(err: tokio::task::JoinError) -> Self {
		Error {
			bt:  Backtrace::new_unresolved(),
			err: ErrorEnum::BackgroundTaskFailure(err),
		}
	}

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
			ExpectedGuild => "expected this to be in a guild".into(),
			ExpectedChannel => "expected this to be in a channel".into(),
			Join(e) => format!("failed to join VC: {}", e).into(),
			Reqwest(e) => format!("error while sending http request: {}", e).into(),
			ToStr(e) => format!("error decoding http headers into string: {}", e).into(),
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
			MissingGlobalData => "missing global data".into(),
			MissingInteractionContext => "missing interaction context".into(),
			Custom(e) => format!("Custom error: {}", e).into(),
			AudioTranscription(e) => format!("Failed to transcribe audio message: {}", e).into(),
			KiaiError(e) => format!("Kiai API error: {}", e).into(),
			CallAlreadyExists => "a call for this channel already exists - not trying to rejoin \
			                      the same channel - run `/leave` if this is wrong"
				.into(),
			NoGuildDefaults => "no default configuration exists for this server".into(),
			BadDiscordState => "Discord sent us bad data".into(),
			BackgroundTaskFailure(e) => format!("Background thread panicked: {}", e).into(),
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
			ExpectedChannel => None,
			Join(e) => Some(e),
			Reqwest(e) => Some(e),
			ToStr(e) => Some(e),
			ManualError => None,
			Redis(e) => Some(e),
			RedisPool(e) => Some(e),
			VoiceMessageDecode(e) => Some(e),
			Transcription(e) => Some(e),
			ExpectedPremiumValue => None,
			AudioTranscription(e) => Some(e),
			KiaiError(e) => Some(e),
			CallAlreadyExists => None,
			MissingInteractionContext => None,
			MissingGlobalData => None,
			Custom(_) => None,
			NoGuildDefaults => None,
			BadDiscordState => None,
			BackgroundTaskFailure(e) => Some(e),
		}
	}
}

impl From<serenity::Error> for Error {
	fn from(e: serenity::Error) -> Self {
		Self {
			err: ErrorEnum::Serenity(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<sqlx::Error> for Error {
	fn from(e: sqlx::Error) -> Self {
		Self {
			err: ErrorEnum::Db(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<scripty_audio_handler::Error> for Error {
	fn from(
		scripty_audio_handler::Error { kind, backtrace }: scripty_audio_handler::Error,
	) -> Self {
		let err = match kind {
			scripty_audio_handler::ErrorKind::Join(e) => ErrorEnum::Join(e),
			scripty_audio_handler::ErrorKind::Database(e) => ErrorEnum::Db(e),
			scripty_audio_handler::ErrorKind::Serenity(e) => ErrorEnum::Serenity(e),
			scripty_audio_handler::ErrorKind::Redis(scripty_redis::TransactionError::Redis(e)) => {
				ErrorEnum::Redis(e)
			}
			scripty_audio_handler::ErrorKind::Redis(scripty_redis::TransactionError::Deadpool(
				e,
			)) => ErrorEnum::RedisPool(e),
			scripty_audio_handler::ErrorKind::NoWebhookToken => {
				ErrorEnum::Custom("No webhook token found".to_string())
			}
			scripty_audio_handler::ErrorKind::AlreadyExists => ErrorEnum::CallAlreadyExists,
			scripty_audio_handler::ErrorKind::BadDiscordState => ErrorEnum::BadDiscordState,
		};

		Self { err, bt: backtrace }
	}
}

impl From<scripty_audio_handler::JoinError> for Error {
	fn from(e: scripty_audio_handler::JoinError) -> Self {
		Self {
			err: ErrorEnum::Join(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<reqwest::Error> for Error {
	fn from(e: reqwest::Error) -> Self {
		Self {
			err: ErrorEnum::Reqwest(e),
			bt:  Backtrace::new_unresolved(),
		}
	}
}

impl From<ToStrError> for Error {
	fn from(e: ToStrError) -> Self {
		Self {
			err: ErrorEnum::ToStr(e),
			bt:  Backtrace::new_unresolved(),
		}
	}
}

impl From<scripty_redis::redis::RedisError> for Error {
	fn from(e: scripty_redis::redis::RedisError) -> Self {
		Self {
			err: ErrorEnum::Redis(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<scripty_redis::PoolError> for Error {
	fn from(e: scripty_redis::PoolError) -> Self {
		Self {
			err: ErrorEnum::RedisPool(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<String> for Error {
	fn from(e: String) -> Self {
		Self {
			err: ErrorEnum::Custom(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<OpusSourceError> for Error {
	fn from(e: OpusSourceError) -> Self {
		Self {
			err: ErrorEnum::VoiceMessageDecode(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<SttServerError> for Error {
	fn from(e: SttServerError) -> Self {
		Self {
			err: ErrorEnum::Transcription(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<FileTranscriptError> for Error {
	fn from(e: FileTranscriptError) -> Self {
		Self {
			err: ErrorEnum::AudioTranscription(e),
			bt:  Backtrace::new(),
		}
	}
}

impl From<scripty_redis::TransactionError> for Error {
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

impl From<tokio::task::JoinError> for Error {
	fn from(e: tokio::task::JoinError) -> Self {
		Self {
			err: ErrorEnum::BackgroundTaskFailure(e),
			bt:  Backtrace::new_unresolved(),
		}
	}
}
