use crate::dm_support::DmSupportStatus;
use crate::Data;
use once_cell::sync::OnceCell;
use serenity::client::Cache;
use std::sync::Arc;

pub static CLIENT_CACHE: OnceCell<Arc<Cache>> = OnceCell::new();
pub static CLIENT_DATA: OnceCell<Data> = OnceCell::new();
pub static DM_SUPPORT_GLOBAL: OnceCell<DmSupportStatus> = OnceCell::new();
