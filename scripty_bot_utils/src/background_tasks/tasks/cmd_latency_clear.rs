use std::time::Duration;

use serenity::client::Context;

use crate::{background_tasks::core::BackgroundTask, Error};

/// Clears stale latency metrics every 2 minutes to free up memory.
pub struct CommandLatencyClearer;

impl BackgroundTask for CommandLatencyClearer {
	async fn init(_: Context) -> Result<Self, Error> {
		Ok(Self)
	}

	fn interval(&mut self) -> Duration {
		Duration::from_secs(120)
	}

	async fn run(&mut self) {
		scripty_metrics::clear_latency_start_times();
	}
}
