use std::sync::Arc;

use serenity::{gateway::ShardManager, prelude::TypeMapKey};

mod embed_pagination;
mod hash_user_id;
mod hex_vec;
mod http;
pub mod latency;
mod separate_num;

pub use embed_pagination::do_paginate;
pub use hash_user_id::hash_user_id;
pub use hex_vec::vec_to_hex;
pub use http::{get_thirdparty_http, init_thirdparty_http};
pub use separate_num::separate_num;

pub struct ShardManagerWrapper;
impl TypeMapKey for ShardManagerWrapper {
	type Value = Arc<ShardManager>;
}
