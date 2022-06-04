//! Metrics observatory for the Tokio runtime.

use crate::metrics::Metrics;
use tokio_metrics::RuntimeMetrics;

pub fn register_metrics(handle: tokio::runtime::Handle) {
    info!("injecting runtime metrics monitor");
    let monitor = tokio_metrics::RuntimeMonitor::new(&handle);
    info!("injected runtime metrics monitor, spawning thread");
    std::thread::spawn(move || {
        let m = crate::METRICS.get_or_init(Metrics::new).clone();
        for interval in monitor.intervals() {
            trace!("runtime metrics: {:?}", interval);
            let RuntimeMetrics {
                workers_count,
                total_park_count,
                max_park_count,
                min_park_count,
                total_noop_count,
                max_noop_count,
                min_noop_count,
                total_steal_count,
                max_steal_count,
                min_steal_count,
                num_remote_schedules,
                total_local_schedule_count,
                max_local_schedule_count,
                min_local_schedule_count,
                total_overflow_count,
                max_overflow_count,
                min_overflow_count,
                total_polls_count,
                max_polls_count,
                min_polls_count,
                total_busy_duration,
                max_busy_duration,
                min_busy_duration,
                injection_queue_depth,
                total_local_queue_depth,
                max_local_queue_depth,
                min_local_queue_depth,
                elapsed,
                ..
            } = interval;
            m.runtime_metrics.workers_count.set(workers_count as i64);
            m.runtime_metrics
                .total_park_count
                .set(total_park_count as i64);
            m.runtime_metrics.max_park_count.set(max_park_count as i64);
            m.runtime_metrics.min_park_count.set(min_park_count as i64);
            m.runtime_metrics
                .total_noop_count
                .set(total_noop_count as i64);
            m.runtime_metrics.max_noop_count.set(max_noop_count as i64);
            m.runtime_metrics.min_noop_count.set(min_noop_count as i64);
            m.runtime_metrics
                .total_steal_count
                .set(total_steal_count as i64);
            m.runtime_metrics
                .max_steal_count
                .set(max_steal_count as i64);
            m.runtime_metrics
                .min_steal_count
                .set(min_steal_count as i64);
            m.runtime_metrics
                .num_remote_schedules
                .set(num_remote_schedules as i64);
            m.runtime_metrics
                .total_local_schedule_count
                .set(total_local_schedule_count as i64);
            m.runtime_metrics
                .max_local_schedule_count
                .set(max_local_schedule_count as i64);
            m.runtime_metrics
                .min_local_schedule_count
                .set(min_local_schedule_count as i64);
            m.runtime_metrics
                .total_overflow_count
                .set(total_overflow_count as i64);
            m.runtime_metrics
                .max_overflow_count
                .set(max_overflow_count as i64);
            m.runtime_metrics
                .min_overflow_count
                .set(min_overflow_count as i64);
            m.runtime_metrics
                .total_polls_count
                .set(total_polls_count as i64);
            m.runtime_metrics
                .max_polls_count
                .set(max_polls_count as i64);
            m.runtime_metrics
                .min_polls_count
                .set(min_polls_count as i64);
            m.runtime_metrics
                .total_busy_duration
                .set(total_busy_duration.as_nanos() as i64);
            m.runtime_metrics
                .max_busy_duration
                .set(max_busy_duration.as_nanos() as i64);
            m.runtime_metrics
                .min_busy_duration
                .set(min_busy_duration.as_nanos() as i64);
            m.runtime_metrics
                .injection_queue_depth
                .set(injection_queue_depth as i64);
            m.runtime_metrics
                .total_local_queue_depth
                .set(total_local_queue_depth as i64);
            m.runtime_metrics
                .max_local_queue_depth
                .set(max_local_queue_depth as i64);
            m.runtime_metrics
                .min_local_queue_depth
                .set(min_local_queue_depth as i64);
            m.runtime_metrics.elapsed.set(elapsed.as_nanos() as i64);
        }
    });
}
