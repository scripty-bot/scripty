use crate::types::{
    ActiveUserSet, NextUserList, SsrcIgnoredMap, SsrcStreamMap, SsrcUserDataMap, SsrcUserIdMap,
};
use songbird::model::payload::ClientDisconnect;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

#[allow(clippy::too_many_arguments)]
pub async fn client_disconnect(
    client_disconnect_data: ClientDisconnect,
    ssrc_user_id_map: SsrcUserIdMap,
    ssrc_stream_map: SsrcStreamMap,
    ssrc_user_data_map: SsrcUserDataMap,
    ssrc_ignored_map: SsrcIgnoredMap,
    active_user_set: ActiveUserSet,
    next_user_list: NextUserList,
    premium_level: Arc<AtomicU8>,
) {
    let user_id = client_disconnect_data.user_id;

    debug!(?user_id, "got ClientDisconnect event");
    // i hate this so much but i don't see a better way of doing it
    let ssrc = {
        let mut ssrc = None;
        for val in ssrc_user_id_map.iter() {
            if val.value() == &user_id {
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

    assert!(ssrc_user_id_map.remove(&ssrc).is_some());
    ssrc_stream_map.remove(&ssrc);
    ssrc_user_data_map.remove(&ssrc);
    ssrc_ignored_map.remove(&ssrc);

    #[allow(clippy::wildcard_in_or_patterns)]
    let max_users = match premium_level.load(Ordering::Relaxed) {
        0 => 10,
        1 => 25,
        2 => 50,
        3 => 100,
        4 | _ => usize::MAX,
    };

    if active_user_set.remove(&ssrc).is_some() && active_user_set.len() < max_users {
        debug!(?ssrc, "there is space for another active user");
        if let Some(next) = next_user_list.write().pop_front() {
            debug!(?ssrc, "inserting new user into map");
            active_user_set.insert(next);
        }
    }
}
