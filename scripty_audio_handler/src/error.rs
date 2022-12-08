use scripty_db::sqlx;
use songbird::error::JoinError;

#[derive(Debug)]
pub enum Error {
    Join(JoinError),
    Database(sqlx::Error),
    Serenity(serenity::Error),
}

impl From<JoinError> for Error {
    #[inline]
    fn from(e: JoinError) -> Self {
        Self::Join(e)
    }
}

impl From<sqlx::Error> for Error {
    #[inline]
    fn from(e: sqlx::Error) -> Self {
        Self::Database(e)
    }
}

impl From<serenity::Error> for Error {
    #[inline]
    fn from(e: serenity::Error) -> Self {
        Self::Serenity(e)
    }
}
