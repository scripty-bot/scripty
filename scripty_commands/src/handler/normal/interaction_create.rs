use serenity::client::Context;
use serenity::model::application::interaction::Interaction;

pub async fn interaction_create(_ctx: Context, interaction: Interaction) {
    scripty_metrics::measure_start_latency(interaction.id().0);
}
