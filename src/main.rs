use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use zero2prod::{configuration::get_configuration, startup::run, telemetry};

#[tokio::main]
async fn main() {
    let tracing_subscriber =
        telemetry::get_tracing_subscriber("zero2prod".into(), "debug".into(), std::io::stdout);

    tracing::subscriber::set_global_default(tracing_subscriber)
        .expect("Failed to assign global tracing subscriber.");

    let configuration = get_configuration().expect("Failed to read configuration file.");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let address = SocketAddr::from((
        configuration.application.host,
        configuration.application.port,
    ));
    tracing::info!("listening on {address}");
    run(&address, connection_pool).await;
}
