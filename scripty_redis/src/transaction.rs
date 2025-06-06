//! Transaction helper for Redis.

use redis::{Cmd, FromRedisValue};
use scripty_error::Error;

use crate::get_pool;

pub async fn run_transaction<T: FromRedisValue>(
	cmd_name: &str,
	cmd_fn: impl FnOnce(&mut Cmd),
) -> Result<T, Error> {
	let mut conn = get_pool().get().await?;
	let mut cmd = redis::cmd(cmd_name);
	cmd_fn(&mut cmd);
	Ok(cmd.query_async(&mut conn).await?)
}
