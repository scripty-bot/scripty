use crate::types::{SsrcStreamMap, SsrcUserDataMap};
use serenity::builder::ExecuteWebhook;
use serenity::client::Context;
use serenity::model::channel::Embed;
use serenity::model::webhook::Webhook;
use songbird::events::context_data::SpeakingUpdateData;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub async fn speaking_update(
    update: SpeakingUpdateData,
    ctx: Context,
    webhook: Arc<Webhook>,
    ssrc_user_data_map: SsrcUserDataMap,
    ssrc_stream_map: SsrcStreamMap,
    verbose: Arc<AtomicBool>,
) {
    let ssrc = update.ssrc;
    debug!(?ssrc, ?update.speaking, "got SpeakingUpdate event");
    if update.speaking {
        // user has started speaking, begin a new model
    } else {
        // user has finished speaking, run the STT algo
        let old_stream = match ssrc_stream_map.insert(
            ssrc,
            scripty_audio::get_stream("en").expect("en invalid lang?"),
        ) {
            Some(s) => s,
            None => return,
        };
        debug!(?ssrc, "found Stream for SSRC");

        let user_data = ssrc_user_data_map.get(&ssrc);
        let (username, avatar_url) = match user_data {
            Some(ref d) => d.value(),
            None => return,
        };
        debug!(?ssrc, "found user data for SSRC");

        let mut webhook_execute = ExecuteWebhook::default();

        if verbose.load(Ordering::Relaxed) {
            let res = scripty_audio::run_stt_with_metadata(old_stream, 1).await;
            debug!(?ssrc, "ran stream transcription");
            match res {
                Ok(Ok(res)) if res.num_transcripts() != 0 => {
                    // SAFETY: we have already checked len != 0, so there must be at least one item
                    let transcript = unsafe { res.transcripts().get_unchecked(0) };

                    webhook_execute.embeds(vec![Embed::fake(|e| {
                        e.title(format!("Transcript 1/{}", res.num_transcripts()))
                            .field("Transcription", transcript.to_owned(), false)
                            .field("Confidence", transcript.confidence(), false)
                            .footer(|f| f.text(format!("ssrc {}", ssrc)))
                    })]);
                }
                Ok(Err(e)) => {
                    error!(?ssrc, "stream transcription errored: {}", e);

                    webhook_execute.content(format!(
                        "internal error: running stt algorithm failed with error: {}\nssrc {}",
                        e, ssrc
                    ));
                }
                Err(_) => {
                    error!(?ssrc, "stream transcription errored: sender hung up");

                    webhook_execute.content(format!(
                        "internal error: sender hung up (this is usually global and fatal)\nssrc {}",
                        ssrc
                    ));
                }
                _ => return,
            }
        } else {
            let res = scripty_audio::run_stt(old_stream).await;
            debug!(?ssrc, "ran stream transcription");

            match res {
                Ok(Ok(res)) if !res.is_empty() => {
                    webhook_execute.content(res);
                }
                Ok(Err(e)) => {
                    error!(?ssrc, "stream transcription errored: {}", e);

                    webhook_execute.content(format!(
                        "internal error: running stt algorithm failed with error: {}\nssrc {}",
                        e, ssrc
                    ));
                }
                Err(_) => {
                    error!(?ssrc, "stream transcription errored: sender hung up");

                    webhook_execute.content(format!(
                        "internal error: sender hung up (this is usually global and fatal)\nssrc {}",
                        ssrc
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
}
