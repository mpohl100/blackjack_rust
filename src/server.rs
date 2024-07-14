use blackjack_rust::service::startup::run;
use blackjack_rust::service::domain::BlackjackService;

use std::net::TcpListener;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to port");
    let blackjack_service = Arc::new(BlackjackService::new());
    run(listener, blackjack_service)?.await?;
    Ok(())
}
