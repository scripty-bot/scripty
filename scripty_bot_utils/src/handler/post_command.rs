use poise::BoxFuture;

async fn _post_command(_: crate::Context<'_>) {
	// now unused
}

pub fn post_command(ctx: crate::Context<'_>) -> BoxFuture<()> {
	Box::pin(_post_command(ctx))
}
