use once_cell::sync::OnceCell;

#[macro_use]
extern crate tracing;

mod cmd_handler;
mod get_metrics;
mod metrics;
mod rt_metrics;

pub use get_metrics::get_metrics;
use metrics::METRICS;
