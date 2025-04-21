use std::sync::Arc;

use once_cell::sync::OnceCell;
use serenity::cache::Cache;

use crate::{Data, dm_support::DmSupportStatus};

pub static CLIENT_CACHE: OnceCell<Arc<Cache>> = OnceCell::new();
pub static CLIENT_DATA: OnceCell<Arc<Data>> = OnceCell::new();
pub static DM_SUPPORT_GLOBAL: OnceCell<DmSupportStatus> = OnceCell::new();
