use crate::Error;
use scripty_premium::PremiumTierList;
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
            .create_webhook(&ctx, CreateWebhook::new("Scripty Transcriptions"))
            .await?
    };

    // automatically leave after the specified time period
    let premium_tier = scripty_premium::get_guild(guild_id.0).await;
    let leave_delta = match premium_tier {
        Some(PremiumTierList::None) => 1800, // leave after 1800 seconds (30 minutes) on free tier
        Some(PremiumTierList::Tier1) => 3600, // leave after 3600 seconds (1 hour) on tier 1
        Some(PremiumTierList::Tier2) => 10800, // leave after 10800 seconds (3 hours) on tier 2
        Some(PremiumTierList::Tier3) => 21600, // leave after 21600 seconds (6 hours) on tier 3
        Some(PremiumTierList::Tier4) => 43200, // leave after 43200 seconds (12 hours) on tier 4
        Some(PremiumTierList::Tier5) => 86400, // leave after 86400 seconds (24 hours) on tier 5
        Some(PremiumTierList::Tier6) => 604800, // leave after 604800 seconds (7 days) on tier 6
        None => 1800, // we don't know the tier, so we'll just leave after 30 minutes
    };
    debug!("leave delta: {}", leave_delta);

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

    // spawn background tasks to automatically leave the call after the specified time period
    let sb2 = songbird::get(&ctx).await.expect("songbird not initialized");
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(leave_delta)).await;
        if let Err(e) = sb2.remove(guild_id).await {
            error!("failed to leave call: {}", e);
        }
    });

    Ok(true)
}
