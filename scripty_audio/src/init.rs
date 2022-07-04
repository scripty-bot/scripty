use tokio::io::AsyncWriteExt;

pub async fn init_stt() {
    let mut sock = tokio::net::UnixStream::connect("/tmp/stts.sock")
        .await
        .expect("failed to connect to stts");
    let _ = sock.write_u8(0x03).await;
}
