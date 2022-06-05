#![feature(box_syntax)]
#![feature(async_closure)]

#[macro_use]
extern crate tracing;
#[macro_use]
extern crate scripty_i18n;

use once_cell::sync::OnceCell;
use poise::FrameworkBuilder;
use scripty_audio_handler::SerenityInit;
use scripty_utils::ShardManagerWrapper;
use serenity::async_trait;
use serenity::model::prelude::Ready;
use serenity::prelude::{Context as SerenityContext, EventHandler, GatewayIntents, TypeMapKey};
use std::sync::Arc;

mod checks;
mod cmds;
mod entity_block;
mod error;
mod extern_utils;
mod framework_opts;
mod handler;
mod models;

pub use extern_utils::*;

pub(crate) type Error = error::Error;
pub struct Data {
    shard_manager: <ShardManagerWrapper as TypeMapKey>::Value,
}
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;

pub(crate) static CLIENT_CACHE: OnceCell<Arc<serenity::client::Cache>> = OnceCell::new();

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: SerenityContext, ready: Ready) {
        // initialize cache
        // ignore any errors, as ready can be called multiple times
        let _ = CLIENT_CACHE.set(ctx.cache);

        println!("{} is connected!", ready.user.name);
    }
}

pub async fn entrypoint() {
    let cfg = scripty_config::get_config();

    crate::entity_block::init_blocked()
        .await
        .expect("failed to init blocked entities");

    let client = FrameworkBuilder::default()
        .token(&cfg.token)
        .client_settings(|b| {
            b.event_handler(Handler)
                .register_songbird_from_config(scripty_audio_handler::get_songbird())
        })
        .user_data_setup(move |_, _, c| {
            Box::pin(async move {
                Ok(Data {
                    shard_manager: c.shard_manager().clone(),
                })
            })
        })
        .options(crate::framework_opts::get_framework_opts())
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .build()
        .await
        .expect("failed to build framework");

    let c2 = Arc::clone(&client);
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for ctrl+c");
        c2.shard_manager().lock().await.shutdown_all().await;
    });

    client.start_autosharded().await.expect("failed to run bot");
}
