use crate::service::domain::BlackjackService;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
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

pub async fn create_game(blackjack_service: web::Data<BlackjackService>) -> impl Responder {
    let create_game_response = blackjack_service.create_game().await;
    HttpResponse::Created().json(GameResponse {
        id: create_game_response.game_id,
        access_token: create_game_response.game_token.to_string(),
    })
}

pub async fn delete_game(
    blackjack_service: web::Data<BlackjackService>,
    req: HttpRequest,
    info: web::Path<(String,)>,
) -> impl Responder {
    // Your implementation to delete the game with the specified ID
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(stripped) = auth_str.strip_prefix("Bearer ") {
                // Check token validity and permission
                let token = stripped;
                let game_id = info.into_inner().0;
                let blackjack_game = blackjack_service.get_game(game_id.clone()).await;
                // Implement your token validation logic here
                if let Some(game) = blackjack_game {
                    if game.lock().await.game_token.to_string() == token {
                        // Implement your game deletion logic here
                        blackjack_service.delete_game(game_id).await;
                        return HttpResponse::NoContent();
                    }
                }
                return HttpResponse::Unauthorized();
            }
        }
    }
    HttpResponse::Unauthorized()
}

pub async fn play_game(
    blackjack_service: web::Data<BlackjackService>,
    req: HttpRequest,
    info: web::Path<(String,)>,
    query: web::Query<Action>,
) -> impl Responder {
    // Your implementation to play the game with the specified ID and action
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(stripped) = auth_str.strip_prefix("Bearer ") {
                let token = stripped;
                // Check token validity and permission
                // Implement your token validation logic here
                let game_id = info.into_inner().0;
                let blackjack_game = blackjack_service.get_game(game_id.clone()).await;
                if let Some(game) = blackjack_game {
                    if game.lock().await.game_token.to_string() == token {
                        // Implement your game playing logic here
                        let action = query.into_inner();
                        blackjack_service.play_game(game_id, action.action).await;
                    }
                }
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
pub struct Action {
    action: String,
}
