use crate::types::{ActiveUserSet, NextUserList, SsrcIgnoredMap, SsrcStreamMap, SsrcUserIdMap};
use serenity::client::Context;
use serenity::model::webhook::Webhook;
use songbird::events::context_data::SpeakingUpdateData;
use songbird::model::payload::ClientConnect;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

pub async fn client_connect(
    client_connect_data: &ClientConnect,
    ctx: Context,
    ssrc_user_id_map: SsrcUserIdMap,
    ssrc_ignored_map: SsrcIgnoredMap,
    premium_level: Arc<AtomicU8>,
    active_user_set: ActiveUserSet,
    next_user_list: NextUserList,
) {
    ssrc_user_id_map.insert(client_connect_data.audio_ssrc, client_connect_data.user_id);

    let ignored = serenity::model::id::UserId(user_id.0)
        .to_user(ctx)
        .await
        .map_or(false, |x| x.bot);

    ssrc_ignored_map.insert(ssrc, ignored);

    if ignored {
        return;
    }

    let max_users = match premium_level.load(Ordering::Relaxed) {
        0 => 10,
        1 => 25,
        2 => 50,
        3 => 100,
        4 | _ => usize::MAX,
    };

    if active_user_set.len() < max_users {
        active_user_set.insert(ssrc);
    } else {
        next_user_list.write().push_back(ssrc);
    }
}
