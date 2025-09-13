
use blackjack_rust::service::domain::BlackjackService;
use blackjack_rust::service::startup::run;
use reqwest::Client;
use std::net::TcpListener;

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct TestApp {
    pub addr: String,
    pub blackjack_service: Arc<Mutex<BlackjackService>>,
    pub client: reqwest::Client,
    pub handle: tokio::task::JoinHandle<Result<(), std::io::Error>>,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port();

    let addr = format!("http://127.0.0.1:{}", port);

    let blackjack_service = Arc::new(Mutex::new(BlackjackService::new()));

    let server = run(listener, blackjack_service.clone()).expect("Failed to bind address");

    let handle = tokio::spawn(server);

    TestApp {
        addr,
        blackjack_service,
        client: reqwest::Client::new(),
        handle,
    }
}
