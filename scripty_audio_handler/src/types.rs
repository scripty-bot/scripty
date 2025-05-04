use std::{
	collections::VecDeque,
	sync::{Arc, RwLock},
};

use ahash::RandomState;
use dashmap::{DashMap, DashSet};
use scripty_data_storage::VoiceIngest;
use scripty_stt::Stream;

/// Type alias for a `DashMap` containing SSRCs mapped to `UserId`s.
pub type SsrcUserIdMap = DashMap<u32, u64, RandomState>;

/// Type alias for a `DashMap` containing SSRCs mapped to `Stream`s
pub type SsrcStreamMap = DashMap<u32, Stream, RandomState>;

/// Type alias for a `DashMap` containing SSRCs mapped to user data.
///
/// Field 0 of the internal tuple is the formatted username (name#0000)
///
/// Field 1 of the internal tuple is the user's avatar URL
///
/// Field 2 of the internal tuple is whether the user has the transcribe-only role
pub type SsrcUserDataMap = DashMap<u32, (String, String, bool), RandomState>;

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

/// Type alias for a `Arc<RwLock<Vec<String>>>` containing the transcript results
pub type TranscriptResults = Option<Arc<RwLock<Vec<String>>>>;

/// Type alias for a `Arc<DashSet<u64>>` containing the users that have been seen and who should
/// get a transcript at the end of the session.
pub type SeenUsers = Option<Arc<DashSet<u64, RandomState>>>;
