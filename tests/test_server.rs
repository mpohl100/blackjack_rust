mod helper;

use crate::helper::spawn_app;
use reqwest::StatusCode;

#[tokio::test]
async fn play_blackjack_through_server() {
    // Arrange
    let app = spawn_app().await;

    // Create a game and assert success
    // Act
    let create_game_response = app.client
        .post(&format!("{}/blackjack", app.addr))
        .send()
        .await
        .expect("Failed to create game");

    // Assert
    assert_eq!(create_game_response.status(), StatusCode::CREATED);
    let game_response: serde_json::Value = create_game_response
        .json()
        .await
        .expect("Failed to parse game creation response");
    let game_id = game_response["id"].as_str().expect("No game ID in response");
    let access_token = game_response["access_token"].as_str().expect("No access token in response");

    // Play 10 actions and assert success
    let actions = ["hit", "stand", "double", "split"];
    for i in 0..10 {
        let action = actions[i % actions.len()];
        // Act
        let play_response = app.client
            .post(&format!("{}/blackjack/{}/play", app.addr, game_id))
            .query(&[("action", action)])
            .bearer_auth(access_token)
            .send()
            .await
            .expect("Failed to play action");

        // Assert
        assert_eq!(play_response.status(), StatusCode::OK);
        let game_state: serde_json::Value = play_response
            .json()
            .await
            .expect("Failed to parse game state response");
        
        // Additional assertions can be done here based on the game state
    }

    // Delete the game and assert success
    // Act
    let delete_response = app.client
        .delete(&format!("{}/blackjack/{}", app.addr, game_id))
        .bearer_auth(access_token)
        .send()
        .await
        .expect("Failed to delete game");

    // Assert
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
}
