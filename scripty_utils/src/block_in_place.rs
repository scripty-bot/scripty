use once_cell::sync::OnceCell;
use tokio::sync::Semaphore;

static RT_THREAD_SEMAPHORE: OnceCell<Semaphore> = OnceCell::new();

pub async fn block_in_place<F, T>(f: F) -> T
where
	F: FnOnce() -> T,
{
	let semaphore = RT_THREAD_SEMAPHORE.get_or_init(|| {
		let permits = ((num_cpus::get() as f64) * 0.75).floor().max(1.0) as usize;
		debug!("Creating semaphore with {} permits", permits);
		Semaphore::new(permits)
	});

	debug!("waiting to acquire semaphore");
	let acq = semaphore.acquire().await.expect("semaphore closed?");
	debug!("acquired semaphore");
	let ret = tokio::task::block_in_place(f);
	// yes yes i know this isn't required but it's more explicit
	drop(acq);
	ret
}
