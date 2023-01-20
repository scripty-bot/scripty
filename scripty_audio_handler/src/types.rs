use ahash::RandomState;
use dashmap::{DashMap, DashSet};
use parking_lot::RwLock;
use scripty_audio::Stream;
use scripty_data_storage::VoiceIngest;
use std::collections::VecDeque;
use std::num::NonZeroU64;

/// Type alias for a `DashMap` containing SSRCs mapped to `UserId`s.
pub type SsrcUserIdMap = DashMap<u32, NonZeroU64, RandomState>;

/// Type alias for a `DashMap` containing SSRCs mapped to `Stream`s
pub type SsrcStreamMap = DashMap<u32, Stream, RandomState>;

/// Type alias for a `DashMap` containing SSRCs mapped to user data.
///
/// Field 0 of the internal tuple is the formatted username (name#0000)
///
/// Field 1 of the internal tuple is the user's avatar URL
pub type SsrcUserDataMap = DashMap<u32, (String, String), RandomState>;

/// Type alias for a `DashMap` containing SSRCs mapped to whether they should be ignored
pub type SsrcIgnoredMap = DashMap<u32, bool, RandomState>;

/// Type alias for a `DashMap` containing SSRCs mapped to a voice audio ingest struct.
pub type SsrcVoiceIngestMap = DashMap<u32, Option<VoiceIngest>, RandomState>;

/// Type alias for a `DashSet` containing the SSRCs that were speaking this tick.
pub type SsrcSpeakingSet = DashSet<u32, RandomState>;

/// Type alias for a `DashSet` containing the current list of active users
pub type ActiveUserSet = DashSet<u32, RandomState>;

/// Type alias for a `RwLock<Vec>` containing the next users to be added
pub type NextUserList = RwLock<VecDeque<u32>>;
