use crate::{Data, Error};

pub type Context<'a> = poise::Context<'a, Data, Error>;
