use axum::{http::StatusCode, routing::get, Router};
use std::net::SocketAddr;

pub fn app() -> Router {
    Router::new().route("/health_check", get(health_check))
}

pub async fn run(addr: &SocketAddr) {
    axum::Server::bind(addr)
        .serve(app().into_make_service())
        .await
        .unwrap()
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
