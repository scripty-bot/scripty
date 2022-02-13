use crate::types::{ActiveUserSet, NextUserList, SsrcIgnoredMap, SsrcUserDataMap, SsrcUserIdMap};
use serenity::client::Context;
use songbird::model::payload::ClientConnect;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

#[allow(clippy::too_many_arguments, dead_code)]
pub async fn client_connect(
    client_connect_data: ClientConnect,
    ctx: Context,
    ssrc_user_id_map: SsrcUserIdMap,
    ssrc_user_data_map: SsrcUserDataMap,
    ssrc_ignored_map: SsrcIgnoredMap,
    premium_level: Arc<AtomicU8>,
    active_user_set: ActiveUserSet,
    next_user_list: NextUserList,
) {
    let user_id = client_connect_data.user_id;
    let ssrc = client_connect_data.audio_ssrc;

    debug!("user {} connected with ssrc {}", user_id, ssrc);

    ssrc_user_id_map.insert(ssrc, user_id);

    let user = match serenity::model::id::UserId(user_id.0).to_user(ctx).await {
        Ok(u) => u,
        Err(e) => {
            error!("failed to fetch user: {}", e);
            return;
        }
    };

    let ignored = user.bot;
    ssrc_ignored_map.insert(ssrc, ignored);

    if ignored {
        return;
    }

    ssrc_user_data_map.insert(ssrc, (user.tag(), user.face()));

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

    if active_user_set.len() < max_users {
        active_user_set.insert(ssrc);
    } else {
        next_user_list.write().push_back(ssrc);
    }
}
