use blackjack_rust::service::routes::{create_game, delete_game, play_game};
use blackjack_rust::service::domain::BlackjackService;

use actix_web::{web, App, HttpServer, web::Data};
use tokio::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/blackjack", web::post().to(create_game))
            .route("/blackjack/{game_id}", web::delete().to(delete_game))
            .route("/blackjack/{game_id}/play", web::post().to(play_game))
            .app_data(Data::new(Mutex::new(BlackjackService::new())))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
