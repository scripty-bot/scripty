use std::{collections::HashMap, fmt::Write, ops::Range};

use poise::CreateReply;
use scripty_bot_utils::globals::CLIENT_DATA;
use serenity::{all::ShardId, gateway::ChunkGuildFilter};

use crate::{Context, Error};

/// Check all guilds, looking through their member counts, and report the number of each type,
/// and also report any servers where the bot to user ratio is over the specified value bots for every user.
#[poise::command(prefix_command, hide_in_help, owners_only)]
pub async fn check_guilds(ctx: Context<'_>, specified_ratio: f64) -> Result<(), Error> {
	let m = ctx.say("this could take a bit").await?;

	let dctx = ctx.discord();

	let mut error_count: u32 = 0;

	// guilds should be categorized by the number of members in them
	// groups: 0-2, 2-5, 5-10, 10-25, 25-50, 50-100, 100-250, 250-500, 500-1000, 1000-2500, 2500-5000, 5000-10000, 10000+
	const GROUPS: [Range<u64>; 13] = [
		0..2,
		2..5,
		5..10,
		10..25,
		25..50,
		50..100,
		100..250,
		250..500,
		500..1000,
		1000..2500,
		2500..5000,
		5000..10000,
		10000..u64::MAX,
	];

	// map of bounds to the number of servers in that range
	let mut guild_counts: HashMap<(u64, u64), u64> = HashMap::new();
	// guilds that have a ratio of more than 1 bot for every user
	// fields of the tuple are, respectively, the guild name, the guild id, and the ratio (bot count / user count)
	let mut guild_warnings: Vec<(String, u64, f64)> = Vec::new();

	let shard_manager = CLIENT_DATA
		.get()
		.expect("client data not initialized")
		.shard_manager
		.clone();
	let shard_count = shard_manager.runners.lock().await.len() as u32;

	for guild in dctx.cache.guilds() {
		let g = match guild.to_guild_cached(dctx) {
			Some(g) => g,
			None => {
				error_count += 1;
				continue;
			}
		};

		let member_count = g.member_count;

		// chunk guild if necessary
		if (member_count as usize) < g.members.len() {
			debug!(?g.id, "chunking guild");
			let guild_id = g.id;
			tokio::spawn(async move {
				// calculate the shard this guild is on
				let shard_id = serenity::utils::shard_id(guild_id, shard_count);

				let shard_manager = CLIENT_DATA
					.get()
					.expect("client data not initialized")
					.shard_manager
					.clone();

				let runner_guard = shard_manager.runners.lock().await;
				runner_guard
					.get(&ShardId(shard_id))
					.expect("shard should exist")
					.runner_tx
					.chunk_guild(guild_id, None, false, ChunkGuildFilter::None, None);
			});
		}

		for group in GROUPS {
			if group.contains(&member_count) {
				guild_counts
					.entry((group.start, group.end))
					.and_modify(|e| *e += 1)
					.or_insert(1);
			}
		}

		// skip checking if there are fewer or equal to 2 members in the guild (in that case, it's just the owner and the bot)
		if member_count <= 2 {
			continue;
		}

		let mut user_count = 0;
		let mut bot_count = 0;
		for member in g.members.values() {
			if member.user.bot {
				bot_count += 1;
			} else {
				user_count += 1;
			}
		}
		// if either bot or user count is 0, it's probably a caching issue, so skip it
		if bot_count == 0 || user_count == 0 {
			debug!(?g.id, "skipping guild due to none of an item found (bots: {}, users: {})", bot_count, user_count);
			continue;
		}
		let ratio = bot_count as f64 / user_count as f64;
		if ratio > specified_ratio {
			guild_warnings.push((g.name.clone(), g.id.0.get(), ratio));
		}
	}

	// format the guild counts into a string
	let mut response = String::new();
	for group in GROUPS {
		let count = guild_counts.get(&(group.start, group.end)).unwrap_or(&0);
		// subtract 1 from the end because these ranges are exclusive
		writeln!(
			&mut response,
			"{}-{} members: {} guilds",
			group.start,
			group.end - 1,
			count
		)
		.expect("failed to write to string");
	}
	// add the error count to the string
	write!(
		&mut response,
		"{} guilds were unable to be fetched",
		error_count
	)
	.expect("failed to write to string");

	// add spacing
	response.push_str("\n\n");

	// add the guild warnings to the string
	if !guild_warnings.is_empty() {
		response.push_str("Guilds with a ratio of more than 1 bot for every user:\n");
		// sort the guild warnings by ratio, descending
		// we can unwrap the comparison because NaN is impossible in this case, as zero bots or zero users is impossible
		guild_warnings.sort_by(|a, b| b.2.partial_cmp(&a.2).expect("ratio is NaN"));

		let mut count = guild_warnings.len();
		// truncate at 2,000 total characters in the response
		for (name, id, ratio) in &guild_warnings {
			writeln!(&mut response, "{} ({}): {}", name, id, ratio)
				.expect("failed to write to string");
			count -= 1;

			// add a small buffer to the total character count to account for the final message
			if response.len() > 1960 {
				write!(&mut response, "\n{} guilds were truncated", count)
					.expect("failed to write to string");
				break;
			}
		}
	}

	// send the response
	m.edit(ctx, CreateReply::default().content(response))
		.await?;

	Ok(())
}
