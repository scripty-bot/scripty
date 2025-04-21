use serenity::gateway::client::Context;

#[inline]
pub async fn resume(_: &Context) {
	info!("successfully resumed");
}
