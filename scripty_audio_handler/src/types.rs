use ahash::RandomState;
use dashmap::{DashMap, DashSet};
use parking_lot::RwLock;
use scripty_audio::Stream;
use songbird::model::id::UserId;
use std::sync::Arc;

pub type SsrcUserIdMap = Arc<DashMap<u32, UserId, RandomState>>;
pub type SsrcIgnoredMap = Arc<DashMap<u32, bool, RandomState>>;
pub type SsrcStreamMap = Arc<DashMap<u32, Stream, RandomState>>;
pub type ActiveUserSet = Arc<DashSet<u32, RandomState>>;
pub type NextUserList = Arc<RwLock<Vec<u32>>>;
