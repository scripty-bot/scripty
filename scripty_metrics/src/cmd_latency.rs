//! Helpers to measure latency of command processing.

use dashmap::DashMap;
use once_cell::sync::OnceCell;
use std::time::Instant;

/// Stores the start time of messages.
static LATENCY_START_TIME: OnceCell<DashMap<u64, Instant>> = OnceCell::new();

/// Call this function in on_message. This will quickly measure the start time of the command processing.
pub fn measure_start_latency(id: u64) {
    LATENCY_START_TIME
        .get_or_init(|| DashMap::new())
        .insert(id, Instant::now());
}

/// Call this function in pre_command. This will measure the total latency of the command processing.
pub fn measure_end_latency(id: u64) {
    let et = Instant::now();
    if let Some((_, st)) = LATENCY_START_TIME
        .get_or_init(|| DashMap::new())
        .remove(&id)
    {
        let tt = et.duration_since(st).as_nanos() as i64;
        let metrics = crate::get_metrics();
        // average the latency
        let current = metrics.latency.command_process.get();
        let new = (current + tt) / 2;
        metrics.latency.command_process.set(new);
    }
}

/// Call this function roughly every two minutes. This will clear all stored start times, to free up memory.
#[inline]
pub fn clear_latency_start_times() {
    LATENCY_START_TIME.get_or_init(|| DashMap::new()).clear();
}
