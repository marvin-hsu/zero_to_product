use axum::{routing::get, Router, http::StatusCode};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(handler))
    .route("/health_check", get(health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, world!"
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}