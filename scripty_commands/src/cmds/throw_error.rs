use crate::{Context, Error};

#[poise::command(prefix_command, hide_in_help)]
pub async fn throw_error(_ctx: Context<'_>) -> Result<(), Error> {
	Err(Error::manual())
}
