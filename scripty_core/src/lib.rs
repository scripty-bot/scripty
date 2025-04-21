#[macro_use]
extern crate tracing;

use std::{fs::OpenOptions, time::SystemTime};

use fenrir_rs::{NetworkingBackend, SerializationFormat};
use fern::{
	Dispatch,
	colors::{Color, ColoredLevelConfig},
};
use rlimit::Resource;
use url::Url;

pub fn start() {
	load_config();

	let rt = get_tokio_rt();

	rt.block_on(init_logging());

	increase_open_file_limit();

	scripty_i18n::init_i18n();

	rt.block_on(async_init());
	rt.spawn(scripty_webserver::entrypoint());
	rt.block_on(scripty_bot::entrypoint());
}

async fn init_logging() {
	let cfg = scripty_config::get_config();

	// configure colors for the whole line
	let colors_line = ColoredLevelConfig::new()
		.error(Color::Red)
		.warn(Color::Yellow)
		// we actually don't need to specify the color for debug and info, they are white by default
		.info(Color::White)
		.debug(Color::White)
		// depending on the terminals color scheme, this is the same as the background color
		.trace(Color::BrightBlack);

	// configure colors for the name of the level.
	// since almost all of them are the same as the color for the whole line, we
	// just clone `colors_line` and overwrite our changes
	let colors_level = colors_line.info(Color::Green);

	let mut builder = fenrir_rs::Fenrir::builder()
		.endpoint(Url::parse(&cfg.loki.url).expect("invalid loki url"))
		.network(NetworkingBackend::Reqwest)
		.format(SerializationFormat::Json)
		.max_message_size(cfg.loki.max_message_size)
		.flush_threshold(cfg.loki.flush_threshold.unwrap_or(1_000))
		.include_level();

	for field in cfg.loki.labels.iter() {
		builder = builder.tag(field.0, field.1);
	}
	let fenrir = builder.build();

	Dispatch::new()
		.chain(
			Dispatch::new()
				// configure this logger instance to make a fancy, colorful output
				.format(move |out, message, record| {
					out.finish(format_args!(
						"{color_line}[{date} {level} {target} {color_line}] {message}\x1B[0m",
						color_line = format_args!(
							"\x1B[{}m",
							colors_line.get_color(&record.level()).to_fg_str()
						),
						date = humantime::format_rfc3339(SystemTime::now()),
						target = record.target(),
						level = colors_level.color(record.level()),
						message = message,
					));
				})
				// just log messages with INFO or higher log level
				.level(tracing::log::LevelFilter::Info)
				// completely ignore ureq logs
				.level_for("ureq", tracing::log::LevelFilter::Off)
				// boost fenrir_rs logs to TRACE
				.level_for("fenrir_rs", tracing::log::LevelFilter::Trace)
				// quieten tracing spans
				.level_for("tracing::span", tracing::log::LevelFilter::Off)
				// print this setup of log messages to the console
				.chain(std::io::stdout()),
		)
		.chain(
			Dispatch::new()
				// output a raw message
				.format(|out, message, _record| out.finish(format_args!("{}", message)))
				// log everything
				.level(tracing::log::LevelFilter::Trace)
				// send this setup of log messages to Loki
				.chain(Box::new(fenrir) as Box<dyn tracing::log::Log>),
		)
		.chain(
			Dispatch::new()
				// configure this logger instance to make a slightly more informative output, but without colors
				.format(move |out, message, record| {
					out.finish(format_args!(
						"[{date} {level} {target}] {message}",
						date = humantime::format_rfc3339(SystemTime::now()),
						target = record.target(),
						level = record.level(),
						message = message,
					))
				})
				// log most things
				.level(tracing::log::LevelFilter::Debug)
				// again, ignore tracing spans
				.level_for("tracing::span", tracing::log::LevelFilter::Off)
				// quieten ureq logs
				.level_for("ureq", tracing::log::LevelFilter::Warn)
				// quieten hyper
				.level_for("hyper", tracing::log::LevelFilter::Warn)
				// quieten h2
				.level_for("h2", tracing::log::LevelFilter::Warn)
				// quieten rustls
				.level_for("rustls", tracing::log::LevelFilter::Warn)
				// send this setup of log messages to a file named `output.log`
				.chain(
					OpenOptions::new()
						.write(true)
						.create(true)
						.truncate(true)
						.open("output.log")
						.expect("failed to open output.log"),
				),
		)
		.apply()
		.expect("failed to init logger");
}

async fn async_init() {
	scripty_redis::init_redis().await;

	scripty_stt::init_stt().await;

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

fn increase_open_file_limit() {
	rlimit::setrlimit(Resource::NOFILE, 8192, 8192)
		.expect("failed to increase open file limit: will likely cause issues with STT service");
}
