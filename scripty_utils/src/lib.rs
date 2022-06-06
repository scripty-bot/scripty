use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::Mutex;

#[macro_use]
extern crate tracing;

mod block_in_place;
mod hash_user_id;
pub mod latency;
mod separate_num;

pub use block_in_place::block_in_place;
pub use hash_user_id::hash_user_id;
pub use separate_num::separate_num;

pub struct ShardManagerWrapper;
impl TypeMapKey for ShardManagerWrapper {
    type Value = Arc<Mutex<ShardManager>>;
}
