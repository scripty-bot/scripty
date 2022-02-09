use ahash::RandomState;
use parking_lot::RwLock;
use scripty_audio::Stream;
use songbird::model::id::UserId;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

/// Type alias for a `DashMap` containing SSRCs mapped to `UserId`s.
pub type SsrcUserIdMap = Arc<RwLock<HashMap<u32, UserId, RandomState>>>;
/// Type alias for a `DashMap` containing SSRCs mapped to user data.
///
/// Field 0 of the internal tuple is the formatted username (name#0000)
///
/// Field 1 of the internal tuple is the user's avatar URL
pub type SsrcUserDataMap = Arc<RwLock<HashMap<u32, (String, String), RandomState>>>;
/// Type alias for a `DashMap` containing SSRCs mapped to `Stream`s
pub type SsrcStreamMap = Arc<RwLock<HashMap<u32, Stream, RandomState>>>;
/// Type alias for a `DashMap` containing SSRCs mapped to whether they should be ignored
pub type SsrcIgnoredMap = Arc<RwLock<HashMap<u32, bool, RandomState>>>;
/// Type alias for a `DashSet` containing the current list of active users
pub type ActiveUserSet = Arc<RwLock<HashSet<u32, RandomState>>>;
/// Type alias for a `RwLock<Vec>` containing the next users to be added
pub type NextUserList = Arc<RwLock<VecDeque<u32>>>;
