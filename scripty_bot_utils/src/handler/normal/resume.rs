use serenity::{client::Context, model::event::ResumedEvent};

#[inline]
pub async fn resume(_: Context, _: ResumedEvent) {
	info!("successfully resumed");
}
