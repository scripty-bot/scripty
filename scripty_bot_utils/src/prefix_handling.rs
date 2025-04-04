use std::{borrow::Cow, time::SystemTime};

use poise::PartialContext;
use scripty_data_type::Data;
use serenity::model::id::GuildId;

pub fn dynamic_prefix(
	ctx: PartialContext<Data, crate::Error>,
) -> poise::BoxFuture<'_, Result<Option<Cow<'static, str>>, crate::Error>> {
	Box::pin(_dynamic_prefix(ctx))
}

async fn _dynamic_prefix(
	ctx: PartialContext<'_, Data, crate::Error>,
) -> Result<Option<Cow<'static, str>>, crate::Error> {
	let Some(guild_id) = ctx.guild_id else {
		return Ok(Some(Cow::Borrowed(
			scripty_config::get_config().prefix.as_str(),
		)));
	};
	let _timings = RunTimings::new(guild_id);

	let maybe_prefix = scripty_redis::run_transaction::<Option<String>>("GET", |t| {
		t.arg(format!("prefix_{{{}}}", guild_id.get()));
	})
	.await?;
	if let Some(prefix) = maybe_prefix {
		return Ok(Some(Cow::Owned(prefix)));
	}

	let db = scripty_db::get_db();
	let maybe_prefix = sqlx::query!(
		"SELECT prefix FROM guilds WHERE guild_id = $1",
		guild_id.get() as i64
	)
	.fetch_optional(db)
	.await?
	.and_then(|row| row.prefix)
	.and_then(|prefix| {
		if prefix.len() > 8 {
			error!(%guild_id, "guild prefix too long! got {}, expected 8", prefix.len());
			None
		} else {
			Some(prefix)
		}
	})
	.unwrap_or_else(|| scripty_config::get_config().prefix.to_owned());

	scripty_redis::run_transaction::<()>("SETEX", |cmd| {
		cmd.arg(format!("prefix_{{{}}}", guild_id.get()))
			.arg(60 * 15)
			.arg(&maybe_prefix);
	})
	.await?;

	Ok(Some(Cow::Owned(maybe_prefix)))
}

struct RunTimings {
	start_time: SystemTime,
	guild_id:   GuildId,
}
impl RunTimings {
	fn new(guild_id: GuildId) -> Self {
		Self {
			start_time: SystemTime::now(),
			guild_id,
		}
	}
}
impl Drop for RunTimings {
	fn drop(&mut self) {
		debug!(
			guild_id = %self.guild_id,
			"took {:?} to query prefix for guild",
			self.start_time.elapsed().expect("clock rolled backwards")
		);
	}
}
