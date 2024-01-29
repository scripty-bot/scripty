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
use serenity::{
	all::{ChannelId, GuildId},
	client::Context,
};
use songbird::{driver::DecodeMode, Config, Songbird};
pub use songbird::{error::JoinError, serenity::SerenityInit};
use tokio::sync::oneshot::Sender;

pub fn get_songbird() -> Config {
	Config::default().decode_mode(DecodeMode::Decode)
}

pub async fn get_voice_channel_id(ctx: &Context, guild_id: GuildId) -> Option<ChannelId> {
	let call = songbird::serenity::get(ctx)
		.await
		.expect("failed to get songbird object")
		.get(guild_id)?;

	// this allows the compiler to be happy with the lifetime of the call i guess?
	let current_channel = call.lock().await.current_channel();
	current_channel.map(|c| ChannelId::new(c.0.get()))
}

pub async fn get_songbird_from_ctx(ctx: &Context) -> Arc<Songbird> {
	get(ctx).await.expect("songbird not registered")
}

static AUTO_LEAVE_TASKS: OnceCell<DashMap<GuildId, Sender<()>, ahash::RandomState>> =
	OnceCell::new();
