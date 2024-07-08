use blackjack_rust::game::channel_game::get_options_string;
use blackjack_rust::game::channel_game::ChannelGame;
use blackjack_rust::game::channel_game::GameAction;
use blackjack_rust::game::channel_game::GameInfo;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await.unwrap();
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }

    let (action_sender, action_receiver) = mpsc::channel::<GameAction>(32);
    let (option_sender, mut option_receiver) = mpsc::channel::<Vec<GameAction>>(32);
    let (game_info_sender, _game_info_receiver) = mpsc::channel::<GameInfo>(1);
    let option_sender_clone = option_sender.clone();
    let t = tokio::spawn(async move {
        let mut channel_game =
            ChannelGame::new(action_receiver, option_sender_clone, game_info_sender, true).await;
        loop {
            channel_game.play().await;
            if !channel_game.ask_to_play_another_hand().await {
                break;
            }
        }
    });

    loop {
        let options = option_receiver.recv().await.unwrap();
        if options.is_empty() {
            break;
        }
        println!("Which option do you want to choose?");
        let options_str = get_options_string(&options);
        println!("{}", options_str);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let choice = GameAction::from(
            input
                .to_lowercase()
                .as_str()
                .chars()
                .next()
                .unwrap()
                .to_ascii_lowercase(),
        );
        action_sender.send(choice).await.unwrap();
        if choice == GameAction::Stop {
            break;
        }
    }

    t.await.unwrap();
}
