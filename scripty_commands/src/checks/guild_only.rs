use crate::{Context, Error};

pub async fn is_guild(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.guild_id().is_some())
}
