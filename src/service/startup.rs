use crate::service::domain::BlackjackService;
use crate::service::routes::{create_game, delete_game, play_game};
use actix_web::{dev::Server, web, web::Data, App, HttpServer};

use std::net::TcpListener;

use std::sync::Arc;
use tokio::sync::Mutex;

pub fn run(
    listener: TcpListener,
    blackjack_service: Arc<Mutex<BlackjackService>>,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/blackjack", web::post().to(create_game))
            .route("/blackjack/{game_id}", web::delete().to(delete_game))
            .route("/blackjack/{game_id}/play", web::post().to(play_game))
            .app_data(Data::new(blackjack_service.clone()))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
