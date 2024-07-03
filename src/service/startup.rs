use crate::service::domain::BlackjackService;
use crate::service::routes::{create_game, delete_game, play_game};
use actix_web::{web, web::Data, App, HttpServer, dev::Server};

use std::{net::TcpListener, sync::Arc};

pub fn run(listener: TcpListener, blackjack_service: Arc<BlackjackService>) -> Result<Server, std::io::Error> {
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