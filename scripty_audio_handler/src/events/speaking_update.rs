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
            let res = old_stream.finish_stream_with_metadata_async(3).await;
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
            let res = old_stream.finish_stream_async().await;
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
}
