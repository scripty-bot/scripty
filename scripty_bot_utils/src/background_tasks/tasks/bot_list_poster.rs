use std::{sync::Arc, time::Duration};

use reqwest::Client;
use scripty_botlists::{
	botlist_me::BotListMe,
	discord_bots_gg::DiscordBotsGG,
	discordbotlist_com::DiscordBotListCom,
	discordextremelist_xyz::DiscordExtremeListXyz,
	discords_com::DiscordsCom,
	discordservices_net::DiscordServicesNet,
	disforge_com::DisforgeCom,
	infinitybots_gg::InfinityBotsGG,
	top_gg::TopGG,
	voidbots_net::VoidBotsNet,
	PostStats,
	StatPoster,
};
use scripty_config::BotListsConfig;
use scripty_utils::get_thirdparty_http;
use serenity::client::Context;

use crate::{background_tasks::core::BackgroundTask, Error};

pub struct BotListUpdater {
	ctx:       Context,
	bot_lists: Arc<Vec<BotLists>>,
}

#[async_trait]
impl BackgroundTask for BotListUpdater {
	async fn init(ctx: Context) -> Result<Self, Error> {
		let mut bot_lists = vec![];
		let bot_id = ctx.cache.current_user().id.get();

		add_bot_lists(&mut bot_lists, bot_id);

		Ok(Self {
			ctx,
			bot_lists: Arc::new(bot_lists),
		})
	}

	fn interval(&mut self) -> Duration {
		Duration::from_secs(60 * 60) // 1 hour
	}

	async fn run(&mut self) {
		let stats = PostStats {
			server_count: self.ctx.cache.guild_count(),
			shard_count:  self.ctx.cache.shard_count().get(),
		};
		let client = get_thirdparty_http();

		for list in self.bot_lists.iter() {
			if let Err(e) = list.post_stats(&client, stats).await {
				error!("Failed to post stats to bot list: {}", e);
			}
		}
	}
}

enum BotLists {
	BotListMe(BotListMe),
	DiscordBotsGG(DiscordBotsGG),
	DiscordBotListCom(DiscordBotListCom),
	DiscordExtremeListXyz(DiscordExtremeListXyz),
	DiscordsCom(DiscordsCom),
	DiscordServicesNet(DiscordServicesNet),
	DisforgeCom(DisforgeCom),
	InfinityBotsGG(InfinityBotsGG),
	TopGG(TopGG),
	VoidBotsNet(VoidBotsNet),
}

#[async_trait]
impl StatPoster for BotLists {
	async fn post_stats(
		&self,
		client: &Client,
		stats: PostStats,
	) -> Result<bool, scripty_botlists::Error> {
		match self {
			BotLists::BotListMe(item) => item.post_stats(client, stats).await,
			BotLists::DiscordBotsGG(item) => item.post_stats(client, stats).await,
			BotLists::DiscordBotListCom(item) => item.post_stats(client, stats).await,
			BotLists::DiscordExtremeListXyz(item) => item.post_stats(client, stats).await,
			BotLists::DiscordsCom(item) => item.post_stats(client, stats).await,
			BotLists::DiscordServicesNet(item) => item.post_stats(client, stats).await,
			BotLists::DisforgeCom(item) => item.post_stats(client, stats).await,
			BotLists::InfinityBotsGG(item) => item.post_stats(client, stats).await,
			BotLists::TopGG(item) => item.post_stats(client, stats).await,
			BotLists::VoidBotsNet(item) => item.post_stats(client, stats).await,
		}
	}
}

fn add_bot_lists(bot_lists: &mut Vec<BotLists>, bot_id: u64) {
	let cfg = scripty_config::get_config();
	let bot_list_cfg = &cfg.bot_lists;

	if let Some(BotListsConfig::TokenOnly(token)) = bot_list_cfg.get("botlist_me") {
		bot_lists.push(BotLists::BotListMe(BotListMe::new(token.clone(), bot_id)));
	}

	if let Some(BotListsConfig::TokenOnly(token)) = bot_list_cfg.get("discord_bots_gg") {
		bot_lists.push(BotLists::DiscordBotsGG(DiscordBotsGG::new(
			token.clone(),
			bot_id,
		)));
	}

	if let Some(BotListsConfig::TokenOnly(token)) = bot_list_cfg.get("discordbotlist_com") {
		bot_lists.push(BotLists::DiscordBotListCom(DiscordBotListCom::new(
			token.clone(),
			bot_id,
		)));
	}

	if let Some(BotListsConfig::TokenOnly(token)) = bot_list_cfg.get("discordextremelist_xyz") {
		bot_lists.push(BotLists::DiscordExtremeListXyz(DiscordExtremeListXyz::new(
			token.clone(),
			bot_id,
		)));
	}

	if let Some(BotListsConfig::TokenOnly(token)) = bot_list_cfg.get("discords_com") {
		bot_lists.push(BotLists::DiscordsCom(DiscordsCom::new(
			token.clone(),
			bot_id,
		)));
	}

	if let Some(BotListsConfig::FullConfig { token, .. }) = bot_list_cfg.get("discordservices_net")
	{
		bot_lists.push(BotLists::DiscordServicesNet(DiscordServicesNet::new(
			token.clone(),
			bot_id,
		)));
	}

	if let Some(BotListsConfig::TokenOnly(token)) = bot_list_cfg.get("disforge_com") {
		bot_lists.push(BotLists::DisforgeCom(DisforgeCom::new(
			token.clone(),
			bot_id,
		)));
	}

	if let Some(BotListsConfig::TokenOnly(token)) = bot_list_cfg.get("infinitybots_gg") {
		bot_lists.push(BotLists::InfinityBotsGG(InfinityBotsGG::new(
			token.clone(),
			bot_id,
		)));
	}

	if let Some(BotListsConfig::FullConfig { token, .. }) = bot_list_cfg.get("top_gg") {
		bot_lists.push(BotLists::TopGG(TopGG::new(token.clone(), bot_id)));
	}

	if let Some(BotListsConfig::TokenOnly(token)) = bot_list_cfg.get("voidbots_net") {
		bot_lists.push(BotLists::VoidBotsNet(VoidBotsNet::new(
			token.clone(),
			bot_id,
		)));
	}
}
