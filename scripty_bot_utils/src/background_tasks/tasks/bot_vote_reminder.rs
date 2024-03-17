use std::{fmt, time::Duration};

use serenity::{
	all::UserId,
	builder::{CreateEmbed, CreateMessage},
	client::Context as SerenityContext,
};

use crate::{background_tasks::core::BackgroundTask, Error};

/// Sends vote reminders to users every minute.
pub struct VoteReminderTask {
	ctx: SerenityContext,
}

impl BackgroundTask for VoteReminderTask {
	async fn init(ctx: SerenityContext) -> Result<Self, Error> {
		Ok(Self { ctx })
	}

	fn interval(&mut self) -> Duration {
		Duration::from_secs(60)
	}

	async fn run(&mut self) {
		let vote_query = match sqlx::query!(
			"DELETE FROM vote_reminders WHERE next_reminder < NOW() RETURNING user_id, site_id, \
			 next_reminder"
		)
		.fetch_all(scripty_db::get_db())
		.await
		{
			Ok(vote_query) => vote_query,
			Err(e) => {
				error!("failed to get vote reminders: {}", e);
				return;
			}
		};

		for user in vote_query {
			let site: VoteList = user.site_id.into();
			let user_id = user.user_id as u64;
			let reminder_unix_ts = user.next_reminder.assume_utc().unix_timestamp();

			let msg =
				CreateMessage::new().embed(CreateEmbed::new().title("Vote reminder").description(
					format!(
						"You can vote for Scripty on {} again, as of <t:{}:R>. You can do so at \
						 {}. Thanks for your support!",
						site,
						reminder_unix_ts,
						site.vote_url()
					),
				));
			let ctx2 = self.ctx.clone();
			tokio::spawn(async move {
				let res = match UserId::new(user_id).create_dm_channel(&ctx2.http).await {
					Ok(channel) => channel.send_message(&ctx2.http, msg).await.map(|_| ()),
					Err(e) => Err(e),
				};
				if let Err(e) = res {
					error!("failed to send vote reminder: {}", e);
				}
			});
		}
	}

	fn timeout(&mut self) -> Option<Duration> {
		Some(Duration::from_secs(5))
	}
}

pub enum VoteList {
	TopGg              = 1,
	DiscordServicesNet = 2,
	WumpusStore        = 3,
}
impl From<i16> for VoteList {
	fn from(i: i16) -> Self {
		match i {
			1 => Self::TopGg,
			2 => Self::DiscordServicesNet,
			3 => Self::WumpusStore,
			_ => panic!("invalid vote list id"),
		}
	}
}

impl fmt::Display for VoteList {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::TopGg => write!(f, "top.gg"),
			Self::DiscordServicesNet => write!(f, "discordservices.net"),
			Self::WumpusStore => write!(f, "wumpus.store"),
		}
	}
}

impl VoteList {
	pub fn vote_url(&self) -> &'static str {
		match self {
			Self::TopGg => "https://top.gg/bot/811652199100317726/vote",
			Self::DiscordServicesNet => "https://discordservices.net/bot/scripty",
			Self::WumpusStore => "https://wumpus.store/bot/811652199100317726/vote",
		}
	}
}
