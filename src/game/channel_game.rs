use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::deck::EightDecks;
use crate::blackjack::deck::WrappedDeck;
use crate::blackjack::hand::DealerHand;
use crate::blackjack::hand::PlayerHand;
use crate::blackjack::play_blackjack_hand::play_blackjack_hand_new;
use crate::blackjack::play_blackjack_hand::HandData;
use crate::blackjack::play_blackjack_hand::PlayerHandData;
use crate::blackjack::play_blackjack_hand::HandInfo;
use crate::blackjack::play_blackjack_hand::WrappedHandData;
use crate::blackjack::play_blackjack_hand::PlayMode;
use crate::blackjack::rng::RandomNumberGenerator;
use crate::blackjack::strategy::blackjack_strategy_combined_ordered_hash_map::BlackjackStrategyCombinedOrderedHashMap;

use crate::blackjack::analysis::blackjack_analysis::optimize_blackjack;
use crate::blackjack::traits::BlackjackGame;
use crate::blackjack::traits::WrappedGame;
use crate::blackjack::traits::WrappedStrategy;

use std::cmp::Ordering;

use async_trait::async_trait;

use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameAction {
    Stop,
    Continue,
    Split,
    DoubleDown,
    Hit,
    Stand,
}

pub fn get_options_string(options: &Vec<GameAction>) -> String {
    let mut options_str = String::new();
    for option in options {
        match option {
            GameAction::Split => options_str
                .push_str(&("Split (".to_owned() + &get_short_letter(GameAction::Split) + ") ")),
            GameAction::DoubleDown => options_str.push_str(
                &("Double Down (".to_owned() + &get_short_letter(GameAction::DoubleDown) + ") "),
            ),
            GameAction::Hit => options_str
                .push_str(&("Hit (".to_owned() + &get_short_letter(GameAction::Hit) + ") ")),
            GameAction::Stand => options_str
                .push_str(&("Stand (".to_owned() + &get_short_letter(GameAction::Stand) + ") ")),
            GameAction::Stop => options_str
                .push_str(&("Stop (".to_owned() + &get_short_letter(GameAction::Stop) + ") ")),
            _ => (),
        }
    }
    options_str
}

pub fn get_short_letter(action: GameAction) -> String {
    match action {
        GameAction::Split => 's'.to_string(),
        GameAction::DoubleDown => 'd'.to_string(),
        GameAction::Hit => 'h'.to_string(),
        GameAction::Stand => 't'.to_string(),
        GameAction::Stop => 'x'.to_string(),
        _ => ' '.to_string(),
    }
}

pub fn get_word(action: GameAction) -> String {
    match action {
        GameAction::Split => return "Split".to_owned(),
        GameAction::DoubleDown => return "Double Down".to_owned(),
        GameAction::Hit => "Hit".to_owned(),
        GameAction::Stand => "Stand".to_owned(),
        GameAction::Stop => "Stop".to_owned(),
        _ => "Error".to_owned(),
    }
}

impl From<char> for GameAction {
    fn from(letter: char) -> GameAction {
        match letter {
            's' => GameAction::Split,
            'd' => GameAction::DoubleDown,
            'h' => GameAction::Hit,
            't' => GameAction::Stand,
            'x' => GameAction::Stop,
            _ => GameAction::Continue,
        }
    }
}

struct GameData {
    optimal_strategy: WrappedStrategy,
    nb_hands_played: i32,
    nb_right_decisions: i32,
    action_receiver: mpsc::Receiver<GameAction>,
    option_sender: mpsc::Sender<Vec<GameAction>>,
    cached_decision: Option<GameAction>,
}

impl GameData {
    pub fn new(
        optimal_strategy: WrappedStrategy,
        action_receiver: mpsc::Receiver<GameAction>,
        option_sender: mpsc::Sender<Vec<GameAction>>,
    ) -> GameData {
        GameData {
            optimal_strategy,
            nb_hands_played: 0,
            nb_right_decisions: 0,
            action_receiver,
            option_sender,
            cached_decision: None,
        }
    }
}

#[derive(Clone, Default)]
pub struct GameInfo {
    pub dealer_hand: DealerHand,
    pub player_hand: PlayerHand,
    pub current_balance: f64,
    pub nb_hands_played: i32,
    pub nb_right_decisions: i32,
    pub active_hand_finished: bool,
    pub player_bet: f64,
}

struct ChannelHandInfo{
    hand_info: HandInfo,
    game_info_sender: mpsc::Sender<GameInfo>,
}

impl ChannelHandInfo {
    pub fn new(hand_info: HandInfo, game_info_sender: mpsc::Sender<GameInfo>) -> ChannelHandInfo {
        ChannelHandInfo {
            hand_info,
            game_info_sender,
        }
    }

    async fn get_game_info(&mut self, active_hand_finished: bool) -> GameInfo{
        let game_info = GameInfo {
            dealer_hand: self.hand_info.get_dealer_hand().await.clone(),
            player_hand: self.hand_info.get_active_hand().await.clone(),
            current_balance: 0.0,
            nb_hands_played: 0,
            nb_right_decisions: 0,
            active_hand_finished: active_hand_finished,
            player_bet: self.hand_info.get_active_bet().await,
        };
        game_info
    }
}

#[async_trait]
impl HandData for ChannelHandInfo{
    async fn play_dealer_hand(&mut self, deck: &mut WrappedDeck, rng: &mut RandomNumberGenerator) -> i32 {
        self.hand_info.play_dealer_hand(deck, rng).await
    }

    async fn get_active_hand(&mut self) -> &mut PlayerHand {
        self.hand_info.get_active_hand().await
    }

    async fn get_dealer_hand(&mut self) -> &mut DealerHand {
        self.hand_info.get_dealer_hand().await
    }

    async fn remove_active_hand(&mut self) -> PlayerHandData {
        self.hand_info.remove_active_hand().await
    }

    async fn add_player_hand(&mut self, hand: PlayerHandData) {
        self.hand_info.add_player_hand(hand).await
    }

    async fn set_active_hand(&mut self, index: i32) {
        self.hand_info.set_active_hand(index).await;
    }

    async fn get_active_index(&self) -> i32 {
        self.hand_info.get_active_index().await
    }

    async fn set_active_bet(&mut self, bet: f64) {
        self.hand_info.set_active_bet(bet).await
    }

    async fn get_active_bet(&self) -> f64 {
        self.hand_info.get_active_bet().await
    }

    async fn send_game_info(&mut self, active_hand_finished: bool) {
        let game_info = self.get_game_info(active_hand_finished).await;
        self.game_info_sender.send(game_info).await.unwrap();
    }
}

struct GameState {
    rng: RandomNumberGenerator,
    deck: WrappedDeck,
    hand_info: Option<WrappedHandData>,
    current_balance: f64,
    previous_balance: f64,
    nb_hands_played: i32,
    player_bet: f64,
    game_data: Arc<Mutex<GameData>>,
    game_info_sender: mpsc::Sender<GameInfo>,
    do_print: bool,
}

impl GameState {
    pub fn new(
        optimal_strategy: WrappedStrategy,
        action_receiver: mpsc::Receiver<GameAction>,
        option_sender: mpsc::Sender<Vec<GameAction>>,
        game_info_sender: mpsc::Sender<GameInfo>,
        do_print: bool,
    ) -> GameState {
        GameState {
            rng: RandomNumberGenerator::new(),
            deck: WrappedDeck::new(Box::new(EightDecks::new())),
            hand_info: None,
            current_balance: 1000.0,
            previous_balance: 1000.0,
            nb_hands_played: 0,
            player_bet: 1.0,
            game_data: Arc::new(Mutex::new(GameData::new(
                optimal_strategy,
                action_receiver,
                option_sender,
            ))),
            game_info_sender: game_info_sender,
            do_print: do_print,
        }
    }

    pub fn deal(&mut self) {
        self.hand_info = Some(WrappedHandData::new(Box::new(ChannelHandInfo::new(HandInfo::new(self.player_bet, &mut self.deck, &mut self.rng), self.game_info_sender.clone()))));
    }

    pub async fn print_before_hand(&self) {
        println!("Starting to play hand number {}", self.nb_hands_played);
        println!("Your balance is: {}", self.current_balance);
        if let Some(hand_info) = self.hand_info.as_ref() {
            println!("Your hand: {:?}", hand_info.hand_data.lock().await.get_active_hand().await.get_cards());
        }
    }

    pub async fn play(&mut self) {
        self.previous_balance = self.current_balance;
        let game = GameStrategy::new(self.game_data.clone(), self.do_print);
        let wrapped_game = WrappedGame::new(game);
        self.current_balance += play_blackjack_hand_new(
            &mut self.hand_info.as_mut().unwrap(),
            &mut self.deck,
            wrapped_game,
            &mut self.rng,
            PlayMode::All,
        )
        .await;
    }

    pub fn print_after_hand(&self) {
        let hand_result = self.current_balance - self.previous_balance;
        let hand_result_int = hand_result as i32;
        match hand_result_int.cmp(&0) {
            Ordering::Less => println!("You lost: {}", hand_result),
            Ordering::Equal => println!("You tied"),
            Ordering::Greater => println!("You won: {}", hand_result),
        }
        println!("Your current balance is: {}", self.current_balance);
    }
}

pub struct ChannelGame {
    game_state: GameState,
    do_print: bool,
}

impl ChannelGame {
    pub async fn new(
        action_receiver: mpsc::Receiver<GameAction>,
        option_sender: mpsc::Sender<Vec<GameAction>>,
        game_info_sender: mpsc::Sender<GameInfo>,
        do_print: bool,
    ) -> ChannelGame {
        let game_strat = WrappedStrategy::new(BlackjackStrategyCombinedOrderedHashMap::new());
        let optimal_strategy = optimize_blackjack(game_strat, 0).await;
        ChannelGame {
            game_state: GameState::new(
                optimal_strategy,
                action_receiver,
                option_sender,
                game_info_sender,
                do_print,
            ),
            do_print,
        }
    }

    pub async fn play(&mut self) {
        self.game_state.nb_hands_played += 1;
        self.game_state.deal();
        if self.do_print {
            self.game_state.print_before_hand().await;
        }
        self.game_state.play().await;
        if self.do_print {
            self.game_state.print_after_hand();
        }
    }

    pub async fn ask_to_play_another_hand(&self) -> bool {
        if let Some(cached_decision) = self.game_state.game_data.lock().await.cached_decision {
            if cached_decision == GameAction::Stop {
                return false;
            }
        }
        self.game_state.game_data.lock().await.cached_decision = None;
        true
    }
}

struct GameStrategy {
    game_data: Arc<Mutex<GameData>>,
    do_print: bool,
}

impl GameStrategy {
    pub fn new(game_data: Arc<Mutex<GameData>>, do_print: bool) -> GameStrategy {
        GameStrategy {
            game_data,
            do_print,
        }
    }

    async fn evaluate_double_down(
        &mut self,
        action: GameAction,
        situation: HandSituation,
        _deck: &mut WrappedDeck,
    ) -> bool {
        let do_double_down = action == GameAction::DoubleDown;
        if do_double_down
            == self
                .game_data
                .lock()
                .await
                .optimal_strategy
                .get_double_down(situation, _deck)
                .await
        {
            if self.do_print {
                println!("Right decision for double down");
            }
            self.game_data.lock().await.nb_right_decisions += 1;
        } else {
            if self.do_print {
                println!("Wrong decision for double down");
            }
        }
        self.game_data.lock().await.nb_hands_played += 1;
        if do_double_down {
            self.game_data.lock().await.cached_decision = None;
            true
        } else {
            self.game_data.lock().await.cached_decision = Some(action);
            false
        }
    }

    async fn evaluate_draw(
        &mut self,
        action: GameAction,
        situation: HandSituation,
        _deck: &mut WrappedDeck,
    ) -> bool {
        let do_draw = action == GameAction::Hit;
        if do_draw
            == self
                .game_data
                .lock()
                .await
                .optimal_strategy
                .get_draw(situation, _deck)
                .await
        {
            if self.do_print {
                println!("Right decision for draw");
            }
            self.game_data.lock().await.nb_right_decisions += 1;
        } else {
            if self.do_print {
                println!("Wrong decision for draw");
            }
        }
        self.game_data.lock().await.nb_hands_played += 1;
        if do_draw {
            self.game_data.lock().await.cached_decision = None;
            true
        } else {
            self.game_data.lock().await.cached_decision = Some(action);
            false
        }
    }
}

#[async_trait]
impl BlackjackGame for GameStrategy {
    async fn get_draw(&mut self, situation: HandSituation, _deck: &mut WrappedDeck) -> bool {
        if self.do_print {
            println!(
                "The dealer is showing: {}",
                situation
                    .dealer_card()
                    .get_representative_card()
                    .to_blackjack_score()
            );
            println!(
                "Your hand is: {} {}",
                match situation.situation().lower() == situation.situation().upper() {
                    true => "Hard",
                    false => "Soft",
                },
                situation.situation().upper()
            );
        }
        let mut evaluate_now = false;
        if let Some(cached_decision) = self.game_data.lock().await.cached_decision {
            if cached_decision == GameAction::Stop {
                return false;
            } else if cached_decision == GameAction::Hit || cached_decision == GameAction::Stand {
                evaluate_now = true;
            }
        }
        if evaluate_now {
            let choice = self.game_data.lock().await.cached_decision.unwrap();
            return self.evaluate_draw(choice, situation, _deck).await;
        }
        let _ = self
            .game_data
            .lock()
            .await
            .option_sender
            .send(vec![GameAction::Hit, GameAction::Stand, GameAction::Stop])
            .await;
        let choice = self
            .game_data
            .lock()
            .await
            .action_receiver
            .recv()
            .await
            .unwrap();
        self.evaluate_draw(choice, situation, _deck).await
    }

    async fn get_double_down(&mut self, situation: HandSituation, _deck: &mut WrappedDeck) -> bool {
        if self.do_print {
            println!(
                "The dealer is showing: {}",
                situation
                    .dealer_card()
                    .get_representative_card()
                    .to_blackjack_score()
            );
            println!(
                "Your hand is: {} {}",
                match situation.situation().lower() == situation.situation().upper() {
                    true => "Hard",
                    false => "Soft",
                },
                situation.situation().upper()
            );
        }
        let mut evaluate_now = false;
        if let Some(cached_decision) = self.game_data.lock().await.cached_decision {
            if cached_decision == GameAction::Stop {
                return false;
            } else if cached_decision == GameAction::DoubleDown {
                evaluate_now = true;
            }
        }
        if evaluate_now {
            return self
                .evaluate_double_down(GameAction::DoubleDown, situation, _deck)
                .await;
        }
        let _ = self
            .game_data
            .lock()
            .await
            .option_sender
            .send(vec![
                GameAction::DoubleDown,
                GameAction::Hit,
                GameAction::Stand,
                GameAction::Stop,
            ])
            .await;
        let choice = self
            .game_data
            .lock()
            .await
            .action_receiver
            .recv()
            .await
            .unwrap();
        self.evaluate_double_down(choice, situation, _deck).await
    }

    async fn get_split(&mut self, situation: SplitSituation, _deck: &mut WrappedDeck) -> bool {
        if self.do_print {
            println!(
                "The dealer is showing: {}",
                situation
                    .dealer_card()
                    .get_representative_card()
                    .to_blackjack_score()
            );
            println!(
                "Your hand rank is: {}",
                situation
                    .situation()
                    .get_representative_card()
                    .to_blackjack_score()
            );
        }
        if let Some(cached_decision) = self.game_data.lock().await.cached_decision {
            if cached_decision == GameAction::Stop {
                return false;
            }
        }
        let _ = self
            .game_data
            .lock()
            .await
            .option_sender
            .send(vec![
                GameAction::Split,
                GameAction::DoubleDown,
                GameAction::Hit,
                GameAction::Stand,
                GameAction::Stop,
            ])
            .await;
        let choice = self
            .game_data
            .lock()
            .await
            .action_receiver
            .recv()
            .await
            .unwrap();
        let do_it = choice == GameAction::Split;
        if do_it
            == self
                .game_data
                .lock()
                .await
                .optimal_strategy
                .get_split(situation, _deck)
                .await
        {
            if self.do_print {
                println!("Right decision for split");
            }
            self.game_data.lock().await.nb_right_decisions += 1;
        } else {
            if self.do_print {
                println!("Wrong decision for split");
            }
        }
        self.game_data.lock().await.nb_hands_played += 1;
        if do_it {
            self.game_data.lock().await.cached_decision = None;
            true
        } else {
            self.game_data.lock().await.cached_decision = Some(choice);
            false
        }
    }
}
