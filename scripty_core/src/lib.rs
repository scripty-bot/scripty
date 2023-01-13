#[macro_use]
extern crate tracing;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn start() {
    load_config();

    let rt = get_tokio_rt();

    rt.block_on(init_logging());

    scripty_i18n::init_i18n();

    rt.block_on(async_init());
    rt.spawn(scripty_webserver::entrypoint());
    rt.block_on(scripty_bot::entrypoint());
}

async fn init_logging() {
    let cfg = scripty_config::get_config();
    let (layer, task) = tracing_loki::layer(
        cfg.loki.url.parse().expect("invalid Loki URL"),
        cfg.loki.labels.clone(),
        cfg.loki.extra_fields.clone(),
    )
    .expect("failed to initialize loki ingestor");

    tracing_subscriber::registry()
        .with(layer)
        .with(tracing_subscriber::fmt::Layer::default())
        .init();

    // spawn the background task for loki ingest
    tokio::spawn(task);
}

async fn async_init() {
    scripty_redis::init_redis().await;

    scripty_audio::init_stt().await;

    scripty_db::init_db().await;

    scripty_data_storage::init_cache_async()
        .await
        .expect("failed to init cache");
}

fn get_tokio_rt() -> tokio::runtime::Runtime {
    let threads = num_cpus::get();
    info!("spawning tokio rt with {} threads", threads);

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(threads)
        .enable_all()
        .build()
        .expect("failed to build new tokio rt");

    // register runtime metrics
    scripty_metrics::register_metrics(rt.handle().clone());

    rt
}

fn load_config() {
    let cfg_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./config.toml".to_string());
    println!("reading cfg at {}", cfg_path);

    scripty_config::load_config(&cfg_path);
}
