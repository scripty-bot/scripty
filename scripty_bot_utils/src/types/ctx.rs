use crate::{types::Data, Error};

pub type Context<'a> = poise::Context<'a, Data, Error>;
