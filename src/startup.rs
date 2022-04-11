use crate::routes::{health_check, subscribe};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use sqlx::PgPool;
use std::net::SocketAddr;

pub async fn run(addr: &SocketAddr, connection: PgPool) {
    let app = app(connection);

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[must_use]
pub fn app(connection: PgPool) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(Extension(connection))
}
