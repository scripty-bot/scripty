#![feature(box_syntax)]
#![feature(async_closure)]
#![feature(backtrace)]

#[macro_use]
extern crate tracing;
#[macro_use]
extern crate scripty_i18n;
#[macro_use]
extern crate async_trait;

use crate::dm_support::DmSupportStatus;
use once_cell::sync::OnceCell;
use poise::FrameworkBuilder;
use scripty_audio_handler::SerenityInit;
use scripty_utils::ShardManagerWrapper;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;

mod background_tasks;
mod checks;
mod cmds;
mod dm_support;
mod entity_block;
mod error;
mod extern_utils;
mod framework_opts;
mod handler;
mod models;

pub use extern_utils::*;

pub(crate) type Error = error::Error;
#[derive(Debug)]
pub struct Data {
    shard_manager: <ShardManagerWrapper as TypeMapKey>::Value,
}
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;

pub(crate) static CLIENT_CACHE: OnceCell<Arc<serenity::client::Cache>> = OnceCell::new();
pub(crate) static CLIENT_DATA: OnceCell<Data> = OnceCell::new();
pub(crate) static DM_SUPPORT_GLOBAL: OnceCell<DmSupportStatus> = OnceCell::new();

pub async fn entrypoint() {
    let cfg = scripty_config::get_config();

    entity_block::init_blocked()
        .await
        .expect("failed to init blocked entities");

    let client = FrameworkBuilder::default()
        .token(&cfg.token)
        .client_settings(|b| {
            b.event_handler(handler::BotEventHandler)
                .raw_event_handler(handler::RawEventHandler)
                .register_songbird_from_config(scripty_audio_handler::get_songbird())
        })
        .user_data_setup(move |ctx, _, c| {
            Box::pin(async move {
                CLIENT_DATA
                    .set(Data {
                        shard_manager: c.shard_manager().clone(),
                    })
                    .expect("user data setup called more than once: bug?");
                CLIENT_CACHE
                    .set(ctx.cache.clone())
                    .expect("user data setup called more than once: bug?");

                Ok(Data {
                    shard_manager: c.shard_manager().clone(),
                })
            })
        })
        .options(framework_opts::get_framework_opts())
        .intents(framework_opts::get_gateway_intents())
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
