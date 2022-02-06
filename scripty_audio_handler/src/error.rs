use songbird::error::JoinError;

pub enum Error {
    Join(JoinError),
    Database(scripty_db::Error),
    Serenity(serenity::Error),
}

impl From<JoinError> for Error {
    #[inline]
    fn from(e: JoinError) -> Self {
        Self::Join(e)
    }
}

impl From<scripty_db::Error> for Error {
    #[inline]
    fn from(e: scripty_db::Error) -> Self {
        Self::Database(e)
    }
}

impl From<serenity::Error> for Error {
    #[inline]
    fn from(e: serenity::Error) -> Self {
        Self::Serenity(e)
    }
}
