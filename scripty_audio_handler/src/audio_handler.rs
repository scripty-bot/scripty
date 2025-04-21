use std::{
	collections::VecDeque,
	sync::{
		Arc,
		atomic::{AtomicBool, AtomicU8, Ordering},
	},
};

use ahash::RandomState;
use dashmap::{DashMap, DashSet};
use parking_lot::RwLock;
use scripty_automod::types::AutomodServerConfig;
use scripty_data_type::{CallDeath, get_data};
use scripty_integrations::kiai::KiaiApiClient;
use serenity::{
	builder::ExecuteWebhook,
	gateway::client::Context,
	model::{
		id::{ChannelId, GuildId, RoleId, ThreadId},
		webhook::Webhook,
	},
};
use songbird::{Event, EventContext, EventHandler};

use crate::{
	error::Error,
	events::*,
	types::{
		ActiveUserSet,
		NextUserList,
		SeenUsers,
		SsrcIgnoredMap,
		SsrcSpeakingSet,
		SsrcStreamMap,
		SsrcUserDataMap,
		SsrcUserIdMap,
		SsrcVoiceIngestMap,
		TranscriptResults,
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

pub struct AudioHandler {
	ssrc_state:           ArcSsrcMaps,
	guild_id:             GuildId,
	channel_id:           ChannelId,
	voice_channel_id:     ChannelId,
	thread_id:            Option<ThreadId>,
	webhook:              Arc<Webhook>,
	context:              Context,
	premium_level:        Arc<AtomicU8>,
	verbose:              Arc<AtomicBool>,
	language:             Arc<RwLock<String>>,
	transcript_results:   TranscriptResults,
	seen_users:           SeenUsers,
	automod_server_cfg:   Arc<AutomodServerConfig>,
	auto_detect_lang:     Arc<AtomicBool>,
	transcribe_only_role: Arc<RwLock<Option<RoleId>>>,
	translate:            Arc<AtomicBool>,
	kiai_enabled:         Arc<AtomicBool>,
	pub kiai_client:      KiaiApiClient,
	ephemeral:            bool,
	alive_call:           CallDeath,
}

impl AudioHandler {
	pub async fn new(
		guild_id: GuildId,
		webhook: Webhook,
		context: Context,
		channel_id: ChannelId,
		voice_channel_id: ChannelId,
		thread_id: Option<ThreadId>,
		record_transcriptions: bool,
		automod_server_cfg: AutomodServerConfig,
		kiai_client: KiaiApiClient,
		ephemeral: bool,
	) -> Result<Self, Error> {
		let ssrc_state = Arc::new(SsrcMaps {
			ssrc_user_id_map:      DashMap::with_capacity_and_hasher(10, RandomState::new()),
			ssrc_stream_map:       DashMap::with_capacity_and_hasher(10, RandomState::new()),
			ssrc_user_data_map:    DashMap::with_capacity_and_hasher(10, RandomState::new()),
			ssrc_ignored_map:      DashMap::with_capacity_and_hasher(10, RandomState::new()),
			ssrc_voice_ingest_map: DashMap::with_capacity_and_hasher(10, RandomState::new()),
			ssrc_speaking_set:     DashSet::with_capacity_and_hasher(10, RandomState::new()),
			active_user_set:       DashSet::with_capacity_and_hasher(10, RandomState::new()),
			next_user_list:        RwLock::new(VecDeque::with_capacity(10)),
		});
		crate::INTERNAL_SSRC_MAPS
			.get_or_init(|| DashMap::with_hasher(RandomState::new()))
			.insert(guild_id, ssrc_state.clone());
		let alive_call = CallDeath::new(
			get_data(&context).existing_calls.clone(),
			guild_id,
			voice_channel_id,
		)
		.ok_or_else(Error::already_exists)?;

		let this = Self {
			ssrc_state,
			guild_id,
			channel_id,
			voice_channel_id,
			thread_id,
			webhook: Arc::new(webhook),
			context,
			premium_level: Arc::new(AtomicU8::new(0)),
			verbose: Arc::new(AtomicBool::new(false)),
			language: Arc::new(Default::default()),
			transcript_results: record_transcriptions.then(|| Arc::new(RwLock::new(Vec::new()))),
			seen_users: record_transcriptions
				.then(|| Arc::new(DashSet::with_hasher(RandomState::new()))),
			automod_server_cfg: Arc::new(automod_server_cfg),
			auto_detect_lang: Arc::new(AtomicBool::new(false)),
			transcribe_only_role: Arc::new(RwLock::new(None)),
			translate: Arc::new(AtomicBool::new(false)),
			kiai_enabled: Arc::new(AtomicBool::new(false)),
			kiai_client,
			ephemeral,
			alive_call,
		};
		this.reload_config().await?;

		let t2 = this.clone();
		tokio::spawn(async move {
			const RELOAD_TIME: std::time::Duration = std::time::Duration::from_secs(300);
			let (tx, mut rx) = tokio::sync::broadcast::channel::<()>(1);
			if let Some(old) = crate::VOICE_HANDLER_UPDATES
				.get_or_init(|| DashMap::with_hasher(RandomState::new()))
				.insert(guild_id, tx)
			{
				// trigger a cleanup
				let _ = old.send(());
			}

			loop {
				tokio::select! {
					val = rx.recv() => {
						if val.is_err() {
							debug!(%guild_id, "all tx handlers for this call dropped");
							return;
						}
						debug!(%guild_id, "got request to reload config for this call");
					}
					_ = tokio::time::sleep(RELOAD_TIME) => {}
				}
				if let Err(e) = t2.reload_config().await {
					error!("failed to reload config: {:?}", e);
				};

				if Arc::<_>::strong_count(&t2.verbose) <= 2 {
					// this is the last strong pointer because all the others have been dropped
					// run cleanup tasks
					if ephemeral && let Some(thread_id) = t2.thread_id {
						let http = &t2.context.http;
						if let Err(e) = thread_id.widen().delete(http, None).await {
							let _ = t2
								.webhook
								.execute(
									http,
									false,
									ExecuteWebhook::new()
										.content(format!("Failed to delete this thread: {}", e)),
								)
								.await;
							error!(%thread_id, "Failed to delete thread: {}", e);
						}
					}

					crate::VOICE_HANDLER_UPDATES
						.get_or_init(|| DashMap::with_hasher(RandomState::new()))
						.remove(&guild_id);
					crate::INTERNAL_SSRC_MAPS
						.get_or_init(|| DashMap::with_hasher(RandomState::new()))
						.remove(&guild_id);

					break;
				}
			}
		});

		Ok(this)
	}

	pub async fn reload_config(&self) -> Result<(), sqlx::Error> {
		let db = scripty_db::get_db();
		let mut guild_res = sqlx::query!(
			"SELECT be_verbose, language, auto_detect_lang, transcript_only_role, translate, \
			 kiai_enabled FROM guilds WHERE guild_id = $1",
			self.guild_id.get() as i64
		)
		.fetch_one(db)
		.await?;

		if let Some(lvl) = scripty_premium::get_guild(self.guild_id.get()).await {
			self.premium_level.store(lvl as u8, Ordering::Relaxed);
			self.auto_detect_lang
				.store(guild_res.auto_detect_lang, Ordering::Relaxed);
		} else {
			self.premium_level.store(0, Ordering::Relaxed);
			self.auto_detect_lang.store(false, Ordering::Relaxed);
		}

		self.verbose.store(guild_res.be_verbose, Ordering::Relaxed);
		self.translate.store(guild_res.translate, Ordering::Relaxed);
		self.kiai_enabled
			.store(guild_res.kiai_enabled, Ordering::Relaxed);

		std::mem::swap(&mut *self.language.write(), &mut guild_res.language);
		std::mem::swap(
			&mut *self.transcribe_only_role.write(),
			&mut guild_res
				.transcript_only_role
				.map(|x| RoleId::new(x as u64)),
		);

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
				self.seen_users.clone(),
				self.guild_id,
				*self.transcribe_only_role.read(),
			)),
			EventContext::VoiceTick(voice_data) => tokio::spawn(voice_tick(VoiceTickContext {
				voice_data:         voice_data.clone(),
				ssrc_state:         Arc::clone(&self.ssrc_state),
				guild_id:           self.guild_id,
				voice_channel_id:   self.voice_channel_id,
				language:           self.language.clone(),
				verbose:            self.verbose.clone(),
				ctx:                self.context.clone(),
				webhook:            Arc::clone(&self.webhook),
				channel_id:         self.channel_id,
				thread_id:          self.thread_id,
				transcript_results: self.transcript_results.clone(),
				automod_server_cfg: Arc::clone(&self.automod_server_cfg),
				auto_detect_lang:   Arc::clone(&self.auto_detect_lang),
				translate:          Arc::clone(&self.translate),
				kiai_enabled:       Arc::clone(&self.kiai_enabled),
				kiai_client:        Arc::clone(&self.kiai_client),
			})),
			EventContext::ClientDisconnect(client_disconnect_data) => {
				tokio::spawn(client_disconnect(
					*client_disconnect_data,
					Arc::clone(&self.ssrc_state),
					Arc::clone(&self.premium_level),
					self.context.clone(),
					Arc::clone(&self.webhook),
					self.thread_id,
					self.transcript_results.clone(),
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
				self.thread_id,
				self.transcript_results.clone(),
				self.seen_users.clone(),
				self.ephemeral,
			)),
			EventContext::RtpPacket(rtp_data) => {
				rtp_packet(rtp_data, self.guild_id);
				return None;
			}

			_ => return None,
		};
		None
	}
}

impl Clone for AudioHandler {
	fn clone(&self) -> Self {
		let verbose = self.verbose.clone();

		let remaining = Arc::<_>::strong_count(&self.verbose);
		trace!(
			"{} references to AudioHandler {{ guild_id: {} }} left",
			remaining, self.guild_id
		);

		Self {
			ssrc_state: self.ssrc_state.clone(),
			guild_id: self.guild_id,
			channel_id: self.channel_id,
			voice_channel_id: self.voice_channel_id,
			thread_id: self.thread_id,
			webhook: self.webhook.clone(),
			context: self.context.clone(),
			premium_level: self.premium_level.clone(),
			verbose,
			language: self.language.clone(),
			transcript_results: self.transcript_results.clone(),
			seen_users: self.seen_users.clone(),
			automod_server_cfg: self.automod_server_cfg.clone(),
			auto_detect_lang: self.auto_detect_lang.clone(),
			transcribe_only_role: self.transcribe_only_role.clone(),
			translate: self.translate.clone(),
			kiai_enabled: self.kiai_enabled.clone(),
			kiai_client: self.kiai_client.clone(),
			ephemeral: self.ephemeral,
			alive_call: self.alive_call.clone(),
		}
	}
}
