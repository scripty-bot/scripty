use serenity::http::RatelimitInfo;

pub async fn ratelimit(
	RatelimitInfo {
		timeout,
		limit,
		method,
		path,
		global,
		..
	}: &RatelimitInfo,
) {
	let method = method.reqwest_method();
	warn!(
		"Ratelimited! Timeout: {:?}s, Limit: {}, Method: {}, Path: {}, Global: {}",
		timeout, limit, method, path, global
	);
}
