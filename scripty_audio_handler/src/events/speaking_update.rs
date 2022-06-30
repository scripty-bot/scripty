use crate::types::{
    SsrcLastPktIdMap, SsrcMissedPktList, SsrcMissedPktMap, SsrcStreamMap, SsrcUserDataMap,
    SsrcUserIdMap, SsrcVoiceIngestMap,
};
use scripty_audio::CompleteResult;
use serenity::builder::ExecuteWebhook;
use serenity::client::Context;
use serenity::model::channel::Embed;
use serenity::model::webhook::Webhook;
use songbird::events::context_data::SpeakingUpdateData;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

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

    let verbose = verbose.load(Ordering::Relaxed);

    // user has finished speaking, run the STT algo
    if !ssrc_stream_map.contains_key(&ssrc) {
        ssrc_stream_map.insert(
            ssrc,
            scripty_audio::get_stream("en", verbose).expect("en invalid lang?"),
        );
        // return early since we can't do anything without a stream
        return;
    }
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
    let mut st = None;
    let mut et = None;
    if verbose {
        st = Some(Instant::now());
    }
    enum CompleteResultWrapper {
        Single {
            text: String,
        },
        Multiple {
            count: usize,
            main_text: Option<String>,
            main_confidence: Option<f32>,
        },
    }

    let res = {
        let mut stream = ssrc_stream_map
            .get_mut(&ssrc)
            .expect("already asserted stream exists earlier");
        scripty_utils::block_in_place(|| {
            let res = match stream.final_result() {
                CompleteResult::Single(res) => CompleteResultWrapper::Single {
                    text: res.text.to_string(),
                },
                CompleteResult::Multiple(res) => {
                    let count = res.alternatives.len();
                    let main = res.alternatives.get(0);
                    let main_text = main.map(|res| res.text.to_string());
                    let main_confidence = main.map(|res| res.confidence);
                    CompleteResultWrapper::Multiple {
                        count,
                        main_text,
                        main_confidence,
                    }
                }
            };
            stream.reset();
            res
        })
        .await
    };
    if verbose {
        et = Some(Instant::now());
    }

    let text = match res {
        // single means verbose was set to false
        CompleteResultWrapper::Single { text } => {
            if text.is_empty() {
                return;
            }
            webhook_execute.content(&text);
            text.to_string()
        }
        // multiple means verbose was set to true
        CompleteResultWrapper::Multiple {
            count,
            main_text,
            main_confidence,
        } => {
            if count == 0 {
                return;
            }

            // we know verbose was set, so unwrap the timings
            let st = st.expect("expected st to be set");
            let et = et.expect("expected et to be set");
            let elapsed = et.duration_since(st).as_nanos();

            let text = main_text.expect("asserted at least one item exists");
            let confidence = main_confidence.expect("asserted at least one item exists");

            webhook_execute.embeds(vec![Embed::fake(|e| {
                e.title(format!("Transcription 1/{}", count))
                    .field("Text", &text, true)
                    .field("Confidence", &format!("{:.2}", confidence), true)
                    .field("Process Time", &format!("{}ns", elapsed), true)
                    .footer(|f| f.text(format!("SSRC {}", ssrc)).icon_url(avatar_url))
            })]);
            text
        }
    };
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
                        .destroy(text.to_string())
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
