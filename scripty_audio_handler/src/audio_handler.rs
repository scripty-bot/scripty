use crate::events::*;
use crate::types::{
    ActiveUserSet, NextUserList, SsrcAudioMap, SsrcIgnoredMap, SsrcLastPktIdMap, SsrcStreamMap,
    SsrcUserDataMap, SsrcUserIdMap,
};
use ahash::RandomState;
use dashmap::{DashMap, DashSet};
use parking_lot::RwLock;
use serenity::client::Context;
use serenity::model::id::GuildId;
use serenity::model::webhook::Webhook;
use songbird::{Event, EventContext, EventHandler};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct AudioHandler {
    ssrc_user_id_map: SsrcUserIdMap,
    ssrc_stream_map: SsrcStreamMap,
    ssrc_user_data_map: SsrcUserDataMap,
    ssrc_audio_map: SsrcAudioMap,
    ssrc_ignored_map: SsrcIgnoredMap,
    ssrc_last_pkt_id_map: SsrcLastPktIdMap,
    active_user_set: ActiveUserSet,
    next_user_list: NextUserList,
    guild_id: GuildId,
    webhook: Arc<Webhook>,
    context: Context,
    premium_level: Arc<AtomicU8>,
    verbose: Arc<AtomicBool>,
}

impl AudioHandler {
    pub async fn new(
        guild_id: GuildId,
        webhook: Webhook,
        context: Context,
    ) -> Result<Self, sqlx::Error> {
        let this = Self {
            ssrc_user_id_map: Arc::new(DashMap::with_hasher(RandomState::new())),
            ssrc_stream_map: Arc::new(DashMap::with_hasher(RandomState::new())),
            ssrc_user_data_map: Arc::new(DashMap::with_hasher(RandomState::new())),
            ssrc_audio_map: Arc::new(DashMap::with_hasher(RandomState::new())),
            ssrc_ignored_map: Arc::new(DashMap::with_hasher(RandomState::new())),
            ssrc_last_pkt_id_map: Arc::new(DashMap::with_hasher(RandomState::new())),
            active_user_set: Arc::new(DashSet::with_hasher(RandomState::new())),
            next_user_list: Arc::new(RwLock::new(VecDeque::with_capacity(10))),
            guild_id,
            webhook: Arc::new(webhook),
            context,
            premium_level: Arc::new(AtomicU8::new(0)),
            verbose: Arc::new(AtomicBool::new(false)),
        };
        this.reload_config().await?;
        Ok(this)
    }

    pub async fn reload_config(&self) -> Result<(), sqlx::Error> {
        let db = scripty_db::get_db();
        let guild_res = sqlx::query!(
            "SELECT be_verbose, premium_level FROM guilds WHERE guild_id = $1",
            self.guild_id.0 as i64
        )
        .fetch_one(db)
        .await?;

        self.verbose.store(guild_res.be_verbose, Ordering::Relaxed);
        self.premium_level
            .store(guild_res.premium_level as u8, Ordering::Relaxed);

        Ok(())
    }
}

#[async_trait::async_trait]
impl EventHandler for AudioHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            EventContext::SpeakingStateUpdate(state_update) => tokio::spawn(speaking_state_update(
                *state_update,
                self.context.clone(),
                Arc::clone(&self.ssrc_user_id_map),
                Arc::clone(&self.ssrc_user_data_map),
                Arc::clone(&self.ssrc_ignored_map),
            )),
            EventContext::SpeakingUpdate(update) => tokio::spawn(speaking_update(
                update.clone(),
                self.context.clone(),
                Arc::clone(&self.webhook),
                Arc::clone(&self.ssrc_user_data_map),
                Arc::clone(&self.ssrc_stream_map),
                Arc::clone(&self.ssrc_audio_map),
                Arc::clone(&self.verbose),
            )),
            EventContext::VoicePacket(voice_data) => tokio::spawn(voice_packet(
                voice_data.audio.clone(),
                voice_data.packet.ssrc,
                voice_data.packet.sequence.0 .0,
                Arc::clone(&self.ssrc_stream_map),
                Arc::clone(&self.ssrc_audio_map),
                Arc::clone(&self.ssrc_ignored_map),
                Arc::clone(&self.ssrc_last_pkt_id_map),
            )),
            // so guess what?
            // discord, in their infinite wisdom, randomly removed ClientConnect
            // great job shitcord
            /*
            EventContext::ClientConnect(_) => tokio::spawn(client_connect(
                *client_connect_data,
                self.context.clone(),
                Arc::clone(&self.ssrc_user_id_map),
                Arc::clone(&self.ssrc_user_data_map),
                Arc::clone(&self.ssrc_ignored_map),
                Arc::clone(&self.premium_level),
                Arc::clone(&self.active_user_set),
                Arc::clone(&self.next_user_list),
            )),
            */
            EventContext::ClientDisconnect(client_disconnect_data) => {
                tokio::spawn(client_disconnect(
                    *client_disconnect_data,
                    Arc::clone(&self.ssrc_user_id_map),
                    Arc::clone(&self.ssrc_stream_map),
                    Arc::clone(&self.ssrc_user_data_map),
                    Arc::clone(&self.ssrc_ignored_map),
                    Arc::clone(&self.active_user_set),
                    Arc::clone(&self.next_user_list),
                    Arc::clone(&self.premium_level),
                ))
            }
            EventContext::DriverConnect(connect_data)
            | EventContext::DriverReconnect(connect_data) => tokio::spawn(driver_connect(
                connect_data.session_id.to_owned(),
                connect_data.guild_id,
                connect_data.ssrc,
                Arc::clone(&self.ssrc_ignored_map),
            )),
            EventContext::DriverDisconnect(disconnect_data) => tokio::spawn(driver_disconnect(
                disconnect_data.guild_id,
                disconnect_data.reason,
                self.context.clone(),
                Arc::clone(&self.webhook),
            )),
            _ => return None,
        };
        None
    }
}
