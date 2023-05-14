use std::{
	collections::VecDeque,
	sync::{
		atomic::{AtomicBool, AtomicU8, Ordering},
		Arc,
	},
};

use ahash::RandomState;
use dashmap::{DashMap, DashSet};
use parking_lot::RwLock;
use serenity::{
	client::Context,
	model::{
		id::{ChannelId, GuildId},
		webhook::Webhook,
	},
};
use songbird::{Event, EventContext, EventHandler};

use crate::{
	events::*,
	types::{
		ActiveUserSet,
		NextUserList,
		SsrcIgnoredMap,
		SsrcSpeakingSet,
		SsrcStreamMap,
		SsrcUserDataMap,
		SsrcUserIdMap,
		SsrcVoiceIngestMap,
	},
};

pub struct SsrcMaps {
	pub ssrc_user_id_map:      SsrcUserIdMap,
	pub ssrc_stream_map:       SsrcStreamMap,
	pub ssrc_user_data_map:    SsrcUserDataMap,
	pub ssrc_ignored_map:      SsrcIgnoredMap,
	pub ssrc_voice_ingest_map: SsrcVoiceIngestMap,
	pub ssrc_speaking_set:     SsrcSpeakingSet,
	pub active_user_set:       ActiveUserSet,
	pub next_user_list:        NextUserList,
}
pub type ArcSsrcMaps = Arc<SsrcMaps>;

#[derive(Clone)]
pub struct AudioHandler {
	ssrc_state:       Arc<SsrcMaps>,
	guild_id:         GuildId,
	channel_id:       ChannelId,
	voice_channel_id: ChannelId,
	webhook:          Arc<Webhook>,
	context:          Context,
	premium_level:    Arc<AtomicU8>,
	verbose:          Arc<AtomicBool>,
	language:         Arc<RwLock<String>>,
}

impl AudioHandler {
	pub async fn new(
		guild_id: GuildId,
		webhook: Webhook,
		context: Context,
		channel_id: ChannelId,
		voice_channel_id: ChannelId,
	) -> Result<Self, sqlx::Error> {
		let maps = SsrcMaps {
			ssrc_user_id_map:      DashMap::with_hasher(RandomState::new()),
			ssrc_stream_map:       DashMap::with_hasher(RandomState::new()),
			ssrc_user_data_map:    DashMap::with_hasher(RandomState::new()),
			ssrc_ignored_map:      DashMap::with_hasher(RandomState::new()),
			ssrc_voice_ingest_map: DashMap::with_hasher(RandomState::new()),
			ssrc_speaking_set:     DashSet::with_hasher(RandomState::new()),
			active_user_set:       DashSet::with_hasher(RandomState::new()),
			next_user_list:        RwLock::new(VecDeque::with_capacity(10)),
		};
		let this = Self {
			ssrc_state: Arc::new(maps),
			guild_id,
			channel_id,
			voice_channel_id,
			webhook: Arc::new(webhook),
			context,
			premium_level: Arc::new(AtomicU8::new(0)),
			verbose: Arc::new(AtomicBool::new(false)),
			language: Arc::new(Default::default()),
		};
		this.reload_config().await?;

		let t2 = this.clone();
		tokio::spawn(async move {
			const RELOAD_TIME: std::time::Duration = std::time::Duration::from_secs(300);

			loop {
				tokio::time::sleep(RELOAD_TIME).await;
				if let Err(e) = t2.reload_config().await {
					error!("failed to reload config: {:?}", e);
				};

				if Arc::<_>::strong_count(&t2.verbose) == 1 {
					// this is the last strong pointer because all the others have been dropped
					break;
				}
			}
		});

		Ok(this)
	}

	pub async fn reload_config(&self) -> Result<(), sqlx::Error> {
		let db = scripty_db::get_db();
		let guild_res = sqlx::query!(
			"SELECT be_verbose FROM guilds WHERE guild_id = $1",
			self.guild_id.get() as i64
		)
		.fetch_one(db)
		.await?;

		self.verbose.store(guild_res.be_verbose, Ordering::Relaxed);

		if let Some(lvl) = scripty_premium::get_guild(self.guild_id.0).await {
			self.premium_level.store(lvl as u8, Ordering::Relaxed)
		}

		let mut lang = sqlx::query!(
			"SELECT language FROM guilds WHERE guild_id = $1",
			self.guild_id.get() as i64
		)
		.fetch_one(db)
		.await?
		.language;
		std::mem::swap(&mut *self.language.write(), &mut lang);

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
				Arc::clone(&self.ssrc_state),
			)),
			EventContext::VoiceTick(voice_data) => tokio::spawn(voice_tick(
				voice_data.clone(),
				Arc::clone(&self.ssrc_state),
				self.language.clone(),
				self.verbose.clone(),
				self.context.clone(),
				Arc::clone(&self.webhook),
			)),
			EventContext::ClientDisconnect(client_disconnect_data) => {
				tokio::spawn(client_disconnect(
					*client_disconnect_data,
					Arc::clone(&self.ssrc_state),
					Arc::clone(&self.premium_level),
				))
			}
			EventContext::DriverConnect(connect_data)
			| EventContext::DriverReconnect(connect_data) => tokio::spawn(driver_connect(
				connect_data.session_id.to_owned(),
				connect_data.guild_id,
				connect_data.ssrc,
				Arc::clone(&self.ssrc_state),
			)),
			EventContext::DriverDisconnect(disconnect_data) => tokio::spawn(driver_disconnect(
				disconnect_data.guild_id,
				disconnect_data.reason,
				self.context.clone(),
				Arc::clone(&self.webhook),
				self.channel_id,
				self.voice_channel_id,
			)),
			_ => return None,
		};
		None
	}
}
