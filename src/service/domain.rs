use blackjack::game::channel_game::ChannelGame;
use std::collections::HashMap;
use uuid::Uuid;
use tokio::sync::{mpsc, Mutex};

pub struct BlackjackGame{
    channel_game: ChannelGame,
    game_token: Uuid,
    action_sender: Option<mpsc::Sender<GameAction>>,
    option_receiver: Option<mpsc::Receiver<Vec<GameAction>>>,
    thread_handle: Option<tokio::task::JoinHandle<()>>,
}

impl BlackjackGame{
    pub async fn new() -> Self{
        let mut game = BlackjackGame{
            channel_game: ChannelGame::new(),
            game_token: Uuid::new_v4(),
        };
        game.start().await;
        game
    }

    pub async fn start(&mut self){
        let (action_sender, action_receiver) = mpsc::channel::<GameAction>(32);
        let (option_sender, mut option_receiver) = mpsc::channel::<Vec<GameAction>>(32);
        self.action_sender = Some(action_sender);
        self.option_receiver = Some(option_receiver);
        let option_sender_clone = option_sender.clone();
        let t = tokio::spawn(async move {
            let mut channel_game = ChannelGame::new(action_receiver, option_sender_clone).await;
            loop {
                channel_game.play().await;
                if !channel_game.ask_to_play_another_hand().await {
                    break;
                }
            }
        });
        self.thread_handle = Some(t);
    }
}

pub struct BlackjackService{
    games: Mutex<HashMap<String, BlackjackGame>>,
}

pub struct CreateGameResponse{
    game_id: String,
    game_token: Uuid,
}

impl CreateGameResponse{
    pub fn new(game_id: String, game_token: Uuid) -> String{
        CreateGameResponse{
            game_id,
            game_token,
        }
    }
}

impl BlackjackService{
    pub fn new() -> Self{
        BlackjackService{
            games: Mutex::new(HashMap::new()),
        }
    }

    pub async fn create_game(&mut self) -> String{
        let game = BlackjackGame::new().await;
        let guard = self.games.lock().await;
        let mut data = guard.lock().await;
        let game_id = data.len().to_string();
        data.insert(game_id.clone(), game);
        CreateGameResponse::new(game_id, game.game_token)
    }

    pub async fn delete_game(&mut self, game_id: String){
        let game = self.games.lock().await.remove(&game_id);
        if let Some(game) = game{
            game.action_sender.lock().await.send(GameAction::Quit).await;
            game.thread_handle.lock().await.unwrap();
        }
    }

    pub async fn play_game(&mut self, game_id: String, action: String){
        if let Some(game) = self.games.lock().await.get_mut(&game_id){
            game.channel_game.send(action);
        }
    }
}