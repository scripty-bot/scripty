pub async fn entrypoint() {
	let cfg = scripty_config::get_config();
	let bind_addr = cfg.bind_address.parse().expect("invalid bind address");

	let router = crate::endpoints::router();

	axum::Server::bind(&bind_addr)
		.serve(router.into_make_service())
		.await
		.expect("failed to start server");
}
