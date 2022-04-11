use sqlx::PgPool;
use std::net::SocketAddr;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration file.");

    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect with specified Postgresql database.");

    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));
    println!("listening on http://{}", addr);
    run(&addr, connection).await;
}
