//! Transaction helper for Redis.

use redis::{Cmd, FromRedisValue};

use crate::get_pool;

#[derive(Debug)]
pub enum TransactionError {
	Deadpool(deadpool_redis::PoolError),
	Redis(redis::RedisError),
}

impl std::fmt::Display for TransactionError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TransactionError::Deadpool(e) => write!(f, "deadpool error: {}", e),
			TransactionError::Redis(e) => write!(f, "redis error: {}", e),
		}
	}
}

impl std::error::Error for TransactionError {}

impl From<deadpool_redis::PoolError> for TransactionError {
	fn from(e: deadpool_redis::PoolError) -> Self {
		TransactionError::Deadpool(e)
	}
}

impl From<redis::RedisError> for TransactionError {
	fn from(e: redis::RedisError) -> Self {
		TransactionError::Redis(e)
	}
}

pub async fn run_transaction<T: FromRedisValue>(
	cmd_name: &str,
	cmd_fn: impl FnOnce(&mut Cmd),
) -> Result<T, TransactionError> {
	let mut conn = get_pool().get().await?;
	let mut cmd = redis::cmd(cmd_name);
	cmd_fn(&mut cmd);
	Ok(cmd.query_async(&mut conn).await?)
}
