use serenity::model::prelude::Ready;

pub async fn ready(ready: Ready) {
    let Ready {
        guilds,
        presences,
        user,
        ..
    } = ready;

    info!(
        "bot ready: logged in as {}, in {} guilds with {} members",
        user.tag(),
        guilds.len(),
        presences.len()
    )
}
