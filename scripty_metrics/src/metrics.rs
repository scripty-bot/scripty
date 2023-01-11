//! Code used from sushiibot
//! https://raw.githubusercontent.com/sushiibot/sushii-2/888fbcdaecc0838e5c3735a5aac677a2d327ef10/src/model/metrics.rs

use chrono::{NaiveDateTime, Utc};
use once_cell::sync::OnceCell;
use prometheus::{IntCounter, IntCounterVec, IntGauge, IntGaugeVec, Opts, Registry};
use prometheus_static_metric::make_static_metric;
use std::sync::Arc;

make_static_metric! {
    pub label_enum UserType {
        user,
        other_bot,
        own,
    }
    pub struct MessageCounterVec: IntCounter {
        "user_type" => UserType,
    }

    pub label_enum EventType {
        command_permissions_update,
        auto_mod_rule_create,
        auto_mod_rule_update,
        auto_mod_rule_delete,
        auto_mod_action_execution,
        channel_create,
        channel_delete,
        channel_pins_update,
        channel_update,
        guild_ban_add,
        guild_ban_remove,
        guild_create,
        guild_delete,
        guild_emojis_update,
        guild_integrations_update,
        guild_member_add,
        guild_member_remove,
        guild_member_update,
        guild_members_chunk,
        guild_role_create,
        guild_role_delete,
        guild_role_update,
        guild_stickers_update,
        guild_update,
        invite_create,
        invite_delete,
        message_create,
        message_delete,
        message_delete_bulk,
        message_update,
        presence_update,
        presences_replace,
        reaction_add,
        reaction_remove,
        reaction_remove_all,
        reaction_remove_emoji,
        ready,
        resumed,
        typing_start,
        user_update,
        voice_state_update,
        voice_server_update,
        webhook_update,
        interaction_create,
        integration_create,
        integration_update,
        integration_delete,
        stage_instance_create,
        stage_instance_update,
        stage_instance_delete,
        thread_create,
        thread_update,
        thread_delete,
        thread_list_sync,
        thread_member_update,
        thread_members_update,
        guild_scheduled_event_create,
        guild_scheduled_event_update,
        guild_scheduled_event_delete,
        guild_scheduled_event_user_add,
        guild_scheduled_event_user_remove,
        unknown,
    }
    pub struct EventCounterVec: IntCounter {
        "event_type" => EventType,
    }

    pub label_enum CommandsUsed {
        credits,
        data_storage,
        donate,
        block,
        help,
        join,
        language,
        ping,
        register_cmds,
        setup,
        unknown
    }
    pub struct CommandsUsedVec: IntCounter {
        "command_name" => CommandsUsed,
    }

    pub label_enum RuntimeMetrics {
        workers_count,
        total_park_count,
        max_park_count,
        min_park_count,
        total_noop_count,
        max_noop_count,
        min_noop_count,
        total_steal_count,
        max_steal_count,
        min_steal_count,
        num_remote_schedules,
        total_local_schedule_count,
        max_local_schedule_count,
        min_local_schedule_count,
        total_overflow_count,
        max_overflow_count,
        min_overflow_count,
        total_polls_count,
        max_polls_count,
        min_polls_count,
        total_busy_duration,
        max_busy_duration,
        min_busy_duration,
        injection_queue_depth,
        total_local_queue_depth,
        max_local_queue_depth,
        min_local_queue_depth,
        elapsed,
    }
    pub struct RuntimeMetricsVec: IntGauge {
        "runtime_metrics" => RuntimeMetrics,
    }

    pub label_enum LatencyType {
        websocket,
        http,
        db,
        command_process,
    }
    pub struct LatencyVec: IntGauge {
        "latency_type" => LatencyType,
    }
}

pub static METRICS: OnceCell<Arc<Metrics>> = OnceCell::new();

pub fn get_metrics() -> Arc<Metrics> {
    METRICS.get().expect("metrics not initialized").clone()
}

pub struct Metrics {
    pub registry: Registry,
    pub start_time: NaiveDateTime,
    pub messages: MessageCounterVec,
    pub events: EventCounterVec,
    pub guilds: IntGauge,
    pub users: IntGauge,
    pub ms_transcribed: IntCounter,
    pub audio_bytes_processed: IntCounter,
    pub total_events: IntCounter,
    // TODO: switch to Histogram
    pub avg_audio_process_time: IntGauge,
    pub total_commands: IntCounter,
    pub stt_server_fetch_success: IntCounter,
    pub stt_server_fetch_failure: IntCounter,
    pub commands: CommandsUsedVec,
    pub runtime_metrics: RuntimeMetricsVec,
    pub latency: LatencyVec,
}

impl Metrics {
    pub(crate) fn new() -> Arc<Self> {
        let registry = Registry::new_custom(Some("scripty".into()), None).unwrap();

        let messages_vec =
            IntCounterVec::new(Opts::new("messages", "Received messages"), &["user_type"]).unwrap();
        let messages_static_vec = MessageCounterVec::from(&messages_vec);
        registry.register(Box::new(messages_vec)).unwrap();

        let events_vec =
            IntCounterVec::new(Opts::new("events", "Gateway events"), &["event_type"]).unwrap();
        let events_static_vec = EventCounterVec::from(&events_vec);
        registry.register(Box::new(events_vec)).unwrap();

        let guilds_gauge = IntGauge::new("guilds", "Current guilds").unwrap();
        registry.register(Box::new(guilds_gauge.clone())).unwrap();

        let members_gauge = IntGauge::new("members", "Current members").unwrap();
        registry.register(Box::new(members_gauge.clone())).unwrap();

        let ms_transcribed =
            IntCounter::new("audio_transcribed", "Milliseconds of audio transcribed").unwrap();
        registry.register(Box::new(ms_transcribed.clone())).unwrap();

        let events = IntCounter::new("total_events", "Total gateway events").unwrap();
        registry.register(Box::new(events.clone())).unwrap();

        let audio_process = IntGauge::new(
            "avg_audio_process_time",
            "Average time to process one audio packet",
        )
        .unwrap();
        registry.register(Box::new(audio_process.clone())).unwrap();

        let total_commands_used =
            IntCounter::new("total_commands_used", "All commands used").unwrap();
        registry
            .register(Box::new(total_commands_used.clone()))
            .unwrap();

        let commands_used = IntCounterVec::new(
            Opts::new("commands_used", "Commands used"),
            &["command_name"],
        )
        .unwrap();
        let commands_used_static = CommandsUsedVec::from(&commands_used);
        registry.register(Box::new(commands_used)).unwrap();

        let runtime_metrics_stats = IntGaugeVec::new(
            Opts::new("runtime_metrics", "Tokio runtime metrics"),
            &["runtime_metrics"],
        )
        .unwrap();
        let runtime_metrics_static = RuntimeMetricsVec::from(&runtime_metrics_stats);
        registry.register(Box::new(runtime_metrics_stats)).unwrap();

        let latency_stats = IntGaugeVec::new(
            Opts::new("latency", "Latency of various components"),
            &["latency_type"],
        )
        .expect("failed to init latency stats");
        let latency_static = LatencyVec::from(&latency_stats);
        registry.register(Box::new(latency_stats)).unwrap();

        let audio_bytes_processed =
            IntCounter::new("audio_bytes_processed", "Bytes of audio processed").unwrap();
        registry
            .register(Box::new(audio_bytes_processed.clone()))
            .unwrap();

        let stt_server_fetch_success = IntCounter::new(
            "stt_server_fetch_success",
            "Successful stream creations to any STT server",
        )
        .unwrap();
        registry
            .register(Box::new(stt_server_fetch_success.clone()))
            .unwrap();

        let stt_server_fetch_failure = IntCounter::new(
            "stt_server_fetch_failure",
            "Failed stream creations to any STT server",
        )
        .unwrap();
        registry
            .register(Box::new(stt_server_fetch_failure.clone()))
            .unwrap();

        Arc::new(Self {
            registry,
            start_time: Utc::now().naive_utc(),
            messages: messages_static_vec,
            events: events_static_vec,
            guilds: guilds_gauge,
            users: members_gauge,
            ms_transcribed,
            audio_bytes_processed,
            total_events: events,
            avg_audio_process_time: audio_process,
            total_commands: total_commands_used,
            commands: commands_used_static,
            runtime_metrics: runtime_metrics_static,
            latency: latency_static,
            stt_server_fetch_success,
            stt_server_fetch_failure,
        })
    }
}
