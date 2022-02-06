use crate::events::*;
use crate::types::{
    ActiveUserSet, NextUserList, SsrcIgnoredMap, SsrcStreamMap, SsrcUserDataMap, SsrcUserIdMap,
};
use ahash::RandomState;
use dashmap::{DashMap, DashSet};
use parking_lot::RwLock;
use serenity::client::Context;
use serenity::model::id::GuildId;
use serenity::model::webhook::Webhook;
use songbird::model::id::UserId;
use songbird::{Event, EventContext, EventHandler};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;

pub struct AudioHandler {
    ssrc_user_id_map: SsrcUserIdMap,
    ssrc_stream_map: SsrcStreamMap,
    ssrc_user_data_map: SsrcUserDataMap,
    ssrc_ignored_map: SsrcIgnoredMap,
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
    ) -> Result<Self, scripty_db::Error> {
        let this = Self {
            ssrc_user_id_map: Arc::new(DashMap::with_hasher(RandomState::new())),
            ssrc_stream_map: Arc::new(DashMap::with_hasher(RandomState::new())),
            ssrc_user_data_map: Arc::new(DashMap::with_hasher(RandomState::new())),
            ssrc_ignored_map: Arc::new(DashMap::with_hasher(RandomState::new())),
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

    pub async fn reload_config(&self) -> Result<(), scripty_db::Error> {
        let db = scripty_db::get_db();
        let guild_res = scripty_db::query!(
            "SELECT (be_verbose, premium_level) FROM guilds WHERE guild_id = $1",
            self.guild_id.0
        )
        .fetch_one(db)
        .await?;

        self.verbose.store(guild_res.verbose, Ordering::Relaxed);
        self.premium_level
            .store(guild_res.premium_level, Ordering::Relaxed);

        Ok(())
    }
}

#[async_trait::async_trait]
impl EventHandler for AudioHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            EventContext::SpeakingStateUpdate(state_update) => tokio::spawn(speaking_state_update(
                state_update,
                self.context.clone(),
                Arc::clone(&self.ssrc_user_id_map),
                Arc::clone(&self.ssrc_ignored_map),
            )),
            EventContext::SpeakingUpdate(update) => tokio::spawn(speaking_update(
                update,
                self.context.clone(),
                Arc::clone(&self.webhook),
                Arc::clone(&self.ssrc_user_data_map),
                Arc::clone(&self.ssrc_stream_map),
            )),
            EventContext::VoicePacket(voice_data) => tokio::spawn(voice_packet(
                voice_data,
                Arc::clone(&self.ssrc_stream_map),
                Arc::clone(&self.ssrc_ignored_map),
            )),
            EventContext::ClientConnect(client_connect_data) => tokio::spawn(client_connect(
                client_connect_data,
                self.context.clone(),
                Arc::clone(&self.ssrc_user_id_map),
                Arc::clone(&self.ssrc_ignored_map),
                Arc::clone(&self.premium_level),
                Arc::clone(&self.active_user_set),
                Arc::clone(&self.next_user_list),
            )),
            EventContext::ClientDisconnect(client_disconnect_data) => {
                tokio::spawn(client_disconnect(
                    client_disconnect_data,
                    Arc::clone(&self.ssrc_user_id_map),
                    Arc::clone(&self.ssrc_stream_map),
                    Arc::clone(&self.ssrc_user_data_map),
                    Arc::clone(&self.ssrc_ignored_map),
                    Arc::clone(&self.active_user_set),
                    Arc::clone(&self.next_user_list),
                    Arc::clone(&self.premium_level),
                ))
            }
            EventContext::DriverConnect(connect_data) => tokio::spawn(driver_connect(
                connect_data,
                Arc::clone(&self.ssrc_ignored_map),
            )),
            EventContext::DriverReconnect(connect_data) => tokio::spawn(driver_reconnect(
                connect_data,
                Arc::clone(&self.ssrc_ignored_map),
            )),
            EventContext::DriverDisconnect(disconnect_data) => tokio::spawn(driver_disconnect(
                disconnect_data,
                self.context.clone(),
                Arc::clone(&self.webhook),
            )),
            _ => return None,
        }
        None
    }
}
