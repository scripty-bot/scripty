use serenity::client::{Context as SerenityContext, EventHandler};
use serenity::model::application::interaction::Interaction;
use serenity::model::channel::Message;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;

mod cache_ready;
mod interaction_create;
mod message;
mod ready;
mod resume;

pub struct BotEventHandler;

#[async_trait]
impl EventHandler for BotEventHandler {
    #[inline(always)]
    async fn cache_ready(&self, ctx: SerenityContext, guilds: Vec<GuildId>) {
        cache_ready::cache_ready(ctx, guilds).await;
    }

    #[inline(always)]
    async fn message(&self, ctx: SerenityContext, new_message: Message) {
        message::message(ctx, new_message).await;
    }

    #[inline(always)]
    async fn ready(&self, ctx: SerenityContext, ready: Ready) {
        ready::ready(ctx, ready).await;
    }

    #[inline(always)]
    async fn resume(&self, ctx: SerenityContext, resume: ResumedEvent) {
        resume::resume(ctx, resume).await;
    }

    #[inline(always)]
    async fn interaction_create(&self, ctx: SerenityContext, interaction: Interaction) {
        interaction_create::interaction_create(ctx, interaction).await;
    }
}
