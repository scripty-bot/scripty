//! This crate shall stay as small as possible, and only contain the
//! core code to run the bot. All other code should be in other crates.

mod framework_opts;

use poise::FrameworkBuilder;
use scripty_audio_handler::SerenityInit;
use scripty_bot_utils::extern_utils::set_cache_http;
use scripty_bot_utils::globals::{CLIENT_CACHE, CLIENT_DATA};
use scripty_bot_utils::{handler, Data};

pub async fn entrypoint() {
    // fetch the config
    let cfg = scripty_config::get_config();

    // initialize blocked entity list
    scripty_bot_utils::entity_block::init_blocked()
        .await
        .expect("failed to init blocked entities");

    // initialize the framework
    let client = FrameworkBuilder::default()
        .token(&cfg.token)
        .client_settings(|b| {
            b.event_handler(handler::BotEventHandler)
                .raw_event_handler(handler::RawEventHandler)
                .register_songbird_from_config(scripty_audio_handler::get_songbird())
        })
        .user_data_setup(move |ctx, _, c| {
            Box::pin(async move {
                set_cache_http(ctx.http.clone(), ctx.cache.clone());

                CLIENT_DATA
                    .set(Data {
                        shard_manager: c.shard_manager().clone(),
                    })
                    .expect("user data setup called more than once: bug?");
                CLIENT_CACHE
                    .set(ctx.cache.clone())
                    .expect("user data setup called more than once: bug?");

                let sm = c.shard_manager().clone();
                tokio::spawn(async move {
                    tokio::signal::ctrl_c()
                        .await
                        .expect("failed to listen for ctrl+c");
                    sm.lock().await.shutdown_all().await;
                });

                Ok(Data {
                    shard_manager: c.shard_manager().clone(),
                })
            })
        })
        .options(framework_opts::get_framework_opts())
        .intents(framework_opts::get_gateway_intents());

    client.run_autosharded().await.expect("failed to run bot");
}
