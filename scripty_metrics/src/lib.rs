#[macro_use]
extern crate tracing;

mod cmd_handler;
mod get_metrics;
mod metrics;
mod rt_metrics;

pub use get_metrics::get_formatted_metrics;
use metrics::METRICS;
pub use metrics::{get_metrics, Metrics};
pub use rt_metrics::register_metrics;
