use serenity::client::Context;
use serenity::model::guild::Member;

pub async fn guild_member_update(ctx: Context, _: Option<Member>, new: Member) {
    // check that we're looking only at ourself
    if new.user.id != ctx.cache.current_user().id {
        return;
    }

    // check if we're connected to voice in this guild
    if !scripty_audio_handler::check_voice_state(&ctx, new.guild_id.0.into()).await {
        return;
    }

    // check if our nickname has been changed from "[TRANSCRIBING] Scripty" to something else
    // if it has, reset it to "[TRANSCRIBING] Scripty"
    // if we can't reset it, leave the voice channel
    // if we can't leave the voice channel, log an error
    if new.nick != Some("[TRANSCRIBING] Scripty".to_string()) {
        if let Err(e) = new
            .guild_id
            .edit_nickname(&ctx, Some("[TRANSCRIBING] Scripty"))
            .await
        {
            error!("failed to reset nickname: {}", e);
            if let Err(e) =
                scripty_audio_handler::disconnect_from_vc(&ctx, new.guild_id.0.into()).await
            {
                error!("failed to leave voice channel: {:?}", e);
            }
        }
    }
}
