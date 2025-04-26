//! Helpers to measure latency of command processing.

use std::time::Instant;

use dashmap::DashMap;
use once_cell::sync::OnceCell;

/// Stores the start time of messages.
static LATENCY_START_TIME: OnceCell<DashMap<u64, Instant>> = OnceCell::new();

/// Call this function in the raw event handler for a message/interaction.
/// This will quickly measure the start time of the command processing.
pub fn measure_start_latency(time: Instant, id: u64) {
	debug!(?id, "measure_start_latency");
	LATENCY_START_TIME
		.get_or_init(DashMap::new)
		.insert(id, time);
}

/// Call this function in a pre-command hook.
/// This will measure the total latency of the command processing.
pub fn measure_end_latency(id: u64) {
	let et = Instant::now();
	debug!(?id, "measure_end_latency");
	let latency_map = LATENCY_START_TIME.get_or_init(DashMap::new);
	match latency_map.remove(&id) {
		Some((_, st)) => {
			debug!(?id, "found start time");
			let tt = i64::try_from(et.duration_since(st).as_nanos())
				.expect("duration should not exceed 292.27 years");
			let metrics = crate::get_metrics();
			// average the latency
			let current = metrics.latency.command_process.get();
			let new = if current == 0 { tt } else { (current + tt) / 2 };
			metrics.latency.command_process.set(new);
		}
		None => {
			trace!(?id, "no start time found");
		}
	}
}

/// Call this function roughly every two minutes.
/// This will clear all stored start times, to free up memory.
#[inline]
pub fn clear_latency_start_times() {
	LATENCY_START_TIME.get_or_init(DashMap::new).clear();
}
