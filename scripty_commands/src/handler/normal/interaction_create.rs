use serenity::client::Context;
use serenity::model::application::interaction::Interaction;

pub async fn interaction_create(_ctx: Context, interaction: Interaction) {
    if let Some(cmd) = interaction.application_command() {
        info!("got data {:?}", cmd.data);
    }
}
