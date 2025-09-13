use blackjack_rust::service::domain::BlackjackService;
use blackjack_rust::service::startup::run;

use std::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to port");
    let blackjack_service = Arc::new(Mutex::new(BlackjackService::new()));
    run(listener, blackjack_service)?.await?;
    Ok(())
}
