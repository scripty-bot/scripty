use crate::{Context, Error};
use poise::CreateReply;
use serenity::builder::CreateEmbed;

const DONATION_DESCRIPTION: &str =
    "Scripty is not cheap to run. It is currently running on a $1,500 USD server with an \
    AMD Ryzen 9 3900 CPU and 128GB RAM. Even with that hardware, we estimate it can only handle \
    100 concurrent transcriptions. Donations would allow us to scale our hardware capacity and \
    handle many more concurrent transcriptions, perhaps tens of thousands someday with enough \
    donations.\n\n\
    Training a model is not easy either, as that needs relatively recent (CUDA 10.1 support) \
    Nvidia GPUs. We hate asking for donations, but we absolutely can't support the bot out of our \
    own pockets, since it's just too expensive. So we're asking for help, and giving rewards in the \
    form of premium subscriptions.\n\n\
    You can view more info at https://scripty.org/premium, but the gist of it is that there are 6 \
    tiers ranging in price from $5 USD to $100 USD per month. The $100 tier comes with its own \
    managed instance of Scripty for your own server, with a custom name, and profile picture.\n\n\
    We also support one-time donations directly through GitHub Sponsors: \
    [https://github.com/sponsors/tazz4843]\
    (https://github.com/sponsors/tazz4843?frequency=one-time&sponsor=tazz4843)\n\n\n\
    https://scripty.org/premium";

/// Get information on donating to support Scripty.
#[poise::command(prefix_command, slash_command)]
pub async fn donate(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(
        CreateReply::default().ephemeral(true).embed(
            CreateEmbed::default()
                .title("Donations")
                .description(DONATION_DESCRIPTION),
        ),
    )
    .await?;
    Ok(())
}
