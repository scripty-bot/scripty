use once_cell::sync::OnceCell;
use serenity::client::Context;

pub trait BackgroundTask: Sized {
	async fn init(ctx: Context) -> Result<Self, crate::Error>;

	/// How often the task should be run. This gets called after every call to `run()`.
	fn interval(&mut self) -> std::time::Duration;

	/// Run the background task.
	///
	/// This gets called every `interval()`.
	async fn run(&mut self);

	/// Timeout for the task.
	/// If this returns `None`, the task will never time out.
	///
	/// This gets called just before every call to `run()`.
	/// If the task times out,
	/// its future will be dropped, and after `interval()` has passed, it will be rerun.
	fn timeout(&mut self) -> Option<std::time::Duration> {
		None
	}
}

/// Initialize a task. Accepts one argument, the full path to the task struct from the crate root.
/// Spawns the background task required.
macro_rules! init_task {
	($path: ty, $ctx: expr) => {{
		let ctx = $ctx.clone();
		tokio::spawn(async move {
			let mut task = match <$path as BackgroundTask>::init(ctx).await {
				Ok(t) => t,
				Err(e) => {
					error!("background task failed to initialize: {:?}", e);
					return;
				}
			};
			let mut interval;
			loop {
				match task.timeout() {
					Some(timeout) => {
						if tokio::time::timeout(timeout, task.run()).await.is_err() {
							error!(concat!("background task timed out: ", stringify!($path)));
						}
					}
					None => task.run().await,
				}
				interval = task.interval();
				tokio::time::sleep(interval).await;
			}
		});
	}};
}

static READY: OnceCell<()> = OnceCell::new();

pub fn init_background_tasks(ctx: Context) {
	if READY.set(()).is_err() {
		return;
	}

	init_task!(crate::background_tasks::tasks::LatencyUpdater, ctx);
	init_task!(crate::background_tasks::tasks::BasicStatsUpdater, ctx);
	init_task!(crate::background_tasks::tasks::StatusUpdater, ctx);
	init_task!(crate::background_tasks::tasks::CommandLatencyClearer, ctx);
	init_task!(crate::background_tasks::tasks::BotListUpdater, ctx);
	init_task!(crate::background_tasks::tasks::VoteReminderTask, ctx);
}
