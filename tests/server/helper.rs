
use crate::service::domain::BlackjackService;
use reqwest::Client;
use std::net::TcpListener;

pub struct TestApp {
    pub addr: String,
    pub blackjack_service: BlackjackService,
    pub client: reqwest::Client,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port();

    let addr = format!("http://127.0.0.1:{}", port);

    let blackjack_service = Arc::new(BlackjackService::new());

    let server = run(listener, blackjack_service.clone()).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        addr,
        blackjack_service,
        client: reqwest::Client::new(),
    }
}
