#![feature(slice_as_array)]

mod embed_pagination;
mod hash_user_id;
pub mod latency;
mod separate_num;

pub use embed_pagination::do_paginate;
pub use hash_user_id::hash_user_id;
pub use separate_num::separate_num;
