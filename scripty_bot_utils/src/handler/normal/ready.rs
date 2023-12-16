use serenity::{client::Context, model::prelude::Ready};

pub async fn ready(ctx: Context, ready: Ready) {
	let Ready {
		version,
		user,
		guilds,
		..
	} = ready;

	info!(
		"bot ready: logged in as {}, in {} guilds, using API version {}",
		user.tag(),
		guilds.len(),
		version
	);

	// wait for a few seconds for the cache to be ready
	tokio::time::sleep(std::time::Duration::from_secs(5)).await;
	crate::background_tasks::init_background_tasks(ctx);
}
