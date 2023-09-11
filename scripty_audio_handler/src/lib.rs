#[macro_use]
extern crate tracing;

mod audio_handler;
mod connect;
mod consts;
mod disconnect;
mod error;
mod events;
mod types;

pub use audio_handler::AudioHandler;
pub use connect::connect_to_vc;
pub use disconnect::disconnect_from_vc;
pub use error::Error;
pub use scripty_audio::{check_model_language, get_model_languages};
use serenity::{
	all::{ChannelId, GuildId},
	client::Context,
};
use songbird::{driver::DecodeMode, Config};
pub use songbird::{error::JoinError, serenity::SerenityInit};

pub fn get_songbird() -> Config {
	Config::default().decode_mode(DecodeMode::Decode)
}

pub async fn get_voice_channel_id(ctx: &Context, guild_id: GuildId) -> Option<ChannelId> {
	let Some(call) = songbird::get(ctx)
		.await
		.expect("failed to get songbird object")
		.get(guild_id)
	else {
		return None;
	};

	// this allows the compiler to be happy with the lifetime of the call i guess?
	let current_channel = call.lock().await.current_channel();
	current_channel.map(|c| ChannelId(c.0))
}
