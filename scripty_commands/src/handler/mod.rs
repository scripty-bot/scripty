use serenity::client::{Context as SerenityContext, EventHandler};
use serenity::model::gateway::Ready;

mod cache_ready;
mod message;
mod post_command;
mod ready;
mod resume;

pub struct BotEventHandler;

#[async_trait]
impl EventHandler for BotEventHandler {
    async fn ready(&self, _: SerenityContext, _: Ready) {
        println!("Bot is ready!");
    }
}
