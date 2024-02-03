use poise::CreateReply;
use scripty_bot_utils::{Context, Error};
use serenity::{
	all::{Webhook, WebhookId},
	builder::ExecuteWebhook,
};

#[poise::command(prefix_command, hide_in_help, owners_only)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
	let msg = ctx.say("shutting down").await?;
	let cfg = scripty_config::get_config();

	let webhook_builder = ExecuteWebhook::new().content(format!(
		"Scripty is shutting down for maintenance. If you have any questions, please feel free to \
		 join our support server at {}.",
		cfg.support_invite
	));

	// iterate over all active voice connections and notify
	let songbird = scripty_audio_handler::get_songbird();
	let calls = songbird.iter().map(|x| x.0).collect::<Vec<_>>();
	msg.edit(
		ctx,
		CreateReply::new().content(format!(
			"shutting down {} voice connections\ndone: 0 (0%), succeeded 0, failed 0",
			calls.len()
		)),
	)
	.await?;
	let mut success = 0;
	let mut error = 0;
	let mut to_do = calls.len();
	let total = calls.len();

	for guild_id in calls {
		let f = async {
			// leave the call
			songbird.remove(guild_id).await?;

			// notify the guild

			// need to fetch from redis
			let webhook_id = scripty_redis::run_transaction::<u64>("GETDEL", |f| {
				f.arg(format!("voice:{{{}}}:webhook_id", guild_id));
			})
			.await
			.map(WebhookId::new)?;
			let webhook_token = scripty_redis::run_transaction::<String>("GETDEL", |f| {
				f.arg(format!("voice:{{{}}}:webhook_token", guild_id));
			})
			.await?;

			let hook = Webhook::from_id_with_token(ctx.http(), webhook_id, &webhook_token).await?;
			hook.execute(&ctx.serenity_context().http, false, webhook_builder.clone())
				.await?;

			Result::<(), Error>::Ok(())
		};

		match f.await {
			Ok(()) => success += 1,
			Err(e) => {
				error += 1;
				error!("error while shutting down voice connection: {}", e);
			}
		}
		to_do -= 1;

		msg.edit(
			ctx,
			CreateReply::new().content(format!(
				"shutting down {} voice connections\ndone: {} ({}%), succeeded {}, failed {}",
				total,
				total - to_do,
				(total - to_do) * 100 / total,
				success,
				error
			)),
		)
		.await?;
	}

	msg.edit(
		ctx,
		CreateReply::new().content("done shutting down voice connections, shutting down bot now"),
	)
	.await?;
	ctx.data()
		.shard_manager
		.get()
		.ok_or_else(|| Error::custom("shard manager not found".to_string()))?
		.shutdown_all()
		.await;

	Ok(())
}
