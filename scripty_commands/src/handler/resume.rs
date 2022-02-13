use serenity::model::event::ResumedEvent;

pub async fn resume(_: ResumedEvent) {
    info!("successfully resumed");
}
