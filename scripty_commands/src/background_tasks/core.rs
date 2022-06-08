use once_cell::sync::OnceCell;
use serenity::client::Context;

#[async_trait]
pub trait BackgroundTask {
    async fn init(ctx: Context) -> Result<Self, crate::Error>;

    /// How often the task should be run. This gets called once at the start, and then after every call to `run()`.
    fn interval(&mut self) -> std::time::Duration;

    /// Run the background task.
    ///
    /// This gets called every `interval()`.
    async fn run(&mut self);
}

/// Initialize a task. Accepts one argument, the full path to the task struct from the crate root.
/// Spawns the background task required.
macro_rules! init_task {
    ($path: path, $ctx: expr) => {{
        use $path;
        tokio::spawn(async move {
            let task = $path::init($ctx).await;
            let mut interval = task.interval();
            loop {
                task.run().await;
                interval = task.interval();
                tokio::time::sleep(interval).await;
            }
        });
    }};
}

pub fn init_background_tasks(ctx: Context) {
    init_task!(crate::background_tasks::tasks::latency_update::LatencyUpdater);
    init_task!(crate::background_tasks::tasks::basic_stats_update::BasicStatsUpdater);
}
