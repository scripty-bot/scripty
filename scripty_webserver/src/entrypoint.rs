pub async fn entrypoint() {
    let router = crate::router::router();

    axum::Server::bind(&"127.0.0.1:42069".parse().expect("invalid bind address"))
        .serve(router.into_make_service())
        .await
        .expect("failed to start server");
}
