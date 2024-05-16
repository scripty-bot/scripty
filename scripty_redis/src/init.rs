use std::time::Duration;

use deadpool::managed::{PoolConfig, QueueMode, Timeouts};
use deadpool_redis::{redis::cmd, Config, Runtime};

pub async fn init_redis() {
	info!("configuring redis pool");

	// set up pool config
	let mut config = Config::from_url(&scripty_config::get_config().redis_url);
	let mut timeouts = Timeouts::new();
	timeouts.create = Some(Duration::from_secs(5));
	timeouts.recycle = Some(Duration::from_secs(2));
	timeouts.wait = Some(Duration::from_secs(5));
	config.pool = Some(PoolConfig {
		max_size: 128,
		timeouts,
		queue_mode: QueueMode::Fifo,
	});

	// initialize the pool
	info!("connecting to redis server");
	let pool = config
		.create_pool(Some(Runtime::Tokio1))
		.expect("failed to init redis");

	// test the pool by setting a key and getting it, then deleting it
	info!("testing connection");
	let mut conn = pool.get().await.unwrap();
	let _: () = cmd("SET")
		.arg("test")
		.arg("test")
		.query_async(&mut conn)
		.await
		.unwrap();
	let test: String = cmd("GET").arg("test").query_async(&mut conn).await.unwrap();
	assert_eq!(test, "test");
	let _: () = cmd("DEL").arg("test").query_async(&mut conn).await.unwrap();

	// set the pool as the global pool
	crate::REDIS_POOL
		.set(pool)
		.unwrap_or_else(|_| panic!("failed to set redis pool"));
}
