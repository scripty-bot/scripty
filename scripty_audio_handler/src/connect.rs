use crate::Error;
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::prelude::Webhook;
use serenity::prelude::Context;
use songbird::driver::retry::Retry;
use songbird::driver::{CryptoMode, DecodeMode};
use songbird::error::{JoinError, JoinResult};
use songbird::events::Event;
use songbird::{Call, Config, CoreEvent};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn connect_to_vc(
    ctx: Context,
    guild_id: GuildId,
    channel_id: ChannelId,
    force: bool,
) -> Result<(), Error> {
    let sb = songbird::get(&ctx).await.expect("songbird not initialized");

    let db = scripty_db::get_db();
    let res = scripty_db::query!(
        "SELECT webhook_id, webhook_token FROM channels WHERE channel_id = $1",
        channel_id.0
    )
    .fetch_one(db)
    .await?;

    let webhook = ctx
        .http
        .get_webhook_with_token(res.webhook_id, res.webhook_token)
        .await?;

    let handler = crate::AudioHandler::new(guild_id, webhook, ctx.clone()).await?;

    let call_lock = match sb.join(guild_id, channel_id).await {
        (r, Ok(_)) => r,
        (_, Err(e)) => return Err(e.into()),
    };

    let mut call = call_lock.lock().await;

    call.mute(true).await?;

    call.add_global_event(Event::Core(CoreEvent::SpeakingStateUpdate), handler.clone());
    call.add_global_event(Event::Core(CoreEvent::SpeakingUpdate), handler.clone());
    call.add_global_event(Event::Core(CoreEvent::VoicePacket), handler.clone());
    call.add_global_event(Event::Core(CoreEvent::ClientConnect), handler.clone());
    call.add_global_event(Event::Core(CoreEvent::ClientDisconnect), handler.clone());
    call.add_global_event(Event::Core(CoreEvent::DriverConnect), handler.clone());
    call.add_global_event(Event::Core(CoreEvent::DriverDisconnect), handler.clone());
    call.add_global_event(Event::Core(CoreEvent::DriverReconnect), handler);

    Ok(())
}
