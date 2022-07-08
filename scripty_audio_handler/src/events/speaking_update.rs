use crate::types::{
    SsrcLastPktIdMap, SsrcMissedPktList, SsrcMissedPktMap, SsrcStreamMap, SsrcUserDataMap,
    SsrcUserIdMap, SsrcVoiceIngestMap,
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
    if update.speaking {
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

    let new_stream = match scripty_audio::Stream::new("en", verbose).await {
        Ok(s) => s,
        Err(e) => {
            error!(?ssrc, "failed to create new stream: {}", e);
            return;
        }
    };

    // user has finished speaking, run the STT algo
    let old_stream = match ssrc_stream_map.insert(ssrc, new_stream) {
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
    let transcript = if verbose {
        let res = old_stream.get_result_verbose().await;
        debug!(?ssrc, "ran stream transcription");
        match res {
            Ok(res) if res.num_transcripts != 0 => {
                let main_transcript = res
                    .main_transcript
                    .expect("asserted there is at least one transcript");
                if res.main_transcript.is_empty() {
                    return;
                }

                let confidence = res
                    .main_confidence
                    .expect("asserted there is at least one transcript")
                    .to_string();

                webhook_execute.embeds(vec![Embed::fake(|e| {
                    e.title(format!("Transcript 1/{}", res.num_transcripts))
                        .field("Transcription", &main_transcript, false)
                        .field("Confidence", &confidence, false)
                        .footer(|f| f.text(format!("ssrc {}", ssrc)))
                })]);
                Some(main_transcript)
            }
            Err(e) => {
                error!(?ssrc, "stream transcription errored: {}", e);

                webhook_execute.content(format!(
                    "internal error: running stt algorithm failed with error: {}\nssrc {}",
                    e, ssrc
                ));
                None
            }
            _ => return,
        }
    } else {
        let res = old_stream.get_result().await;
        debug!(?ssrc, "ran stream transcription");

        match res {
            Ok(res) if !res.result.is_empty() => {
                webhook_execute.content(res.result.clone());
                Some(res.result)
            }
            Err(e) => {
                error!(?ssrc, "stream transcription errored: {}", e);

                webhook_execute.content(format!(
                    "internal error: running stt algorithm failed with error: {}\nssrc {}",
                    e, ssrc
                ));
                None
            }
            _ => return,
        }
    };
    debug!(?ssrc, "stream transcription succeeded");

    if let Some(transcript) = transcript {
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
                            .destroy(transcript)
                            .await;
                    }
                }
            }
        } else {
            debug!(?ssrc, "no voice ingest for SSRC");
        }
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
