use crate::game::channel_game::ChannelGame;
use crate::game::channel_game::GameAction;
use crate::game::channel_game::GameInfo;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

pub struct BlackjackGame {
    pub game_token: Uuid,
    action_sender: Option<mpsc::Sender<GameAction>>,
    option_receiver: Option<mpsc::Receiver<Vec<GameAction>>>,
    game_info_receiver: Option<mpsc::Receiver<GameInfo>>,
    thread_handle: Option<tokio::task::JoinHandle<()>>,
}

impl BlackjackGame {
    pub async fn new() -> Self {
        let mut game = BlackjackGame {
            game_token: Uuid::new_v4(),
            action_sender: None,
            option_receiver: None,
            game_info_receiver: None,
            thread_handle: None,
        };
        game.start().await;
        game
    }

    pub async fn start(&mut self) {
        let (action_sender, action_receiver) = mpsc::channel::<GameAction>(32);
        let (option_sender, option_receiver) = mpsc::channel::<Vec<GameAction>>(32);
        let (game_info_sender, game_info_receiver) = mpsc::channel::<GameInfo>(32);
        self.action_sender = Some(action_sender);
        self.option_receiver = Some(option_receiver);
        self.game_info_receiver = Some(game_info_receiver);
        let option_sender_clone = option_sender.clone();
        let game_info_sender_clone = game_info_sender.clone();
        let t = tokio::spawn(async move {
            let mut channel_game = ChannelGame::new(action_receiver, option_sender_clone, game_info_sender_clone, true).await;
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

pub struct BlackjackService {
    games: Mutex<HashMap<String, Arc<Mutex<BlackjackGame>>>>,
}

impl Default for BlackjackService {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CreateGameResponse {
    pub game_id: String,
    pub game_token: Uuid,
}

impl CreateGameResponse {
    pub fn new(game_id: String, game_token: Uuid) -> CreateGameResponse {
        CreateGameResponse {
            game_id,
            game_token,
        }
    }
}

impl BlackjackService {
    pub fn new() -> Self {
        BlackjackService {
            games: Mutex::new(HashMap::new()),
        }
    }

    pub async fn create_game(&self) -> CreateGameResponse {
        let game = Arc::new(Mutex::new(BlackjackGame::new().await));
        let mut data = self.games.lock().await;
        let game_id = data.len().to_string();
        data.insert(game_id.clone(), game.clone());
        let token = game.lock().await.game_token;
        CreateGameResponse::new(game_id, token)
    }

    pub async fn get_game(&self, game_id: String) -> Option<Arc<Mutex<BlackjackGame>>> {
        self.games.lock().await.get(&game_id).cloned()
    }

    pub async fn delete_game(&self, game_id: String) -> bool {
        let game = self.games.lock().await.remove(&game_id);
        if let Some(game) = game {
            let sender = &game.lock().await.action_sender;
            if let Some(s) = sender {
                let _ = s.send(GameAction::Stop).await;
            } else {
                return false;
            }
            if let Some(t) = &game.lock().await.thread_handle {
                t.abort();
            } else {
                return false;
            }
        }
        true
    }

    pub async fn play_game(&self, game_id: String, action: String) {
        if let Some(game) = self.games.lock().await.get_mut(&game_id) {
            if let Some(sender) = &game.lock().await.action_sender {
                let _ = sender
                    .send(GameAction::from(
                        action
                            .to_lowercase()
                            .as_str()
                            .chars()
                            .next()
                            .unwrap()
                            .to_ascii_lowercase(),
                    ))
                    .await;
            }
            if let Some(receiver) = game.lock().await.option_receiver.as_mut() {
                let _options = receiver.recv().await;
            }
        }
    }
}
