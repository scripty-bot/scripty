#![feature(box_syntax)]
#![feature(async_closure)]

#[macro_use]
extern crate tracing;

use poise::FrameworkBuilder;
use scripty_audio_handler::SerenityInit;
use serenity::async_trait;
use serenity::model::prelude::Ready;
use serenity::prelude::{Context as SerenityContext, EventHandler};
use std::sync::Arc;

mod checks;
mod cmds;
mod entity_block;
mod error;
mod framework_opts;
mod handler;
mod models;

type Error = error::Error;
type Data = ();
type Context<'a> = poise::Context<'a, Data, Error>;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: SerenityContext, ready: Ready) {
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
        .user_data_setup(move |_, _, _| Box::pin(async move { Ok(()) }))
        .options(crate::framework_opts::get_framework_opts())
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
