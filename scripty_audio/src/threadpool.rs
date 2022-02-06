use once_cell::sync::OnceCell;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError as TokioRecvError;

static THREADPOOL_SUBMIT: OnceCell<mpsc::SyncSender<Pin<Box<dyn FnOnce() + Send + Sync>>>> =
    OnceCell::new();
static COMPLETED_JOBS: AtomicU64 = AtomicU64::new(0);

pub fn init_threadpool() {
    let pool = threadpool::Builder::new()
        .num_threads(num_cpus::get() / 2)
        .build();
    let (tx, rx) = mpsc::sync_channel(usize::MAX);
    THREADPOOL_SUBMIT.set(tx);

    std::thread::spawn(move || match rx.recv() {
        Ok(rx) => pool.execute(|| {
            rx();
            COMPLETED_JOBS.fetch_add(1, Ordering::Relaxed);
        }),
        Err(_) => return,
    });
}

pub fn submit_job<T: 'static + Send + Sync>(
    f: Box<dyn FnOnce(oneshot::Sender<T>) + Send + Sync>,
) -> Result<T, TokioRecvError> {
    let (tx, rx) = oneshot::channel();

    THREADPOOL_SUBMIT
        .get()
        .expect("failed to fetch threadpool submitter")
        .send(Box::pin(|| f(tx)));

    rx.blocking_recv()
}

pub async fn submit_job_async<T: 'static + Send + Sync>(
    f: Box<dyn FnOnce(oneshot::Sender<T>) + Send + Sync>,
) -> Result<T, TokioRecvError> {
    let (tx, rx) = oneshot::channel();

    THREADPOOL_SUBMIT
        .get()
        .expect("failed to fetch threadpool submitter")
        .send(Box::pin(|| f(tx)));

    rx.await
}

#[inline]
pub fn get_completed_jobs() -> u64 {
    COMPLETED_JOBS.load(Ordering::Relaxed)
}
