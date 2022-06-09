use poise::BoxFuture;

async fn _post_command(ctx: crate::Context<'_>) {
    let metrics = scripty_metrics::get_metrics();

    // log command name invoked
    let command = ctx.command();
    let command_root = command.category.unwrap_or(command.name);

    match command_root {
        "credits" => metrics.commands.credits.inc(),
        "data_storage" => metrics.commands.data_storage.inc(),
        "donate" => metrics.commands.donate.inc(),
        "block" => metrics.commands.block.inc(),
        "help" => metrics.commands.help.inc(),
        "join" => metrics.commands.join.inc(),
        "language" => metrics.commands.language.inc(),
        "ping" => metrics.commands.ping.inc(),
        "register_cmds" => metrics.commands.register_cmds.inc(),
        "setup" => metrics.commands.setup.inc(),
        _ => metrics.commands.unknown.inc(),
    };
}

#[inline]
pub fn post_command(ctx: crate::Context<'_>) -> BoxFuture<()> {
    Box::pin(_post_command(ctx))
}
