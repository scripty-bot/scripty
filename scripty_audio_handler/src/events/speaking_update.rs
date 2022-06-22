use crate::types::{
    SsrcLastPktIdMap, SsrcMissedPktList, SsrcMissedPktMap, SsrcStreamMap, SsrcUserDataMap,
    SsrcUserIdMap, SsrcVoiceIngestMap,
};
use serenity::builder::ExecuteWebhook;
use serenity::client::Context;
use serenity::model::webhook::Webhook;
use songbird::events::context_data::SpeakingUpdateData;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[allow(clippy::too_many_arguments)]
pub async fn speaking_update(
    update: SpeakingUpdateData,
    ctx: Context,
    webhook: Arc<Webhook>,
    ssrc_user_id_map: SsrcUserIdMap,
    ssrc_user_data_map: SsrcUserDataMap,
    ssrc_stream_map: SsrcStreamMap,
    ssrc_last_pkt_id_map: SsrcLastPktIdMap,
    ssrc_missed_pkt_map: SsrcMissedPktMap,
    ssrc_missed_pkt_list: SsrcMissedPktList,
    ssrc_voice_ingest_map: SsrcVoiceIngestMap,
    _verbose: Arc<AtomicBool>,
) {
    let ssrc = update.ssrc;
    debug!(?ssrc, ?update.speaking, "got SpeakingUpdate event");
    if !update.speaking {
        return;
    }

    // clear old data
    ssrc_last_pkt_id_map.remove(&ssrc);
    if let Some(pkt_id_list) = ssrc_missed_pkt_list.remove(&ssrc) {
        for pkt in pkt_id_list.1 {
            ssrc_missed_pkt_map.remove(&(ssrc, pkt));
        }
    }

    // user has finished speaking, run the STT algo
    let mut old_stream = match ssrc_stream_map.insert(
        ssrc,
        scripty_audio::get_stream("en").expect("en invalid lang?"),
    ) {
        Some(s) => s,
        None => {
            warn!(?ssrc, "stream not found in ssrc_stream_map, bailing");
            return;
        }
    };
    debug!(?ssrc, "found Stream for SSRC");

    let user_data = ssrc_user_data_map.get(&ssrc);
    let (username, avatar_url) = match user_data {
        Some(ref d) => d.value(),
        None => {
            warn!(?ssrc, "user data not found in ssrc_user_data_map, bailing");
            return;
        }
    };
    debug!(?ssrc, "found user data for SSRC");

    let mut webhook_execute = ExecuteWebhook::default();

    debug!(?ssrc, "running transcription");
    let res = scripty_utils::block_in_place(|| old_stream.final_result()).await;
    if res.text.is_empty() {
        return;
    }
    webhook_execute.content(res.text);
    debug!(?ssrc, "stream transcription succeeded");

    if ssrc_voice_ingest_map
        .get(&ssrc)
        .map_or(false, |v| v.is_some())
    {
        debug!(?ssrc, "found voice ingest for SSRC");
        if let Some(user_id) = ssrc_user_id_map.get(&ssrc) {
            debug!(?ssrc, "found user_id for SSRC");
            if let Some(ingest) =
                scripty_data_storage::VoiceIngest::new(user_id.0, "en".to_string()).await
            {
                debug!(?ssrc, "created VoiceIngest, and retrieved old one");
                if let Some(voice_ingest) = ssrc_voice_ingest_map.insert(ssrc, Some(ingest)) {
                    debug!(?ssrc, "found old VoiceIngest, finalizing");
                    voice_ingest
                        .expect("asserted voice ingest object already exists")
                        .destroy(res.text.to_string())
                        .await;
                }
            }
        }
    } else {
        debug!(?ssrc, "no voice ingest for SSRC");
    }

    webhook_execute.username(username).avatar_url(avatar_url);
    debug!(?ssrc, "sending webhook msg");
    let res = webhook
        .execute(&ctx, false, |e| {
            *e = webhook_execute;
            e
        })
        .await;
    if let Err(e) = res {
        error!(?ssrc, "failed to send webhook msg: {}", e);
    }
}
