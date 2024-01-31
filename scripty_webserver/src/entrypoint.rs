use std::net::SocketAddr;

use tokio::net::TcpListener;

pub async fn entrypoint() {
	let cfg = scripty_config::get_config();
	let bind_addr: SocketAddr = cfg.bind_address.parse().expect("invalid bind address");

	let router = crate::endpoints::router();

	let tcp = TcpListener::bind(bind_addr).await.unwrap();

	axum::serve(tcp, router)
		.await
		.expect("failed to start server");
}
