use serenity::client::{Context, RawEventHandler as SerenityRawEventHandler};
use serenity::model::event::Event;
use std::time::Instant;

pub struct RawEventHandler;

#[async_trait]
impl SerenityRawEventHandler for RawEventHandler {
    async fn raw_event(&self, _ctx: Context, event: Event) {
        // we need to handle command latency measurements here too,
        // as poise overwrites serenity and calls it after command processing
        let st = Instant::now();
        match &event {
            Event::MessageCreate(e) => scripty_metrics::measure_start_latency(st, e.message.id.0),
            Event::InteractionCreate(e) => {
                scripty_metrics::measure_start_latency(st, e.interaction.id().0)
            }
            _ => {}
        }

        let metrics = scripty_metrics::get_metrics();

        // increment the total number of events
        metrics.total_events.inc();

        // increment the total number of events per type
        match event {
            Event::ApplicationCommandPermissionsUpdate(_) => {
                metrics.events.application_command_permissions_update.inc()
            }
            Event::ChannelCreate(_) => metrics.events.channel_create.inc(),
            Event::ChannelDelete(_) => metrics.events.channel_delete.inc(),
            Event::ChannelPinsUpdate(_) => metrics.events.channel_pins_update.inc(),
            Event::ChannelUpdate(_) => metrics.events.channel_update.inc(),
            Event::GuildBanAdd(_) => metrics.events.guild_ban_add.inc(),
            Event::GuildBanRemove(_) => metrics.events.guild_ban_remove.inc(),
            Event::GuildCreate(_) => metrics.events.guild_create.inc(),
            Event::GuildDelete(_) => metrics.events.guild_delete.inc(),
            Event::GuildEmojisUpdate(_) => metrics.events.guild_emojis_update.inc(),
            Event::GuildIntegrationsUpdate(_) => metrics.events.guild_integrations_update.inc(),
            Event::GuildMemberAdd(_) => metrics.events.guild_member_add.inc(),
            Event::GuildMemberRemove(_) => metrics.events.guild_member_remove.inc(),
            Event::GuildMemberUpdate(_) => metrics.events.guild_member_update.inc(),
            Event::GuildMembersChunk(_) => metrics.events.guild_members_chunk.inc(),
            Event::GuildRoleCreate(_) => metrics.events.guild_role_create.inc(),
            Event::GuildRoleDelete(_) => metrics.events.guild_role_delete.inc(),
            Event::GuildRoleUpdate(_) => metrics.events.guild_role_update.inc(),
            Event::GuildStickersUpdate(_) => metrics.events.guild_stickers_update.inc(),
            Event::GuildUnavailable(_) => metrics.events.guild_unavailable.inc(),
            Event::GuildUpdate(_) => metrics.events.guild_update.inc(),
            Event::InviteCreate(_) => metrics.events.invite_create.inc(),
            Event::InviteDelete(_) => metrics.events.invite_delete.inc(),
            Event::MessageCreate(_) => metrics.events.message_create.inc(),
            Event::MessageDelete(_) => metrics.events.message_delete.inc(),
            Event::MessageDeleteBulk(_) => metrics.events.message_delete_bulk.inc(),
            Event::MessageUpdate(_) => metrics.events.message_update.inc(),
            Event::PresenceUpdate(_) => metrics.events.presence_update.inc(),
            Event::PresencesReplace(_) => metrics.events.presences_replace.inc(),
            Event::ReactionAdd(_) => metrics.events.reaction_add.inc(),
            Event::ReactionRemove(_) => metrics.events.reaction_remove.inc(),
            Event::ReactionRemoveAll(_) => metrics.events.reaction_remove_all.inc(),
            Event::Ready(_) => metrics.events.ready.inc(),
            Event::Resumed(_) => metrics.events.resumed.inc(),
            Event::TypingStart(_) => metrics.events.typing_start.inc(),
            Event::UserUpdate(_) => metrics.events.user_update.inc(),
            Event::VoiceStateUpdate(_) => metrics.events.voice_state_update.inc(),
            Event::VoiceServerUpdate(_) => metrics.events.voice_server_update.inc(),
            Event::WebhookUpdate(_) => metrics.events.webhook_update.inc(),
            _ => metrics.events.unknown.inc(),
        }
    }
}
