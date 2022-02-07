#![feature(box_syntax)]
#![feature(async_closure)]

#[macro_use]
extern crate tracing;

use poise::FrameworkBuilder;

mod cmds;
mod entity_block;
mod error;
mod framework_opts;

type Error = error::Error;
type Data = ();

pub async fn entrypoint() {
    let cfg = scripty_config::get_config();

    let builder = FrameworkBuilder::default()
        .token(&cfg.token)
        .client_settings(|b| {
            scripty_audio_handler::register_songbird(b).cache_settings(|s| s.max_messages(0))
        })
        .user_data_setup(move |_, _, _| Box::pin(async move { Ok(()) }))
        .options(crate::framework_opts::get_framework_opts());
}
