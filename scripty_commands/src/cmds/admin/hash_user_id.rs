use std::num::NonZeroU64;

use crate::{Context, Error};

#[poise::command(prefix_command, hide_in_help, owners_only)]
pub async fn hash_user_id(ctx: Context<'_>, uid: u64) -> Result<(), Error> {
	let uid = NonZeroU64::new(uid)
		.ok_or_else(|| Error::custom("user id must be non-zero".to_string()))?;
	let hashed_uid = hex::encode(scripty_utils::hash_user_id(uid));
	ctx.say(hashed_uid).await?;
	Ok(())
}
