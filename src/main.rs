use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::SocketAddr;
use zero2prod::{configuration::get_configuration, startup::run, telemetry};

#[tokio::main]
async fn main() {
    let tracing_subscriber =
        telemetry::get_tracing_subscriber("zero2prod".into(), "debug".into(), std::io::stdout);

    tracing::subscriber::set_global_default(tracing_subscriber)
        .expect("Failed to assign global tracing subscriber.");

    let configuration = get_configuration().expect("Failed to read configuration file.");
    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect with specified Postgresql database.");

    // Here we choose to bind explicitly to localhost, 127.0.0.1, for security
    // reasons. This binding may cause issues in some environments. For example,
    // it causes connectivity issues running in WSL2, where you cannot reach the
    // server when it is bound to WSL2's localhost interface. As a workaround,
    // you can choose to bind to all interfaces, 0.0.0.0, instead, but be aware
    // of the security implications when you expose the server on all interfaces.
    let address = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));
    tracing::info!("listening on {address}");
    run(&address, connection_pool).await;
}
