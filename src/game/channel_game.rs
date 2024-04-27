use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::deck::Card;
use crate::blackjack::deck::WrappedDeck;
use crate::blackjack::deck::EightDecks;
use crate::blackjack::hand::DealerHand;
use crate::blackjack::hand::PlayerHand;
use crate::blackjack::play_blackjack_hand::play_blackjack_hand;
use crate::blackjack::play_blackjack_hand::PlayMode;
use crate::blackjack::rng::RandomNumberGenerator;
use crate::blackjack::strategy::blackjack_strategy_combined_ordered_hash_map::BlackjackStrategyCombinedOrderedHashMap;

use crate::blackjack::analysis::blackjack_analysis::optimize_blackjack;
use crate::blackjack::traits::BlackjackGame;
use crate::blackjack::traits::BlackjackStrategyTrait;

use std::cmp::Ordering;

use async_trait::async_trait;

use tokio::sync::mpsc;

#[derive(Clone, Copy)]
pub enum GameAction{
    Stop,
    Continue,
    Split,
    DoubleDown,
    Hit,
    Stand,
}

struct GameData {
    optimal_strategy: Box<dyn BlackjackStrategyTrait + Send>,
    nb_hands_played: i32,
    nb_right_decisions: i32,
    action_receiver: mpsc::Receiver<GameAction>,
    option_sender: mpsc::Sender<Vec<GameAction>>,
}

impl GameData {
    pub fn new(optimal_strategy: Box<dyn BlackjackStrategyTrait + Send>, action_receiver: mpsc::Receiver<GameAction>, option_sender: mpsc::Sender<Vec<GameAction>>) -> GameData {
        GameData {
            optimal_strategy,
            nb_hands_played: 0,
            nb_right_decisions: 0,
            action_receiver: action_receiver,
            option_sender: option_sender,
        }
    }
}

struct GameState {
    rng: RandomNumberGenerator,
    deck: WrappedDeck,
    dealer_hand: DealerHand,
    player_hand: PlayerHand,
    current_balance: f64,
    previous_balance: f64,
    nb_hands_played: i32,
    player_bet: f64,
    game_data: GameData,
}

impl GameState {
    pub fn new(optimal_strategy: Box<dyn BlackjackStrategyTrait + Send>, action_receiver: mpsc::Receiver<GameAction>, option_sender: mpsc::Sender<Vec<GameAction>>) -> GameState {
        GameState {
            rng: RandomNumberGenerator::new(),
            deck: WrappedDeck::new(Box::new(EightDecks::new())),
            dealer_hand: DealerHand::new(&[Card::new_with_int(0), Card::new_with_int(1)]),
            player_hand: PlayerHand::new(&[Card::new_with_int(2), Card::new_with_int(3)]),
            current_balance: 1000.0,
            previous_balance: 1000.0,
            nb_hands_played: 0,
            player_bet: 1.0,
            game_data: GameData::new(optimal_strategy, action_receiver, option_sender),
        }
    }

    pub fn deal(&mut self) {
        self.dealer_hand = DealerHand::new(&[
            self.deck.deal_card(&mut self.rng),
            self.deck.deal_card(&mut self.rng),
        ]);
        self.player_hand = PlayerHand::new(&[
            self.deck.deal_card(&mut self.rng),
            self.deck.deal_card(&mut self.rng),
        ]);
    }

    pub fn print_before_hand(&self) {
        println!("Starting to play hand number {}", self.nb_hands_played);
        println!("Your balance is: {}", self.current_balance);
        println!("Your hand: {:?}", self.player_hand.get_cards());
    }

    pub async fn play(&mut self) {
        self.previous_balance = self.current_balance;
        let mut game = GameStrategy::new(&mut self.game_data);
        self.current_balance += play_blackjack_hand(
            self.player_bet,
            self.player_hand.clone(),
            self.dealer_hand.clone(),
            &mut self.deck,
            &mut game,
            &mut self.rng,
            PlayMode::All,
        ).await;
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
}

impl ChannelGame {
    pub async fn new(action_receiver: mpsc::Receiver<GameAction>, option_sender: mpsc::Sender<Vec<GameAction>>) -> ChannelGame {
        let game_strat = BlackjackStrategyCombinedOrderedHashMap::new();
        let optimal_strategy = optimize_blackjack(game_strat, 0).await;
        ChannelGame {
            game_state: GameState::new(Box::new(optimal_strategy), action_receiver, option_sender),
        }
    }

    pub fn play(&mut self) {
        self.game_state.nb_hands_played += 1;
        self.game_state.deal();
        self.game_state.print_before_hand();
        self.game_state.play();
        self.game_state.print_after_hand();
    }

    pub fn ask_to_play_another_hand(&self) -> bool {
        println!("Do you want to play another hand? (y/n)");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim() == "y"
    }
}

struct GameStrategy<'_gs> {
    game_data: &'_gs mut GameData,
}

impl GameStrategy<'_> {
    pub fn new(game_data: &mut GameData) -> GameStrategy {
        GameStrategy { game_data }
    }
}

#[async_trait]
impl BlackjackGame for GameStrategy<'_> {
    async fn get_draw(&mut self, situation: HandSituation, _deck: &mut WrappedDeck) -> bool {
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
        let mut input = String::new();
        println!("Do you want to draw? (y/n)");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let result = input.trim() == "y";
        if result == self.game_data.optimal_strategy.get_draw(situation, _deck).await {
            println!("Right decision");
            self.game_data.nb_right_decisions += 1;
        } else {
            println!("Wrong decision");
        }
        self.game_data.nb_hands_played += 1;
        result
    }

    async fn get_double_down(&mut self, situation: HandSituation, _deck: &mut WrappedDeck) -> bool {
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
        let mut input = String::new();
        println!("Do you want to double down? (y/n)");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let result = input.trim() == "y";
        if result
            == self
                .game_data
                .optimal_strategy
                .get_double_down(situation, _deck).await
        {
            println!("Right decision");
            self.game_data.nb_right_decisions += 1;
        } else {
            println!("Wrong decision");
        }
        self.game_data.nb_hands_played += 1;
        result
    }

    async fn get_split(&mut self, situation: SplitSituation, _deck: &mut WrappedDeck) -> bool {
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
        println!("Your options are split (s), double down (d), hit (h), stand (t)");
        self.game_data.option_sender.send(vec![GameAction::Split, GameAction::DoubleDown, GameAction::Hit, GameAction::Stand]);
        let choice = self.game_data.action_receiver.recv().await.unwrap();
        let mut input = String::new();
        println!("Do you want to split? (y/n)");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let result = input.trim() == "y";
        if result == self.game_data.optimal_strategy.get_split(situation, _deck).await {
            println!("Right decision");
            self.game_data.nb_right_decisions += 1;
        } else {
            println!("Wrong decision");
        }
        self.game_data.nb_hands_played += 1;
        result
    }
}
