use crate::types::Data;
use crate::Error;

pub type Context<'a> = poise::Context<'a, Data, Error>;
