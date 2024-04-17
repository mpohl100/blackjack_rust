use crate::blackjack::deck::Deck;
use crate::blackjack::deck::EightDecks;
use crate::blackjack::hand::PlayerHand;
use crate::blackjack::hand::DealerHand;
use crate::blackjack::play_blackjack_hand::play_blackjack_hand;

struct GameState{
    rng: RandomNumberGenerator,
    deck: EightDecks,
    dealer_hand: DealerHand,
    player_hand: PlayerHand,
    current_balance: f64,
    nb_hands_played: i32,
    player_bet: f64,
}

impl GameState{
    pub fn new() -> GameState{
        GameState{
            deck: EightDecks::new(),
            dealer_hand: DealerHand::new(),
            player_hand: PlayerHand::new(),
            current_balance: 1000.0,
            nb_hands_played: 0,
            player_bet: 1.0,
        }
    }

    pub fn deal(&mut self){
        self.dealer_hand = DealerHand::new(&[self.deck.deal_card(self.rng), self.deck.deal_card(self.rng)]);
        self.player_hand = PlayerHand::new(&[self.deck.deal_card(self.rng), self.deck.deal_card(self.rng)]);
    }

    pub fn print_before_hand(&self){
        println!("Starting to play hand number {}", self.nb_hands_played);
        println!("Your balance is: {}", self.current_balance);
        println!("Your hand: {:?}", self.player_hand.to_string_internal());
    }

    pub fn play(&mut self){
        self.previous_balance = self.current_balance;
        self.current_balance += play_blackjack_hand(&mut self.deck, &mut self.player_hand, &mut self.dealer_hand, self.player_bet, &mut self.rng); 
    }

    pub fn print_after_hand(&self){
        let hand_result = self.current_balance - self.previous_balance;
        match hand_result.cmp(&0.0){
            Ordering::Less => println!("You lost: {}", hand_result),
            Ordering::Equal => println!("You tied"),
            Ordering::Greater => println!("You won: {}", hand_result),
        }
        println!("Your current balance is: {}", self.current_balance);
    }
}

struct CliGame{
    game_state: GameState,
}

impl CliGame{
    pub fn new() -> CliGame{
        CliGame{
            game_state: GameState::new(),
        }
    }

    pub fn play(&mut self){
        self.game_state.nb_hands_played += 1;
        self.game_state.deal();
        self.game_state.print_before_hand();
        self.game_state.play();
        self.game_state.print_after_hand();
    }
}