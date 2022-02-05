use crate::types::{SsrcStreamMap, SsrcUserIdMap};
use serenity::client::Context;
use serenity::model::webhook::Webhook;
use songbird::events::context_data::SpeakingUpdateData;
use songbird::model::payload::ClientConnect;
use std::sync::Arc;

pub async fn client_connect(
    client_connect: &ClientConnect,
    ctx: Arc<Context>,
    webhook: Arc<Webhook>,
    ssrc_user_id_map: SsrcUserIdMap,
    ssrc_stream_map: SsrcStreamMap,
) {
    ssrc_user_id_map.insert(client_connect.audio_ssrc)
}
