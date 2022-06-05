use coqui_stt::{Error, Metadata, Model, Stream as StreamingState};
use once_cell::sync::OnceCell;
use std::sync::Arc;

pub enum FinishType {
    WithMetadata(coqui_stt::Result<Metadata>),
    WithoutMetadata(coqui_stt::Result<String>),
    Error,
}

enum Command {
    FinishWithMetadata(u32),
    FinishWithoutMetadata,
    FeedAudio(Vec<i16>),
}

pub struct Stream {
    finish_rx: flume::Receiver<FinishType>,
    command_tx: flume::Sender<Command>,
}

static STT_THREAD_LOCK: OnceCell<std_semaphore::Semaphore> = OnceCell::new();

impl Stream {
    pub fn init(model: Arc<Model>) -> Result<Self, Error> {
        let (start_tx, start_rx) = flume::bounded(0);
        let (command_tx, command_rx) = flume::unbounded();
        // capacity 0 means the background thread will immediately join after completing the send
        let (finish_tx, finish_rx) = flume::bounded(0);

        std::thread::spawn(move || {
            debug!("spawned stt thread, now creating stream");
            let state = StreamingState::from_model(model);
            let mut state = match state {
                Ok(s) => {
                    debug!("created stream, now entering main loop");
                    start_tx.send(Ok(())).expect("failed to send ok to init");
                    s
                }
                Err(e) => {
                    debug!("failed to create streaming state: {:?}", e);
                    start_tx.send(Err(e)).expect("failed to send error to init");
                    return;
                }
            };

            while let Ok(cmd) = command_rx.recv() {
                debug!("got command, waiting for new action permission");
                let _guard = STT_THREAD_LOCK
                    .get_or_init(|| {
                        std_semaphore::Semaphore::new(
                            ((num_cpus::get() as f64) * 0.75).floor().max(1.0) as isize,
                        )
                    })
                    .access();
                debug!("got permission, executing command");
                match cmd {
                    Command::FeedAudio(audio) => {
                        debug!("feeding audio");
                        state.feed_audio(&audio);
                        debug!("audio fed");
                    }
                    Command::FinishWithMetadata(num_results) => {
                        debug!("finishing with metadata");
                        finish_tx
                            .send(FinishType::WithMetadata(
                                state.finish_stream_with_metadata(num_results),
                            ))
                            .expect("failed to send finish");
                        debug!("finished with metadata");
                        return;
                    }
                    Command::FinishWithoutMetadata => {
                        debug!("finishing without metadata");
                        finish_tx
                            .send(FinishType::WithoutMetadata(state.finish_stream()))
                            .expect("failed to send finish");
                        debug!("finished without metadata");
                        return;
                    }
                }
            }
        });
        start_rx.recv().expect("failed to receive from init")?;

        Ok(Self {
            finish_rx,
            command_tx,
        })
    }

    pub fn feed_audio(&self, audio: Vec<i16>) {
        self.command_tx
            .send(Command::FeedAudio(audio))
            .expect("failed to send audio to stt thread");
    }

    pub async fn finish_stream(self) -> coqui_stt::Result<String> {
        self.command_tx
            .send(Command::FinishWithoutMetadata)
            .expect("failed to send finish to stt thread");
        match self
            .finish_rx
            .recv_async()
            .await
            .expect("failed to join stt thread")
        {
            FinishType::WithoutMetadata(s) => s,
            _ => unreachable!("invalid finish type"),
        }
    }

    pub async fn finish_stream_with_metadata(
        self,
        num_results: u32,
    ) -> coqui_stt::Result<Metadata> {
        self.command_tx
            .send(Command::FinishWithMetadata(num_results))
            .expect("failed to send finish to stt thread");
        match self
            .finish_rx
            .recv_async()
            .await
            .expect("failed to join stt thread")
        {
            FinishType::WithMetadata(s) => s,
            _ => unreachable!("invalid finish type"),
        }
    }
}
