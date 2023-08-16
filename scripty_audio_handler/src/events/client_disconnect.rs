use std::sync::{
	atomic::{AtomicU8, Ordering},
	Arc,
};

use parking_lot::RwLock;
use serenity::{
	all::{Context, Webhook},
	builder::ExecuteWebhook,
};
use songbird::model::payload::ClientDisconnect;

use crate::{audio_handler::ArcSsrcMaps, types::TranscriptResults};

pub async fn client_disconnect(
	client_disconnect_data: ClientDisconnect,
	ssrc_state: ArcSsrcMaps,
	premium_level: Arc<AtomicU8>,
	ctx: Context,
	webhook: Arc<Webhook>,
	transcript_results: TranscriptResults,
) {
	let user_id = client_disconnect_data.user_id;

	debug!(?user_id, "got ClientDisconnect event");
	// i hate this so much but i don't see a better way of doing it
	let ssrc = {
		let mut ssrc = None;
		for val in ssrc_state.ssrc_user_id_map.iter() {
			if val.value().get() == user_id.0 {
				ssrc = Some(*val.key());
				break;
			}
		}
		match ssrc {
			Some(s) => s,
			None => return,
		}
	};
	debug!(?ssrc, ?user_id, "got ClientDisconnect event");

	assert!(ssrc_state.ssrc_user_id_map.remove(&ssrc).is_some());
	ssrc_state.ssrc_stream_map.remove(&ssrc);
	ssrc_state.ssrc_ignored_map.remove(&ssrc);
	ssrc_state.ssrc_voice_ingest_map.remove(&ssrc);
	let Some((_, (username, avatar_url))) = ssrc_state.ssrc_user_data_map.remove(&ssrc) else {
		warn!(%ssrc, "got no user data for ssrc");
		return;
	};

	#[allow(clippy::wildcard_in_or_patterns)]
	let max_users = match premium_level.load(Ordering::Relaxed) {
		0 => 5,
		1 => 10,
		2 => 25,
		3 => 50,
		4 => 100,
		5 => 250,
		6 | _ => usize::MAX,
	};

	if ssrc_state.active_user_set.remove(&ssrc).is_some()
		&& ssrc_state.active_user_set.len() < max_users
	{
		debug!(?ssrc, "there is space for another active user");
		if let Some(next) = ssrc_state.next_user_list.write().pop_front() {
			debug!(?ssrc, "inserting new user into map");
			ssrc_state.active_user_set.insert(next);
		}
	}

	if let Err(e) = webhook
		.execute(
			&ctx,
			false,
			ExecuteWebhook::new()
				.content(format!("{} disconnected", &username))
				.avatar_url(avatar_url)
				.username(&username),
		)
		.await
	{
		warn!(%ssrc, "failed to send the user leave webhook: {}", e);
	}

	if let Some(transcript_results) = transcript_results {
		let mut transcript_results = transcript_results.write();
		transcript_results.push(format!("[{}] - event: disconnected", username));
	}
}
