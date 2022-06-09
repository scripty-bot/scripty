use poise::BoxFuture;
use std::ops::DerefMut;
use std::time::Instant;

async fn _post_command(ctx: crate::Context<'_>) {
    let et = Instant::now();

    // log the time it took to execute the command
    let tt = match ctx.invocation_data::<Instant>().await {
        Some(st) => Some(et.duration_since(st).as_nanos()),
        None => {
            warn!("timestamp was not logged in pre_command");
            None
        }
    };

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

    // log the time it took to execute the command
    if let Some(tt) = tt {
        metrics.command_time.observe(tt as f64 / 1_000_000_000.0);
    }
}

#[inline]
pub fn post_command(ctx: crate::Context<'_>) -> BoxFuture<()> {
    Box::pin(_post_command(ctx))
}
