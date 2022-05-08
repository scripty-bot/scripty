use crate::types::{
    SsrcLastPktIdMap, SsrcMissedPktList, SsrcMissedPktMap, SsrcStreamMap, SsrcUserDataMap,
};
use serenity::builder::ExecuteWebhook;
use serenity::client::Context;
use serenity::model::channel::Embed;
use serenity::model::webhook::Webhook;
use songbird::events::context_data::SpeakingUpdateData;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[allow(clippy::too_many_arguments)]
pub async fn speaking_update(
    update: SpeakingUpdateData,
    ctx: Context,
    webhook: Arc<Webhook>,
    ssrc_user_data_map: SsrcUserDataMap,
    ssrc_stream_map: SsrcStreamMap,
    ssrc_last_pkt_id_map: SsrcLastPktIdMap,
    ssrc_missed_pkt_map: SsrcMissedPktMap,
    ssrc_missed_pkt_list: SsrcMissedPktList,
    verbose: Arc<AtomicBool>,
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
    let old_stream = match ssrc_stream_map.insert(
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
    if verbose.load(Ordering::Relaxed) {
        let res = scripty_utils::block_in_place(|| old_stream.finish_stream_with_metadata(3)).await;
        debug!(?ssrc, "ran stream transcription");
        match res {
            Ok(res) if res.num_transcripts() != 0 => {
                // SAFETY: we have already checked len != 0, so there must be at least one item
                let transcript = unsafe { res.transcripts().get_unchecked(0) };

                webhook_execute.embeds(vec![Embed::fake(|e| {
                    e.title(format!("Transcript 1/{}", res.num_transcripts()))
                        .field("Transcription", transcript.to_owned(), false)
                        .field("Confidence", transcript.confidence(), false)
                        .footer(|f| f.text(format!("ssrc {}", ssrc)))
                })]);
            }
            Err(e) => {
                error!(?ssrc, "stream transcription errored: {}", e);

                webhook_execute.content(format!(
                    "internal error: running stt algorithm failed with error: {}\nssrc {}",
                    e, ssrc
                ));
            }
            _ => return,
        }
    } else {
        let res = scripty_utils::block_in_place(|| old_stream.finish_stream()).await;
        debug!(?ssrc, "ran stream transcription");

        match res {
            Ok(res) if !res.is_empty() => {
                webhook_execute.content(res);
            }
            Err(e) => {
                error!(?ssrc, "stream transcription errored: {}", e);

                webhook_execute.content(format!(
                    "internal error: running stt algorithm failed with error: {}\nssrc {}",
                    e, ssrc
                ));
            }
            _ => return,
        }
    }
    debug!(?ssrc, "stream transcription succeeded");

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
