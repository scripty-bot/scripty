use serenity::client::Context;
use serenity::model::event::ResumedEvent;

#[inline]
pub async fn resume(_: Context, _: ResumedEvent) {
    info!("successfully resumed");
}
