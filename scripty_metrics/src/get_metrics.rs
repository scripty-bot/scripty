use prometheus::TextEncoder;

pub fn get_formatted_metrics() -> String {
	TextEncoder::new()
		.encode_to_string(
			&crate::METRICS
				.get()
				.expect("metrics not initialized")
				.registry
				.gather(),
		)
		.expect("writing to a string should be infallible")
}
