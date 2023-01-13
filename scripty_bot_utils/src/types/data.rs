use crate::Error;
use serenity::all::ShardManager;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Data {
    pub shard_manager: Arc<Mutex<ShardManager>>,
}
