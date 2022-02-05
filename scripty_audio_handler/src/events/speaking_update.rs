use crate::types::{SsrcStreamMap, SsrcUserIdMap};
use serenity::builder::{CreateEmbed, ExecuteWebhook};
use serenity::client::Context;
use serenity::model::channel::Embed;
use serenity::model::webhook::Webhook;
use serenity::utils::Color;
use songbird::events::context_data::SpeakingUpdateData;
use std::sync::Arc;

pub async fn speaking_update(
    update: &SpeakingUpdateData,
    ctx: Context,
    webhook: Arc<Webhook>,
    ssrc_user_id_map: SsrcUserIdMap,
    ssrc_stream_map: SsrcStreamMap,
    verbose: bool,
) {
    let ssrc = update.ssrc;
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

        if verbose {
            let res = scripty_audio::run_stt_with_metadata(old_stream, 1).await;
            let mut webhook_execute = ExecuteWebhook::default();
            let to_send = match res {
                Ok(Ok(res)) => {
                    let f: dyn FnOnce(&mut CreateEmbed) -> &mut CreateEmbed =
                        if res.num_transcripts() != 0 {
                            // SAFETY: we have already checked len != 0, so there must be at least one item
                            let transcript = unsafe { res.transcripts().get_unchecked(0) };

                            |e| {
                                e.title(format!("Transcript 1/{}", res.num_transcripts()))
                                    .field("")
                                    .field()
                            }
                        };
                    webhook_execute.embeds(vec![Embed::fake(f)])
                }
                Ok(Err(e)) => webhook_execute.content(format!(
                    "internal error: running stt algorithm failed with error: {}",
                    e
                )),
                Err(_) => webhook_execute
                    .content("internal error: sender hung up (this is usually global and fatal)"),
            };
            webhook.execute(&ctx, false, |e| e.username())
        } else {
        }
    }
}
