use crate::models::MODELS;
use coqui_stt::{Metadata, Stream};
use tokio::sync::oneshot::error::RecvError;

pub fn get_stream(lang: &str) -> Option<Stream> {
    MODELS
        .get()
        .expect("models should've been initialized before attempting to get a stream")
        .get(lang)
        .and_then(|x| x.try_clone().ok())
}

#[inline]
pub async fn run_stt_with_metadata(
    stream: Stream,
    num_results: u32,
) -> Result<coqui_stt::Result<Metadata>, RecvError> {
    crate::threadpool::submit_job_async(Box::new(move |rx| {
        let res = stream.finish_stream_with_metadata(num_results);
        rx.send(res).unwrap_or_else(|_| panic!("sender hung up"));
    }))
    .await
}

#[inline]
pub async fn run_stt(stream: Stream) -> Result<coqui_stt::Result<String>, RecvError> {
    crate::threadpool::submit_job_async(Box::new(move |rx| {
        let res = stream.finish_stream();
        rx.send(res).expect("sender hung up");
    }))
    .await
}
