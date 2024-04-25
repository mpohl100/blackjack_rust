use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::deck::Card;
use crate::blackjack::deck::Deck;
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
use threadpool::ThreadPool;

struct GameData {
    optimal_strategy: Box<dyn BlackjackStrategyTrait>,
}

impl GameData {
    pub fn new(optimal_strategy: Box<dyn BlackjackStrategyTrait>) -> GameData {
        GameData { optimal_strategy }
    }
}

struct GameState {
    rng: RandomNumberGenerator,
    deck: EightDecks,
    dealer_hand: DealerHand,
    player_hand: PlayerHand,
    current_balance: f64,
    previous_balance: f64,
    nb_hands_played: i32,
    player_bet: f64,
    game_data: GameData,
}

impl GameState {
    pub fn new(optimal_strategy: Box<dyn BlackjackStrategyTrait>) -> GameState {
        GameState {
            rng: RandomNumberGenerator::new(),
            deck: EightDecks::new(),
            dealer_hand: DealerHand::new(&[Card::new_with_int(0), Card::new_with_int(1)]),
            player_hand: PlayerHand::new(&[Card::new_with_int(2), Card::new_with_int(3)]),
            current_balance: 1000.0,
            previous_balance: 1000.0,
            nb_hands_played: 0,
            player_bet: 1.0,
            game_data: GameData::new(optimal_strategy),
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

    pub fn play(&mut self) {
        self.previous_balance = self.current_balance;
        let game = GameStrategy::new(&mut self.game_data);
        self.current_balance += play_blackjack_hand(
            self.player_bet,
            self.player_hand.clone(),
            self.dealer_hand.clone(),
            &mut self.deck,
            &game,
            &mut self.rng,
            PlayMode::All,
        );
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

impl Default for ChannelGame {
    fn default() -> Self {
        Self::new()
    }
}

impl ChannelGame {
    pub fn new() -> ChannelGame {
        let game_strat = BlackjackStrategyCombinedOrderedHashMap::new();
        let thread_pool = ThreadPool::new(4);
        let optimal_strategy = optimize_blackjack(game_strat, &thread_pool, 0);
        ChannelGame {
            game_state: GameState::new(Box::new(optimal_strategy)),
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

impl BlackjackGame for GameStrategy<'_> {
    fn get_draw(&self, situation: HandSituation, _deck: &dyn Deck) -> bool {
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
        if result == self.game_data.optimal_strategy.get_draw(situation, _deck) {
            println!("Right decision");
        } else {
            println!("Wrong decision");
        }
        result
    }

    fn get_double_down(&self, situation: HandSituation, _deck: &dyn Deck) -> bool {
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
                .get_double_down(situation, _deck)
        {
            println!("Right decision");
        } else {
            println!("Wrong decision");
        }
        result
    }

    fn get_split(&self, situation: SplitSituation, _deck: &dyn Deck) -> bool {
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
        let mut input = String::new();
        println!("Do you want to split? (y/n)");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let result = input.trim() == "y";
        if result == self.game_data.optimal_strategy.get_split(situation, _deck) {
            println!("Right decision");
        } else {
            println!("Wrong decision");
        }
        result
    }
}
