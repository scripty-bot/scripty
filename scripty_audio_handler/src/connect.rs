use crate::Error;
use serenity::builder::CreateWebhook;
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::Context;
use songbird::error::JoinError;
use songbird::events::Event;
use songbird::CoreEvent;

// TODO: implement `force`
#[allow(clippy::let_unit_value)]
pub async fn connect_to_vc(
    ctx: Context,
    guild_id: GuildId,
    channel_id: ChannelId,
    voice_channel_id: ChannelId,
    _force: bool,
) -> Result<bool, Error> {
    debug!("fetching webhook");
    let webhook = if let Some(h) = channel_id.webhooks(&ctx).await?.pop() {
        h
    } else {
        channel_id
            .create_webhook(
                &ctx,
                CreateWebhook::default().name("Scripty Transcriptions"),
            )
            .await?
    };

    debug!("fetching songbird");
    let sb = songbird::get(&ctx).await.expect("songbird not initialized");
    debug!("leaving old call");
    match sb.remove(guild_id).await {
        Ok(()) | Err(JoinError::NoCall) => {}
        Err(e) => return Err(e.into()),
    };
    debug!("joining new call");
    let (call_lock, res) = sb.join(guild_id, voice_channel_id).await;
    debug!("getting call res");
    let _: () = res?;

    debug!("locking call");
    let mut call = call_lock.lock().await;

    debug!("muting call");
    call.mute(true).await?;

    debug!("initializing audio handler");
    let handler =
        crate::AudioHandler::new(guild_id, webhook, ctx.clone(), channel_id, voice_channel_id)
            .await?;

    debug!("adding global events");
    call.add_global_event(Event::Core(CoreEvent::SpeakingStateUpdate), handler.clone());
    call.add_global_event(Event::Core(CoreEvent::SpeakingUpdate), handler.clone());
    call.add_global_event(Event::Core(CoreEvent::VoicePacket), handler.clone());
    // aaaaaa i hate discord
    // discord randomly stopped sending the ClientConnect event
    // call.add_global_event(Event::Core(CoreEvent::ClientConnect), handler.clone());
    call.add_global_event(Event::Core(CoreEvent::ClientDisconnect), handler.clone());
    call.add_global_event(Event::Core(CoreEvent::DriverConnect), handler.clone());
    call.add_global_event(Event::Core(CoreEvent::DriverDisconnect), handler.clone());
    call.add_global_event(Event::Core(CoreEvent::DriverReconnect), handler);

    Ok(true)
}
