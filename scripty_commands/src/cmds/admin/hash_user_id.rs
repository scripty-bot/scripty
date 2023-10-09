use crate::{Context, Error};

#[poise::command(prefix_command, hide_in_help, owners_only)]
pub async fn hash_user_id(ctx: Context<'_>, uid: u64) -> Result<(), Error> {
	ctx.say(hex::encode(scripty_utils::hash_user_id(uid)))
		.await?;
	Ok(())
}
