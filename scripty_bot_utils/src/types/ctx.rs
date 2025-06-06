use scripty_error::Error;

use crate::Data;

pub type Context<'a> = poise::Context<'a, Data, Error>;
