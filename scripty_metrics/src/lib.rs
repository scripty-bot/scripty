use once_cell::sync::OnceCell;

#[macro_use]
extern crate tracing;

mod cmd_handler;
mod get_metrics;
mod metrics;
mod rt_metrics;

use metrics::METRICS;
