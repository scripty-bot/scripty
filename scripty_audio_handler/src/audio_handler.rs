use crate::events::*;
use crate::types::{ActiveUserSet, NextUserList, SsrcIgnoredMap, SsrcStreamMap, SsrcUserIdMap};
use ahash::RandomState;
use dashmap::{DashMap, DashSet};
use parking_lot::RwLock;
use serenity::client::Context;
use serenity::model::id::GuildId;
use serenity::model::webhook::Webhook;
use songbird::model::id::UserId;
use songbird::{Event, EventContext, EventHandler};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;

pub struct AudioHandler {
    ssrc_user_id_map: SsrcUserIdMap,
    ssrc_stream_map: SsrcStreamMap,
    ssrc_ignored_map: SsrcIgnoredMap,
    active_user_set: ActiveUserSet,
    next_user_list: NextUserList,
    guild_id: GuildId,
    webhook: Arc<Webhook>,
    context: Context,
    premium_level: AtomicU8,
    verbose: AtomicBool,
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
            ssrc_ignored_map: Arc::new(DashMap::with_hasher(RandomState::new())),
            active_user_set: Arc::new(DashSet::with_hasher(RandomState::new())),
            next_user_list: Arc::new(RwLock::new(Vec::with_capacity(0))),
            guild_id,
            webhook: Arc::new(webhook),
            context,
            premium_level: AtomicU8::new(0),
            verbose: AtomicBool::new(false),
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
                Arc::clone(&self.ssrc_user_id_map),
                Arc::clone(&self.ssrc_stream_map),
            )),
            EventContext::VoicePacket(voice_data) => {
                tokio::spawn(voice_packet(voice_data, Arc::clone(&self.ssrc_stream_map)))
            }
            EventContext::ClientConnect(client_connect) => tokio::spawn(client_connect(
                client_connect,
                Arc::clone(&self.context),
                Arc::clone(&self.ssrc_user_id_map),
                Arc::clone(&self.ssrc_stream_map),
            )),
            EventContext::ClientDisconnect(client_disconnect) => tokio::spawn(client_disconnect(
                client_disconnect,
                Arc::clone(&self.context),
                Arc::clone(&self.ssrc_user_id_map),
                Arc::clone(&self.ssrc_stream_map),
            )),
            EventContext::DriverConnect(connect_data) => tokio::spawn(driver_connect(
                connect_data,
                Arc::clone(&self.context),
                Arc::clone(&self.ssrc_user_id_map),
                Arc::clone(&self.ssrc_stream_map),
            )),
            EventContext::DriverReconnect(connect_data) => tokio::spawn(driver_reconnect(
                connect_data,
                Arc::clone(&self.context),
                Arc::clone(&self.ssrc_user_id_map),
                Arc::clone(&self.ssrc_stream_map),
            )),
            EventContext::DriverDisconnect(disconnect_data) => tokio::spawn(driver_disconnect(
                disconnect_data,
                Arc::clone(&self.context),
                Arc::clone(&self.ssrc_user_id_map),
                Arc::clone(&self.ssrc_stream_map),
            )),
            _ => {}
        }
        None
    }
}
