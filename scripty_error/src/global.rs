use std::{borrow::Cow, fmt};

use backtrace::Backtrace;
use serenity::{http::JsonErrorCode, prelude::SerenityError};

use crate::{
	FileTranscriptError,
	SttServerError,
	SttServerErrorEnum,
	file_transcript::FileTranscriptErrorEnum,
	internal::error_fn_impl,
};

pub struct Error {
	bt:    Backtrace,
	error: ErrorEnum,
}

impl std::error::Error for ErrorEnum {}
impl Error {
	pub fn backtrace(&mut self) -> &Backtrace {
		self.bt.resolve();
		&self.bt
	}

	/// Whether a command handler should actually handle this error and note it for the user,
	/// or if it should silently be ignored.
	///
	/// Returns true if the error should be handled, false if it should be ignored.
	pub fn should_handle(&self) -> bool {
		match &self.error {
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
			&self.error,
			ErrorEnum::ExpectedGuild | ErrorEnum::ExpectedChannel | ErrorEnum::CallAlreadyExists
		)
	}

	pub fn peek_inner(&self) -> &ErrorEnum {
		&self.error
	}

	pub fn is_dropped_or_timed_out(&self) -> bool {
		matches!(
			self.error,
			ErrorEnum::Join(
				songbird::error::JoinError::Dropped | songbird::error::JoinError::TimedOut
			)
		)
	}
}

impl fmt::Debug for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Error")
			.field("error", &self.error)
			.finish_non_exhaustive()
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.error)
	}
}
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorEnum {
	Serenity(serenity::Error),
	Db(sqlx::Error),
	Join(songbird::error::JoinError),
	Reqwest(reqwest::Error),
	ToStr(reqwest::header::ToStrError),
	Redis(redis::RedisError),
	RedisPool(deadpool_redis::PoolError),
	VoiceMessageDecode(magnum::error::OpusSourceError),
	SttServer(SttServerErrorEnum),
	FileTranscript(FileTranscriptErrorEnum),
	Kiai(scripty_integrations::kiai::KiaiApiError),
	BackgroundTask(tokio::task::JoinError),

	NoGuildDefaults,
	ExpectedGuild,
	ExpectedChannel,
	ExpectedPremiumValue,
	ExpectedInteractionContext,
	ExpectedGlobalData,
	ExpectedWebhookToken,
	CallAlreadyExists,
	BadDiscordState,

	Manual,
	Custom(String),
}

impl fmt::Display for ErrorEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use self::ErrorEnum::*;
		let res: Cow<str> = match &self {
			Serenity(e) => format!("Discord/wrapper returned an error: {}", e).into(),
			Db(e) => format!("Database returned an error: {}", e).into(),
			Join(e) => format!("failed to join VC: {}", e).into(),
			Reqwest(e) => format!("error while sending http request: {}", e).into(),
			ToStr(e) => format!("error decoding http headers into string: {}", e).into(),
			Redis(e) => format!("Redis returned an error: {}", e).into(),
			RedisPool(e) => format!("Redis pool returned an error: {}", e).into(),
			VoiceMessageDecode(e) => format!(
				"Failed to decode voice message: {}\nNote: this error can be caused by uploading \
				 custom voice message files. Scripty will only attempt to decode those uploaded \
				 with the same format as the official client.",
				e
			)
			.into(),
			SttServer(e) => format!("STT model returned an error: {}", e).into(),
			FileTranscript(e) => format!("Failed to transcribe audio message: {}", e).into(),
			BackgroundTask(e) => format!("Background thread panicked: {}", e).into(),
			Kiai(e) => format!("Kiai API error: {}", e).into(),
			ExpectedGuild => "expected this to be in a guild".into(),
			ExpectedChannel => "expected this to be in a channel".into(),
			ExpectedPremiumValue => "Expected a response from Premium service, got none".into(),
			ExpectedGlobalData => "missing global data".into(),
			ExpectedInteractionContext => "missing interaction context".into(),
			ExpectedWebhookToken => "expected webhook token when joining".into(),
			CallAlreadyExists => "a call for this channel already exists - not trying to rejoin \
			                      the same channel - run `/leave` if this is wrong"
				.into(),
			NoGuildDefaults => "no default configuration exists for this server".into(),
			BadDiscordState => "Discord sent us bad data".into(),
			Manual => "manual error".into(),
			Custom(e) => format!("Custom error: {}", e).into(),
		};
		f.write_str(res.as_ref())
	}
}

error_fn_impl!(
	Error, ErrorEnum;
	serenity, Serenity, serenity::Error;
	db, Db, sqlx::Error;
	join, Join, songbird::error::JoinError;
	reqwest, Reqwest, reqwest::Error;
	header_to_str, ToStr, reqwest::header::ToStrError;
	redis, Redis, redis::RedisError;
	redis_pool, RedisPool, deadpool_redis::PoolError;
	voice_message_decode, VoiceMessageDecode, magnum::error::OpusSourceError;
	kiai, Kiai, scripty_integrations::kiai::KiaiApiError;
	background_task, BackgroundTask, tokio::task::JoinError;

	no_guild_defaults, NoGuildDefaults;
	expected_premium_value, ExpectedPremiumValue;
	expected_guild, ExpectedGuild;
	expected_channel, ExpectedChannel;
	expected_interaction_context, ExpectedInteractionContext;
	expected_global_data, ExpectedGlobalData;
	expected_webhook_token, ExpectedWebhookToken;
	call_already_exists, CallAlreadyExists;
	bad_discord_state, BadDiscordState;

	manual, Manual;
	custom, Custom, String, nofrom;
);

impl Error {
	pub fn stt_server(SttServerError { bt, error }: SttServerError) -> Self {
		let error = ErrorEnum::SttServer(error);
		Self { error, bt }
	}

	pub fn file_transcript(FileTranscriptError { bt, error }: FileTranscriptError) -> Self {
		let error = ErrorEnum::FileTranscript(error);
		Self { error, bt }
	}
}

impl From<SttServerError> for Error {
	fn from(error: SttServerError) -> Self {
		Self::stt_server(error)
	}
}

impl From<FileTranscriptError> for Error {
	fn from(error: FileTranscriptError) -> Self {
		Self::file_transcript(error)
	}
}
