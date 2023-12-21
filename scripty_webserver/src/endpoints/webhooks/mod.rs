use axum::routing::post;

mod discordservices_net;
mod top_gg;
mod wumpus_store;

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/webhooks/top_gg", post(top_gg::top_gg_incoming_webhook))
		.route(
			"/webhooks/wumpus_store",
			post(wumpus_store::wumpus_store_incoming_webhook),
		)
		.route(
			"/webhooks/discordservices_net",
			post(discordservices_net::discordservices_net_incoming_webhook),
		)
}
