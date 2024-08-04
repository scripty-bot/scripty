use scripty_config::DatabaseConnection;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

pub async fn init_db() {
	let cfg = scripty_config::get_config();

	let mut conn_opts = PgConnectOptions::new()
		.database(&cfg.database.database)
		.password(&cfg.database.password)
		.username(&cfg.database.user)
		.application_name("scripty");

	conn_opts = match &cfg.database.host {
		DatabaseConnection::Tcp(host, port) => conn_opts.host(host).port(*port),
		DatabaseConnection::Unix(path) => conn_opts.socket(path),
	};

	info!("trying to connect to postgres");
	let pool = PgPoolOptions::new()
		.min_connections(2)
		.max_connections(32)
		.connect_with(conn_opts)
		.await
		.expect("failed to connect to db");

	info!("running pending migrations");
	sqlx::migrate!("../migrations")
		.run(&pool)
		.await
		.expect("failed to run migrations");

	crate::store::set_db(pool);
}
