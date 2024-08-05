use serenity::{client::Context, model::event::ResumedEvent};

#[inline]
pub async fn resume(_: Context, _evt: ResumedEvent) {
	info!("successfully resumed");
}
