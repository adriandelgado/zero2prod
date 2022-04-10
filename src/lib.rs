use axum::{
    extract::Form,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(Form(_input): Form<FormData>) -> StatusCode {
    StatusCode::OK
}

pub async fn run(addr: &SocketAddr) {
    axum::Server::bind(addr)
        .serve(app().into_make_service())
        .await
        .unwrap()
}
