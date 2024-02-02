#[macro_use]
extern crate tracing;

mod audio_handler;
mod connect;
mod consts;
mod disconnect;
mod error;
mod events;
mod types;

use std::sync::{Arc, OnceLock as OnceCell};

pub use audio_handler::AudioHandler;
pub use connect::connect_to_vc;
use dashmap::DashMap;
pub use disconnect::disconnect_from_vc;
pub use error::{Error, ErrorKind};
pub use scripty_stt::{check_model_language, get_model_languages};
use serenity::all::{ChannelId, GuildId};
use songbird::{driver::DecodeMode, Config};
pub use songbird::{error::JoinError, Songbird};
use tokio::sync::oneshot::Sender;

pub fn get_songbird_config() -> Config {
	Config::default().decode_mode(DecodeMode::Decode)
}

static SONGBIRD: OnceCell<Arc<Songbird>> = OnceCell::new();

pub async fn get_voice_channel_id(guild_id: GuildId) -> Option<ChannelId> {
	let call = get_songbird().get(guild_id)?;

	// this allows the compiler to be happy with the lifetime of the call i guess?
	let current_channel = call.lock().await.current_channel();
	current_channel.map(|c| ChannelId::new(c.get()))
}

pub fn set_songbird(sb: Arc<Songbird>) {
	SONGBIRD
		.set(sb)
		.expect("should not call set_songbird more than once");
}

pub fn get_songbird() -> Arc<Songbird> {
	SONGBIRD.get().expect("songbird not registered").clone()
}

static AUTO_LEAVE_TASKS: OnceCell<DashMap<GuildId, Sender<()>, ahash::RandomState>> =
	OnceCell::new();
