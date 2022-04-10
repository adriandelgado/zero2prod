use std::net::SocketAddr;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration file.");
    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));
    println!("listening on http://{}", addr);
    run(&addr).await;
}
