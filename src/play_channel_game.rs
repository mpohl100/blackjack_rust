use tokio::sync::mpsc;
use blackjack_rust::game::channel_game::ChannelGame;

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

    let t = tokio::spawn(async move {
        let mut channel_game = ChannelGame::new();
        loop{
            channel_game.play();
            if !channel_game.ask_to_play_another_hand() {
                break;
            }
        }
    });

    t.await.unwrap();
}