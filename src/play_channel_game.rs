use tokio::sync::mpsc;
use blackjack_rust::game::channel_game::ChannelGame;
use blackjack_rust::game::channel_game::GameAction;

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
    let option_sender_clone = option_sender.clone();
    let t = tokio::spawn(async move {
        let mut channel_game = ChannelGame::new(action_receiver, option_sender_clone);
        loop{
            channel_game.play();
            if !channel_game.ask_to_play_another_hand() {
                break;
            }
        }
    });

    loop{
        let options = option_receiver.recv().await.unwrap();
        if options.len() == 0 {
            break;
        }
        let first_option = options.first().unwrap();
        action_sender.send(*first_option).await.unwrap();
    }

    t.await.unwrap();
}