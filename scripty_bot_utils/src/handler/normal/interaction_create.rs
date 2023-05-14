use serenity::{all::Interaction, client::Context};

pub async fn interaction_create(_ctx: Context, interaction: Interaction) {
	if let Some(cmd) = interaction.command() {
		info!("got data {:?}", cmd.data);
	}
}
