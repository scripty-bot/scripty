use std::time::Instant;

use serenity::{
	client::{Context, RawEventHandler as SerenityRawEventHandler},
	model::event::{Event, MessageCreateEvent, ReadyEvent, ResumedEvent, VoiceStateUpdateEvent},
};

pub struct RawEventHandler;

#[async_trait]
impl SerenityRawEventHandler for RawEventHandler {
	async fn raw_event(&self, ctx: Context, event: Event) {
		// we need to handle command latency measurements here too,
		// as poise overwrites serenity and calls it after command processing
		let st = Instant::now();
		match &event {
			Event::MessageCreate(e) => {
				scripty_metrics::measure_start_latency(st, e.message.id.get())
			}
			Event::InteractionCreate(e) => {
				scripty_metrics::measure_start_latency(st, e.interaction.id().get())
			}
			_ => {}
		}

		let metrics = scripty_metrics::get_metrics();

		// increment the total number of events
		metrics.total_events.inc();

		// increment the total number of events per type
		match event {
			Event::CommandPermissionsUpdate(_) => metrics.events.command_permissions_update.inc(),
			Event::AutoModRuleCreate(_) => metrics.events.auto_mod_rule_create.inc(),
			Event::AutoModRuleUpdate(_) => metrics.events.auto_mod_rule_update.inc(),
			Event::AutoModRuleDelete(_) => metrics.events.auto_mod_rule_delete.inc(),
			Event::AutoModActionExecution(_) => metrics.events.auto_mod_action_execution.inc(),
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
			Event::GuildUpdate(_) => metrics.events.guild_update.inc(),
			Event::InviteCreate(_) => metrics.events.invite_create.inc(),
			Event::InviteDelete(_) => metrics.events.invite_delete.inc(),
			Event::MessageCreate(_) => metrics.events.message_create.inc(),
			Event::MessageDelete(_) => metrics.events.message_delete.inc(),
			Event::MessageDeleteBulk(_) => metrics.events.message_delete_bulk.inc(),
			Event::MessageUpdate(_) => metrics.events.message_update.inc(),
			Event::PresenceUpdate(_) => metrics.events.presence_update.inc(),
			Event::ReactionAdd(_) => metrics.events.reaction_add.inc(),
			Event::ReactionRemove(_) => metrics.events.reaction_remove.inc(),
			Event::ReactionRemoveAll(_) => metrics.events.reaction_remove_all.inc(),
			Event::ReactionRemoveEmoji(_) => metrics.events.reaction_remove_emoji.inc(),
			Event::Ready(_) => metrics.events.ready.inc(),
			Event::Resumed(_) => metrics.events.resumed.inc(),
			Event::TypingStart(_) => metrics.events.typing_start.inc(),
			Event::UserUpdate(_) => metrics.events.user_update.inc(),
			Event::VoiceStateUpdate(_) => metrics.events.voice_state_update.inc(),
			Event::VoiceServerUpdate(_) => metrics.events.voice_server_update.inc(),
			Event::WebhookUpdate(_) => metrics.events.webhook_update.inc(),
			Event::InteractionCreate(_) => metrics.events.interaction_create.inc(),
			Event::IntegrationCreate(_) => metrics.events.integration_create.inc(),
			Event::IntegrationUpdate(_) => metrics.events.integration_update.inc(),
			Event::IntegrationDelete(_) => metrics.events.integration_delete.inc(),
			Event::StageInstanceCreate(_) => metrics.events.stage_instance_create.inc(),
			Event::StageInstanceUpdate(_) => metrics.events.stage_instance_update.inc(),
			Event::StageInstanceDelete(_) => metrics.events.stage_instance_delete.inc(),
			Event::ThreadCreate(_) => metrics.events.thread_create.inc(),
			Event::ThreadUpdate(_) => metrics.events.thread_update.inc(),
			Event::ThreadDelete(_) => metrics.events.thread_delete.inc(),
			Event::ThreadListSync(_) => metrics.events.thread_list_sync.inc(),
			Event::ThreadMemberUpdate(_) => metrics.events.thread_member_update.inc(),
			Event::ThreadMembersUpdate(_) => metrics.events.thread_members_update.inc(),
			Event::GuildScheduledEventCreate(_) => {
				metrics.events.guild_scheduled_event_create.inc()
			}
			Event::GuildScheduledEventUpdate(_) => {
				metrics.events.guild_scheduled_event_update.inc()
			}
			Event::GuildScheduledEventDelete(_) => {
				metrics.events.guild_scheduled_event_delete.inc()
			}
			Event::GuildScheduledEventUserAdd(_) => {
				metrics.events.guild_scheduled_event_user_add.inc()
			}
			Event::GuildScheduledEventUserRemove(_) => {
				metrics.events.guild_scheduled_event_user_remove.inc()
			}
			Event::GuildAuditLogEntryCreate(_) => metrics.events.guild_audit_log_entry_create.inc(),
			Event::VoiceChannelStatusUpdate(_) => metrics.events.voice_state_update.inc(),
			Event::EntitlementCreate(_) => metrics.events.entitlement_create.inc(),
			Event::EntitlementUpdate(_) => metrics.events.entitlement_update.inc(),
			Event::EntitlementDelete(_) => metrics.events.entitlement_delete.inc(),
			Event::MessagePollVoteAdd(_) => metrics.events.message_poll_vote_add.inc(),
			Event::MessagePollVoteRemove(_) => metrics.events.message_poll_vote_remove.inc(),
			_ => metrics.events.unknown.inc(),
		}

		// Dispatch events as normal
		match event {
			Event::MessageCreate(MessageCreateEvent { message, .. }) => {
				super::normal::message(ctx, message).await
			}
			Event::Ready(ReadyEvent { ready, .. }) => super::normal::ready(ctx, ready).await,
			Event::Resumed(ResumedEvent { .. }) => super::normal::resume(ctx).await,
			Event::VoiceStateUpdate(VoiceStateUpdateEvent { voice_state, .. }) => {
				super::normal::voice_state_update(ctx, voice_state).await
			}
			_ => {}
		}
	}
}
