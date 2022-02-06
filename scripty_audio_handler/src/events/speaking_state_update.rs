use crate::types::{SsrcIgnoredMap, SsrcUserIdMap};
use serenity::prelude::Context;
use songbird::model::payload::Speaking;

pub async fn speaking_state_update(
    state_update: Speaking,
    ctx: Context,
    ssrc_user_id_map: SsrcUserIdMap,
    ssrc_ignored_map: SsrcIgnoredMap,
) {
    let ssrc = state_update.ssrc;
    debug!(?state_update.speaking, ?state_update.ssrc, ?state_update.user_id, "SpeakingStateUpdate event fired");
    // check if the user ID is in the state update, or in the SSRC map, and bail if not in either
    let user_id = match state_update
        .user_id
        .or_else(|| ssrc_user_id_map.get(&state_update.ssrc).map(|v| *v.value()))
    {
        Some(id) => id,
        None => {
            warn!(?state_update.speaking, ?state_update.ssrc, ?state_update.user_id, "User ID not in state update or map, bailing out");
            return;
        }
    };

    if !ssrc_ignored_map.contains_key(&ssrc) {
        let ignored = serenity::model::id::UserId(user_id.0)
            .to_user(ctx)
            .await
            .map_or(false, |x| x.bot);

        ssrc_ignored_map.insert(ssrc, ignored);
    }

    if let Some(old_user_id) = ssrc_user_id_map.insert(state_update.ssrc, user_id) {
        if old_user_id != user_id {
            warn!(
                ?state_update.speaking, ?state_update.ssrc, ?state_update.user_id,
                "Old user ID mapped to this SSRC does not match new! old: {}, new: {}",
                old_user_id, user_id
            );
        }
    }
}
