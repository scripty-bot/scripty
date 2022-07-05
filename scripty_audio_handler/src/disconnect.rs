use crate::error::Error;
use serenity::client::Context;
use serenity::model::id::GuildId;
use songbird::error::JoinError;

pub async fn disconnect_from_vc(ctx: &Context, guild_id: GuildId) -> Result<bool, Error> {
    let sb = songbird::get(ctx).await.expect("songbird not initialized");
    match sb.remove(guild_id).await {
        Ok(()) => Ok(true),
        Err(JoinError::NoCall) => Ok(false),
        Err(e) => Err(e.into()),
    }
}
