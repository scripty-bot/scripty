use serenity::{client::Context, model::event::ResumedEvent};

#[inline]
pub async fn resume(_: &Context, evt: &ResumedEvent) {
	info!("successfully resumed");
}
