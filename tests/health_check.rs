//! tests/health_check.rs

use std::net::{SocketAddr, TcpListener};
// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
// Launch our application in the background ~somehow~
async fn spawn_app() {
    // let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    // println!("listening on {}", addr);
    // zero2prod::run(&addr).await
    let listener = TcpListener::bind("127.0.0.1:8000".parse::<SocketAddr>().unwrap()).unwrap();
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(zero2prod::app().into_make_service())
            .await
            .unwrap()
    });
}
