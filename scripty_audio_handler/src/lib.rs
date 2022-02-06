#[macro_use]
extern crate tracing;

mod audio_handler;
mod connect;
mod error;
mod events;
mod types;

pub use audio_handler::AudioHandler;
pub use connect::connect_to_vc;
pub use error::Error;
use serenity::client::ClientBuilder;
use songbird::driver::{CryptoMode, DecodeMode};
use songbird::{register_from_config, Config};

pub fn register_songbird(client_builder: ClientBuilder) -> ClientBuilder {
    let config = Config::default()
        .crypto_mode(CryptoMode::Normal)
        .decode_mode(DecodeMode::Decode);
    register_from_config(client_builder, config)
}
