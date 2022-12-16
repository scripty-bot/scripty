use crate::audio_handler::ArcSsrcMaps;
use serenity::prelude::Context;
use songbird::model::payload::Speaking;
use std::num::NonZeroU64;

pub async fn speaking_state_update(state_update: Speaking, ctx: Context, ssrc_state: ArcSsrcMaps) {
    let ssrc = state_update.ssrc;
    debug!(?state_update.speaking, ?state_update.ssrc, ?state_update.user_id, "SpeakingStateUpdate event fired");
    // check if the user ID is in the state update, or in the SSRC map, and bail if not in either
    let user_id = match state_update.user_id.map_or_else(
        || {
            ssrc_state
                .ssrc_user_id_map
                .get(&state_update.ssrc)
                .map(|v| *v.value())
        },
        |id| NonZeroU64::new(id.0),
    ) {
        Some(id) => id,
        None => {
            warn!(?state_update.speaking, ?state_update.ssrc, ?state_update.user_id, "User ID not in state update or map, bailing out");
            return;
        }
    };

    debug!("checking if either ssrc_ignored_map or ssrc_user_data_map does not contain key");
    if !ssrc_state.ssrc_ignored_map.contains_key(&ssrc)
        || !ssrc_state.ssrc_user_data_map.contains_key(&ssrc)
    {
        debug!("either does not contain key, updating data");
        let user = match serenity::model::id::UserId(user_id).to_user(&ctx).await {
            Ok(u) => u,
            Err(e) => {
                error!("failed to fetch user: {}", e);
                return;
            }
        };

        let ignored = user.bot;
        let user_data = (user.tag(), user.face());

        ssrc_state.ssrc_ignored_map.insert(ssrc, ignored);
        ssrc_state.ssrc_user_data_map.insert(ssrc, user_data);
        debug!("updated data");
    }

    if let Some(old_user_id) = ssrc_state
        .ssrc_user_id_map
        .insert(state_update.ssrc, user_id)
    {
        if old_user_id != user_id {
            warn!(
                ?state_update.speaking, ?state_update.ssrc, ?state_update.user_id,
                "Old user ID mapped to this SSRC does not match new! old: {}, new: {}",
                old_user_id, user_id
            );
        }
    }
}
