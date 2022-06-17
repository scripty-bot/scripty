use crate::types::{
    SsrcLastPktIdMap, SsrcMissedPktList, SsrcMissedPktMap, SsrcStreamMap, SsrcUserDataMap,
};
use scripty_audio::coqui_stt_sys;
use serenity::builder::ExecuteWebhook;
use serenity::client::Context;
use serenity::model::channel::Embed;
use serenity::model::webhook::Webhook;
use songbird::events::context_data::SpeakingUpdateData;
use std::ffi::CStr;
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
        let res = scripty_utils::block_in_place(|| {
            let retval =
                unsafe { coqui_stt_sys::STT_FinishStreamWithMetadata(old_stream.as_ptr(), 3) };
            if retval.is_null() {
                error!("STT_FinishStreamWithMetadata returned null");
                None
            } else {
                Some(retval)
            }
        })
        .await;
        debug!(?ssrc, "ran stream transcription");
        match res {
            Some(res) if unsafe { *res }.num_transcripts != 0 => {
                // obtain the first transcription
                let transcriptions = unsafe {
                    std::slice::from_raw_parts((*res).transcripts, (*res).num_transcripts as usize)
                };
                // get the first transcription
                let transcript = unsafe { transcriptions.get_unchecked(0) };
                // get the transcript text
                let tokens = unsafe {
                    std::slice::from_raw_parts(transcript.tokens, transcript.num_tokens as usize)
                };
                let transcript_text = tokens
                    .iter()
                    .map(|t| unsafe {
                        CStr::from_ptr(t.text)
                            .to_str()
                            .expect("invalid string")
                            .to_owned()
                    })
                    .collect::<Vec<_>>()
                    .join(" ");

                webhook_execute.embeds(vec![Embed::fake(|e| {
                    e.title(format!("Transcript 1/{}", unsafe { *res }.num_transcripts))
                        .field("Transcription", &transcript_text, false)
                        .field("Confidence", &transcript.confidence.to_string(), false)
                        .footer(|f| f.text(format!("ssrc {}", ssrc)))
                })]);

                unsafe { coqui_stt_sys::STT_FreeMetadata(res) };
            }
            None => {
                webhook_execute.content(format!(
                    "internal error: running stt algorithm failed\nssrc {}",
                    ssrc
                ));
            }
            _ => return,
        }
    } else {
        let res = scripty_utils::block_in_place(|| {
            let retval = unsafe { coqui_stt_sys::STT_FinishStream(old_stream.as_ptr()) };
            if retval.is_null() {
                error!("STT_FinishStream returned null");
                None
            } else {
                Some(retval)
            }
        })
        .await;

        debug!(?ssrc, "ran stream transcription");

        match res {
            Some(res) => {
                let cstr = unsafe { CStr::from_ptr(res) };
                // convert the cstr to a string
                let transcript_text = cstr.to_str().expect("invalid string").to_owned();

                webhook_execute.content(transcript_text);

                unsafe { coqui_stt_sys::STT_FreeString(res) };
            }
            None => {
                error!(?ssrc, "stream transcription errored");

                webhook_execute.content(format!(
                    "internal error: running stt algorithm failed with error\nssrc {}",
                    ssrc
                ));
            }
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
