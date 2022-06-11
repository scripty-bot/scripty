use crate::{Context, Error};

/// A list of all the things that made Scripty possible.
#[poise::command(prefix_command, slash_command)]
pub async fn credits(ctx: Context<'_>) -> Result<(), Error> {
    let resolved_language =
        scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

    ctx.send(|resp| {
        resp.ephemeral(true).embed(|e| {
            e.title(format_message!(resolved_language, "credits-title"))
                .description(format_message!(resolved_language, "credits-description"))
                .field(
                    &format_message!(resolved_language, "credits-field1-title"),
                    &format_message!(resolved_language, "credits-field1-description"),
                    false,
                )
                .field(
                    &format_message!(resolved_language, "credits-field2-title"),
                    &format_message!(resolved_language, "credits-field2-description"),
                    false,
                )
                .field(
                    &format_message!(resolved_language, "credits-field3-title"),
                    &format_message!(resolved_language, "credits-field3-description"),
                    false,
                )
                .field(
                    &format_message!(resolved_language, "credits-field4-title"),
                    &format_message!(resolved_language, "credits-field4-description"),
                    false,
                )
                .footer(|f| f.text("<3"))
        })
    })
    .await?;
    Ok(())
}
