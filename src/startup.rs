use crate::routes::{health_check, subscribe};
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

pub async fn run(addr: &SocketAddr) {
    axum::Server::bind(addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}

#[must_use]
pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}
