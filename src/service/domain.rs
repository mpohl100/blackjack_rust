use blackjack::game::channel_game::ChannelGame;
use std::collections::HashMap;

pub struct BlackjackGame{
    channel_game: ChannelGame,
}

impl BlackjackGame{
    pub async fn new() -> Self{
        let mut game = BlackjackGame{
            channel_game: ChannelGame::new(),
        };
        game.start().await;
        game
    }

    pub async fn start(&mut self){

    }
}

pub struct BlackjackService{
    games: HashMap<String, BlackjackGame>,
}

impl BlackjackService{
    pub fn new() -> Self{
        BlackjackService{
            games: HashMap::new(),
        }
    }

    pub async fn create_game(&mut self) -> String{
        let game = BlackjackGame::new().await;
        let game_id = game.channel_game.id.clone();
        self.games.insert(game_id.clone(), game);
        game_id
    }

    pub async fn delete_game(&mut self, game_id: String){
        self.games.remove(&game_id);
    }

    pub async fn play_game(&mut self, game_id: String, action: String){
        if let Some(game) = self.games.get_mut(&game_id){
            game.channel_game.send(action);
        }
    }
}