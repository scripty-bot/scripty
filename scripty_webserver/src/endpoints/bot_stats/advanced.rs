//! GET `/bot_stats/advanced`
//!
//! Return bot statistics, with more details, specifically:
//! * Per-shard latency
//! * Per-shard uptime
//! * Per-shard connection status

use crate::errors::WebServerError;
use axum::routing::get;
use axum::Json;
use scripty_commands::{get_channel_count, get_guild_count, get_shard_count, get_user_count};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvancedBotStats {
    pub guild_count: usize,
    pub user_count: usize,
    pub channel_count: usize,
    pub shard_count: u64,
    /// Per shard information.
    /// The key is the shard ID.
    pub shard_info: HashMap<u64, AdvancedBotStatsShard>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvancedBotStatsShard {
    pub latency: Option<u128>,
    pub connection_status: u8,
    pub guild_count: usize,
}

pub async fn get_advanced_bot_stats() -> Result<Json<AdvancedBotStats>, WebServerError> {
    let shard_info = scripty_commands::get_shard_info().await?;

    let mut processed_shard_info = HashMap::new();
    for (shard_id, shard_info) in shard_info {
        processed_shard_info.insert(
            shard_id,
            AdvancedBotStatsShard {
                latency: shard_info.latency,
                connection_status: shard_info.connection_status,
                guild_count: shard_info.guild_count,
            },
        );
    }

    Ok(Json(AdvancedBotStats {
        guild_count: get_guild_count()?,
        user_count: get_user_count()?,
        channel_count: get_channel_count()?,
        shard_count: get_shard_count()?,
        shard_info: processed_shard_info,
    }))
}

pub fn router() -> axum::Router {
    axum::Router::new().route("/bot_stats/advanced", get(get_advanced_bot_stats))
}
