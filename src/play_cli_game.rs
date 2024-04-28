use blackjack_rust::game::cli_game::CliGame;

#[tokio::main]
async fn main() {
    let mut game = CliGame::new().await;
    // while user inputs 'y' play another hand
    loop {
        game.play().await;
        // if user inputs 'n' break out of loop
        if !game.ask_to_play_another_hand() {
            break;
        }
    }
}
