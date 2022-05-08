#[macro_use]
extern crate tracing;

use tracing_subscriber::EnvFilter;

pub fn start() {
    init_logging();

    load_config();

    scripty_i18n::init_i18n();

    scripty_audio::init_stt();

    scripty_data_storage::init_cache();

    let rt = get_tokio_rt();

    rt.block_on(async_init());
    rt.spawn(scripty_webserver::entrypoint());
    rt.block_on(scripty_commands::entrypoint());
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

async fn async_init() {
    scripty_db::init_db().await;

    scripty_data_storage::init_cache_async()
        .await
        .expect("failed to init cache");
}

fn get_tokio_rt() -> tokio::runtime::Runtime {
    let threads = num_cpus::get();
    info!("spawning tokio rt with {} threads", threads);

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(threads)
        .enable_all()
        .build()
        .expect("failed to build new tokio rt")
}

fn load_config() {
    let cfg_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./config.toml".to_string());
    println!("reading cfg at {}", cfg_path);

    scripty_config::load_config(&cfg_path);
}
