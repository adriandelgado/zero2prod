use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::SocketAddr;
use zero2prod::{configuration::get_configuration, startup::run, telemetry};

#[tokio::main]
async fn main() {
    let tracing_subscriber =
        telemetry::get_tracing_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_tracing_subscriber(tracing_subscriber);

    let configuration = get_configuration().expect("Failed to read configuration file.");
    let connection = PgPool::connect(configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect with specified Postgresql database.");

    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));
    tracing::info!("listening on {addr}");
    run(&addr, connection).await;
}
