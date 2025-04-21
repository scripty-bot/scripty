//! General wrapper around Redis.

#[macro_use]
extern crate tracing;

mod init;
mod transaction;

use deadpool_redis::Pool;
pub use deadpool_redis::PoolError;
pub use init::init_redis;
use once_cell::sync::OnceCell;
pub use redis;
pub use transaction::{TransactionError, run_transaction};

static REDIS_POOL: OnceCell<Pool> = OnceCell::new();

pub fn get_pool() -> &'static Pool {
	REDIS_POOL.get().expect("redis pool not initialized")
}
