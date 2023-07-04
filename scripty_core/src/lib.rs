#[macro_use]
extern crate tracing;

use fenrir_rs::{NetworkingBackend, SerializationFormat};
use fern::Dispatch;
use url::Url;

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

	let mut builder = fenrir_rs::Fenrir::builder()
		.endpoint(Url::parse(&cfg.loki.url).expect("invalid loki url"))
		.network(NetworkingBackend::Ureq)
		.format(SerializationFormat::Json)
		.include_level();

	for field in cfg.loki.extra_fields.iter() {
		builder = builder.tag(field.0, field.1);
	}
	let fenrir = builder.build();

	Dispatch::new()
		.format(|out, message, record| {
			out.finish(format_args!(
				"[{} {} {}] {}",
				humantime::format_rfc3339(std::time::SystemTime::now()),
				record.level(),
				record.target(),
				message
			))
		})
		// just log messages with TRACE or higher log level
		.level(tracing::log::LevelFilter::Debug)
		// do not log messages from the websocket library below TRACE
		.level_for("tokio_tungstenite", tracing::log::LevelFilter::Debug.into())
		.level_for("tungstenite", tracing::log::LevelFilter::Debug.into())
		// completely ignore ureq logs
		.level_for("ureq", tracing::log::LevelFilter::Off.into())
		// other spammy utilities
		.level_for("h2", tracing::log::LevelFilter::Debug.into())
		// print the log messages to the console ...
		.chain(std::io::stdout())
		// ... and to the corresponding loki endpoint
		.chain(Box::new(fenrir) as Box<dyn tracing::log::Log>)
		.apply()
		.expect("failed to init logger");
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
