use blackjack_rust::service::domain::BlackjackService;

#[tokio::test]
async fn play_blackjack_through_service_class(){

    // create a game
    let blackjack_service = BlackjackService::new();
    let create_game_response = blackjack_service.create_game().await;

    assert!(create_game_response.game_id.to_string().len() > 0);
    assert!(create_game_response.game_token.to_string().len() > 0);

    // initialize playing of the game
    let mut game_info = blackjack_service.play_game(create_game_response.game_id, "".to_owned()).await;

    for _ in 0..10 {
        // play 10 actions
        let action = "h".to_owned();
        game_info = blackjack_service.play_game(create_game_response.game_id.clone(), action).await;

        assert!(game_info.game_info.hands.len() > 0);
    }

    // delete the game
    let delete_response = blackjack_service.delete_game(create_game_response.game_id).await;

    assert!(delete_response);



}