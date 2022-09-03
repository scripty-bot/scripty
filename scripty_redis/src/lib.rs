//! General wrapper around Redis.

mod init;
mod transaction;

use deadpool_redis::Pool;
use once_cell::sync::OnceCell;

pub use deadpool_redis::PoolError;
pub use init::init_redis;
pub use redis;
pub use transaction::{run_transaction, TransactionError};

static REDIS_POOL: OnceCell<Pool> = OnceCell::new();

pub fn get_pool() -> &'static Pool {
    REDIS_POOL.get().expect("redis pool not initialized")
}
