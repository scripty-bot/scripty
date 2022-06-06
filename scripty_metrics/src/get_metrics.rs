use prometheus::{Encoder, TextEncoder};

pub fn get_formatted_metrics() -> Vec<u8> {
    let m = crate::METRICS
        .get()
        .expect("metrics not initialized")
        .clone();
    let encoder = TextEncoder::new();

    let mut buffer = vec![];
    let metric_families = m.registry.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    buffer
}
