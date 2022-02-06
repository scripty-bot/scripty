use crate::Error;
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::Context;
use songbird::events::Event;
use songbird::CoreEvent;

// TODO: implement `force`
pub async fn connect_to_vc(
    ctx: Context,
    guild_id: GuildId,
    channel_id: ChannelId,
    _force: bool,
) -> Result<(), Error> {
    let sb = songbird::get(&ctx).await.expect("songbird not initialized");

    let db = scripty_db::get_db();
    let res = sqlx::query!(
        "SELECT webhook_id, webhook_token FROM channels WHERE channel_id = $1",
        channel_id.0 as i64
    )
    .fetch_one(db)
    .await?;

    let webhook = ctx
        .http
        .get_webhook_with_token(res.webhook_id as u64, &res.webhook_token)
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
