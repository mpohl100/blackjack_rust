use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct GameResponse {
    id: String,
    access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Hand {
    cards: Vec<Card>,
    score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Card {
    rank: String,
    suit: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GameState {
    player_hands: Vec<Hand>,
    dealer_hand: Hand,
    player_options: Vec<String>,
    active_hand: i32,
    winner: Option<String>,
}

async fn create_game() -> impl Responder {
    // Your implementation to create a new game
    HttpResponse::Created().json(GameResponse {
        id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        access_token: "123e4567-e89b-12d3-a456-426614174000".to_string(),
    })
}

async fn delete_game(req: HttpRequest, _info: web::Path<(String,)>) -> impl Responder {
    // Your implementation to delete the game with the specified ID
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(stripped) = auth_str.strip_prefix("Bearer ") {
                let _token = stripped;
                // Check token validity and permission
                // Implement your token validation logic here
                return HttpResponse::NoContent();
            }
        }
    }
    HttpResponse::Unauthorized()
}

async fn play_game(
    req: HttpRequest,
    info: web::Path<(String,)>,
    query: web::Query<Action>,
) -> impl Responder {
    // Your implementation to play the game with the specified ID and action
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(stripped) = auth_str.strip_prefix("Bearer ") {
                let _token = stripped;
                // Check token validity and permission
                // Implement your token validation logic here
                let _game_id = info.into_inner().0;
                let _action = query.into_inner();
                return HttpResponse::Ok().json(GameState {
                    player_hands: Vec::new(),
                    dealer_hand: Hand {
                        cards: Vec::new(),
                        score: 0,
                    },
                    player_options: vec!["hit".to_string(), "stand".to_string()],
                    active_hand: 0,
                    winner: None,
                });
            }
        }
    }
    HttpResponse::Unauthorized().finish()
}

#[derive(Debug, Deserialize)]
struct Action {
    _action: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/blackjack", web::post().to(create_game))
            .route("/blackjack/{game_id}", web::delete().to(delete_game))
            .route("/blackjack/{game_id}/play", web::post().to(play_game))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
