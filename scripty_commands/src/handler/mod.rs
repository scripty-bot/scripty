mod cache_ready;
mod message;
mod post_command;
mod ready;
mod resume;

use crate::{Data, Error};
use poise::Event;
pub use post_command::post_command;

pub async fn event_listener(
    _ctx: &serenity::client::Context,
    event: &poise::Event<'_>,
    _framework: &poise::Framework<Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    let event = event.clone();
    match event {
        Event::CacheReady { guilds } => cache_ready::cache_ready(guilds).await,
        Event::ChannelCreate { channel } => {}
        Event::CategoryCreate { category } => {}
        Event::CategoryDelete { category } => {}
        Event::ChannelDelete { channel } => {}
        Event::ChannelPinsUpdate { pin } => {}
        Event::ChannelUpdate { old, new } => {}
        Event::GuildBanAddition {
            guild_id,
            banned_user,
        } => {}
        Event::GuildBanRemoval {
            guild_id,
            unbanned_user,
        } => {}
        Event::GuildCreate { guild, is_new } => {}
        Event::GuildDelete { incomplete, full } => {}
        Event::GuildEmojisUpdate {
            guild_id,
            current_state,
        } => {}
        Event::GuildIntegrationsUpdate { guild_id } => {}
        Event::GuildMemberAddition { new_member } => {}
        Event::GuildMemberRemoval {
            guild_id,
            user,
            member_data_if_available,
        } => {}
        Event::GuildMemberUpdate {
            old_if_available,
            new,
        } => {}
        Event::GuildMembersChunk { chunk } => {}
        Event::GuildRoleCreate { new } => {}
        Event::GuildRoleDelete {
            guild_id,
            removed_role_id,
            removed_role_data_if_available,
        } => {}
        Event::GuildRoleUpdate {
            old_data_if_available,
            new,
        } => {}
        Event::GuildUnavailable { guild_id } => {}
        Event::GuildUpdate {
            old_data_if_available,
            new_but_incomplete,
        } => {}
        Event::InviteCreate { data } => {}
        Event::InviteDelete { data } => {}
        Event::Message { new_message } => message::message(new_message).await,
        Event::MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id,
        } => {}
        Event::MessageDeleteBulk {
            channel_id,
            multiple_deleted_messages_ids,
            guild_id,
        } => {}
        Event::MessageUpdate {
            old_if_available,
            new,
            event,
        } => {}
        Event::ReactionAdd { add_reaction } => {}
        Event::ReactionRemove { removed_reaction } => {}
        Event::ReactionRemoveAll {
            channel_id,
            removed_from_message_id,
        } => {}
        Event::PresenceReplace { new_presences } => {}
        Event::PresenceUpdate { new_data } => {}
        Event::Ready { data_about_bot } => ready::ready(data_about_bot).await,
        Event::Resume { event } => resume::resume(event).await,
        Event::ShardStageUpdate { update } => {}
        Event::TypingStart { event } => {}
        Event::Unknown { name, raw } => {}
        Event::UserUpdate { old_data, new } => {}
        Event::VoiceServerUpdate { update } => {}
        Event::VoiceStateUpdate { old, new } => {}
        Event::WebhookUpdate {
            guild_id,
            belongs_to_channel_id,
        } => {}
        Event::InteractionCreate { interaction } => {}
    };
    Ok(())
}
