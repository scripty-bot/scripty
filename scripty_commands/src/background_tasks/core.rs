use serenity::client::Context;

#[async_trait]
pub trait BackgroundTask: Sized {
    async fn init(ctx: Context) -> Result<Self, crate::Error>;

    /// How often the task should be run. This gets called after every call to `run()`.
    fn interval(&mut self) -> std::time::Duration;

    /// Run the background task.
    ///
    /// This gets called every `interval()`.
    async fn run(&mut self);
}

/// Initialize a task. Accepts one argument, the full path to the task struct from the crate root.
/// Spawns the background task required.
macro_rules! init_task {
    ($path: ty, $ctx: expr) => {{
        let ctx = $ctx.clone();
        tokio::spawn(async move {
            let mut task = match <$path>::init(ctx).await {
                Ok(t) => t,
                Err(e) => {
                    error!("background task failed to initialize: {:?}", e);
                    return;
                }
            };
            let mut interval;
            loop {
                task.run().await;
                interval = task.interval();
                tokio::time::sleep(interval).await;
            }
        });
    }};
}

pub fn init_background_tasks(ctx: Context) {
    init_task!(crate::background_tasks::tasks::LatencyUpdater, ctx);
    init_task!(crate::background_tasks::tasks::BasicStatsUpdater, ctx);
}
