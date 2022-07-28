use ahash::RandomState;
use dashmap::{DashMap, DashSet};
use parking_lot::RwLock;
use scripty_audio::Stream;
use scripty_data_storage::VoiceIngest;
use std::collections::VecDeque;
use std::num::NonZeroU64;
use std::sync::Arc;

/// Type alias for a `DashMap` containing SSRCs mapped to `UserId`s.
pub type SsrcUserIdMap = Arc<DashMap<u32, NonZeroU64, RandomState>>;

/// Type alias for a `DashMap` containing SSRCs mapped to `Stream`s
pub type SsrcStreamMap = Arc<DashMap<u32, Stream, RandomState>>;

/// Type alias for a `DashMap` containing SSRCs mapped to user data.
///
/// Field 0 of the internal tuple is the formatted username (name#0000)
///
/// Field 1 of the internal tuple is the user's avatar URL
pub type SsrcUserDataMap = Arc<DashMap<u32, (String, String), RandomState>>;

/// Type alias for a `DashMap` containing SSRCs and out-of-order packet IDs
/// mapped to their audio packet data.
pub type SsrcMissedPktMap = Arc<DashMap<(u32, u16), Vec<i16>, RandomState>>;

/// Type alias for a `DashMap` containing SSRCs mapped to a `Vec` of
/// audio packet IDs that were received out-of-order.
pub type SsrcMissedPktList = Arc<DashMap<u32, Vec<u16>, RandomState>>;

/// Type alias for a `DashMap` containing SSRCs mapped to whether they should be ignored
pub type SsrcIgnoredMap = Arc<DashMap<u32, bool, RandomState>>;

/// Type alias for a `DashMap` containing SSRCs mapped to the sequence ID of the last packet received
pub type SsrcLastPktIdMap = Arc<DashMap<u32, u16, RandomState>>;

/// Type alias for a `DashMap` containing SSRCs mapped to a voice audio ingest struct.
pub type SsrcVoiceIngestMap = Arc<DashMap<u32, Option<VoiceIngest>, RandomState>>;

/// Type alias for a `DashSet` containing the current list of active users
pub type ActiveUserSet = Arc<DashSet<u32, RandomState>>;

/// Type alias for a `RwLock<Vec>` containing the next users to be added
pub type NextUserList = Arc<RwLock<VecDeque<u32>>>;
