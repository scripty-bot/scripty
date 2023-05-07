use backtrace::Backtrace;
use scripty_audio_handler::JoinError;
use serenity::model::channel::ChannelType;
use serenity::prelude::SerenityError;
use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {
    bt: Backtrace,
    pub(super) err: ErrorEnum,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorEnum {
    Serenity(serenity::Error),
    InvalidChannelType {
        expected: ChannelType,
        got: ChannelType,
    },
    Db(sqlx::Error),
    ExpectedGuild,
    Join(JoinError),
    ManualError,
    Redis(scripty_redis::redis::RedisError),
    RedisPool(scripty_redis::PoolError),
    Custom(String),
}

#[allow(dead_code)]
impl Error {
    #[inline]
    pub fn serenity(err: serenity::Error) -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::Serenity(err),
        }
    }

    #[inline]
    pub fn invalid_channel_type(expected: ChannelType, got: ChannelType) -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::InvalidChannelType { expected, got },
        }
    }

    #[inline]
    pub fn db(err: sqlx::Error) -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::Db(err),
        }
    }

    #[inline]
    pub fn expected_guild() -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::ExpectedGuild,
        }
    }

    #[inline]
    pub fn join(err: JoinError) -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::Join(err),
        }
    }

    #[inline]
    pub fn manual() -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::ManualError,
        }
    }

    #[inline]
    pub fn redis(err: scripty_redis::redis::RedisError) -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::Redis(err),
        }
    }

    #[inline]
    pub fn redis_pool(err: scripty_redis::PoolError) -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::RedisPool(err),
        }
    }

    #[inline]
    pub fn custom(err: String) -> Self {
        Error {
            bt: Backtrace::new(),
            err: ErrorEnum::Custom(err),
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
                    error: serenity::http::DiscordJsonError { code, .. },
                    ..
                }),
            )) if code == &10062 => {
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
            ErrorEnum::ExpectedGuild | ErrorEnum::InvalidChannelType { .. }
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
            Custom(e) => format!("Custom error: {}", e).into(),
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
            Custom(_) => None,
        }
    }
}

impl From<serenity::Error> for Error {
    #[inline]
    fn from(e: serenity::Error) -> Self {
        Self {
            err: ErrorEnum::Serenity(e),
            bt: Backtrace::new(),
        }
    }
}

impl From<sqlx::Error> for Error {
    #[inline]
    fn from(e: sqlx::Error) -> Self {
        Self {
            err: ErrorEnum::Db(e),
            bt: Backtrace::new(),
        }
    }
}

impl From<scripty_audio_handler::Error> for Error {
    #[inline]
    fn from(e: scripty_audio_handler::Error) -> Self {
        match e {
            scripty_audio_handler::Error::Join(e) => Self::join(e),
            scripty_audio_handler::Error::Database(e) => Self::db(e),
            scripty_audio_handler::Error::Serenity(e) => Self::serenity(e),
        }
    }
}

impl From<scripty_redis::redis::RedisError> for Error {
    #[inline]
    fn from(e: scripty_redis::redis::RedisError) -> Self {
        Self {
            err: ErrorEnum::Redis(e),
            bt: Backtrace::new(),
        }
    }
}

impl From<scripty_redis::PoolError> for Error {
    #[inline]
    fn from(e: scripty_redis::PoolError) -> Self {
        Self {
            err: ErrorEnum::RedisPool(e),
            bt: Backtrace::new(),
        }
    }
}

impl From<String> for Error {
    #[inline]
    fn from(e: String) -> Self {
        Self {
            err: ErrorEnum::Custom(e),
            bt: Backtrace::new(),
        }
    }
}