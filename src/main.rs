use std::net::SocketAddr;
use zero2prod::startup::run;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    run(&addr).await;
}
