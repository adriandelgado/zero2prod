use crate::routes::{health_check, subscribe};
use axum::{
    http::Request,
    routing::{get, post},
    Extension, Router,
};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::{span, Level};
use uuid::Uuid;

pub async fn run(addr: &SocketAddr, db_pool: PgPool) {
    let app = app(db_pool);

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
        .layer(TraceLayer::new_for_http().make_span_with(
            |request: &Request<_>| span!(Level::DEBUG, "REQUEST", request_id = %Uuid::new_v4(), uri = %request.uri(), method = %request.method(), version = ?request.version()),
        ))
}
