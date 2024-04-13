use crate::blackjack::card::Card;


use crate::blackjack::card::BlackjackRank;
use crate::blackjack::deck::Deck;
use crate::blackjack::evaluate_blackjack_hand::evaluate_blackjack_hand;

use super::rng::RandomNumberGenerator;

#[derive(Default, Clone)]
pub struct BlackjackHand {
    pub cards: Vec<Card>,
}

impl BlackjackHand{
    pub fn add_card(&mut self, card: &Card) {
        self.cards.push(card.clone());
    }

    pub fn is_pair(&self) -> bool {
        if self.cards.len() != 2{
            false
        }
        else{
            self.cards[0].rank() == self.cards[1].rank()
        }
    } 
}

#[derive(Default, Clone)]
pub struct PlayerHand {
    blackjack_hand: BlackjackHand,
}

impl PlayerHand {
    pub fn new(cards: &Vec<Card>) -> PlayerHand {
        PlayerHand { blackjack_hand: BlackjackHand { cards: cards.clone() } }
    }

    pub fn is_pair(&self) -> bool {
        self.blackjack_hand.is_pair()
    }

    pub fn add_card(&mut self, card: &Card) {
        self.blackjack_hand.add_card(card);
    }

    pub fn get_cards(&self) -> Vec<Card>{
        self.blackjack_hand.cards.clone()
    }

    pub fn get_blackjack_hand(&self) -> BlackjackHand{
        self.blackjack_hand.clone()
    }
}

#[derive(Default, Clone)]
pub struct DealerHand {
    blackjack_hand: BlackjackHand,
}

impl DealerHand {
    pub fn new(cards: &Vec<Card>) -> DealerHand {
        DealerHand { blackjack_hand: BlackjackHand { cards: cards.clone() } }
    }

    fn get_cards(&self) -> Vec<Card>{
        self.blackjack_hand.cards.clone()
    }

    pub fn play(&mut self, deck: &mut Box<dyn Deck>, rng: &mut RandomNumberGenerator) -> i32 {
        let draw_until = 17;
        let result;
        loop {
            let points = evaluate_blackjack_hand(&self.blackjack_hand);
            if points.upper() >= draw_until && points.upper() <= 21 {
                result = points.upper();
                break;
            }
            if points.lower() >= draw_until {
                result = points.lower();
                break;
            }
            self.blackjack_hand.add_card(&deck.deal_card(rng));
        }
        if result > 21 {
            return -1;
        }
        return result;
    }

    pub fn open_card(&self) -> BlackjackRank {
        BlackjackRank::new(self.get_cards()[0].rank())
    }
}

#[cfg(test)]
mod blackjack_hand_tests {
    use super::*;

    #[test]
    fn test_add_card() {
        let mut hand = BlackjackHand::default();
        let card = Card::new(Rank::Ace, Suit::Hearts);
        hand.add_card(&card);

        assert_eq!(hand.cards.len(), 1);
        assert_eq!(hand.cards[0], card);
    }

    #[test]
    fn test_is_pair() {
        let mut hand = BlackjackHand::default();

        // Test when hand has 2 cards with the same rank
        hand.add_card(&Card::new(Rank::Ace, Suit::Hearts));
        hand.add_card(&Card::new(Rank::Ace, Suit::Diamonds));
        assert_eq!(hand.is_pair(), true);

        // Test when hand has 2 cards with different ranks
        hand.cards.clear();
        hand.add_card(&Card::new(Rank::King, Suit::Spades));
        hand.add_card(&Card::new(Rank::Queen, Suit::Clubs));
        assert_eq!(hand.is_pair(), false);

        // Test when hand has more than 2 cards
        hand.cards.clear();
        hand.add_card(&Card::new(Rank::Deuce, Suit::Hearts));
        hand.add_card(&Card::new(Rank::Deuce, Suit::Diamonds));
        hand.add_card(&Card::new(Rank::Deuce, Suit::Spades));
        assert_eq!(hand.is_pair(), false);
    }
}

#[cfg(test)]
mod player_hand_tests {
    use super::*;

    #[test]
    fn test_new() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Diamonds),
        ];
        let player_hand = PlayerHand::new(&cards);

        assert_eq!(player_hand.get_cards(), cards);
    }

    #[test]
    fn test_is_pair() {
        let mut player_hand = PlayerHand::default();

        // Test when hand has 2 cards with the same rank
        player_hand.add_card(&Card::new(Rank::Ace, Suit::Hearts));
        player_hand.add_card(&Card::new(Rank::Ace, Suit::Diamonds));
        assert_eq!(player_hand.is_pair(), true);

        // Test when hand has 2 cards with different ranks
        player_hand.add_card(&Card::new(Rank::King, Suit::Spades));
        player_hand.add_card(&Card::new(Rank::Queen, Suit::Clubs));
        assert_eq!(player_hand.is_pair(), false);

        // Test when hand has more than 2 cards
        player_hand.add_card(&Card::new(Rank::Deuce, Suit::Hearts));
        player_hand.add_card(&Card::new(Rank::Deuce, Suit::Diamonds));
        player_hand.add_card(&Card::new(Rank::Deuce, Suit::Spades));
        assert_eq!(player_hand.is_pair(), false);
    }

    #[test]
    fn test_add_card() {
        let mut player_hand = PlayerHand::default();
        let card = Card::new(Rank::Ace, Suit::Hearts);
        player_hand.add_card(&card);

        assert_eq!(player_hand.get_cards().len(), 1);
        assert_eq!(player_hand.get_cards()[0], card);
    }

    #[test]
    fn test_get_cards() {
        let cards = vec![
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Diamonds),
        ];
        let player_hand = PlayerHand::new(&cards);

        assert_eq!(player_hand.get_cards(), cards);
    }

    #[test]
    fn test_get_blackjack_hand() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Diamonds),
        ];
        let player_hand = PlayerHand::new(&cards);
        let blackjack_hand = player_hand.get_blackjack_hand();

        assert_eq!(blackjack_hand.cards, cards);
    }
}

#[cfg(test)]
mod dealer_hand_tests {
    use super::*;

    pub struct DeterministicDeck {
        cards: Vec<Card>,
        current_index: usize,
    }
    
    impl DeterministicDeck {
        pub fn new(cards: Vec<Card>) -> DeterministicDeck {
            DeterministicDeck {
                cards,
                current_index: 0,
            }
        }
    }
    
    impl Deck for DeterministicDeck {
        fn deal_card(&mut self, _rng: &mut RandomNumberGenerator) -> Card {
            let card = self.cards[self.current_index];
            self.current_index += 1;
            card
        }

        fn get_count(&self) -> i32{
            0
        }

        fn get_nb_cards(&self) -> i32{
            (self.cards.len() - self.current_index).try_into().unwrap()
        }
    }

    #[test]
    fn test_new() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Diamonds),
        ];
        let dealer_hand = DealerHand::new(&cards);

        assert_eq!(dealer_hand.get_cards(), cards);
    }

    #[test]
    fn test_play() {
        // Test case: Hand with cards that add up to 16
        let mut deck: Box<dyn Deck> = Box::new(DeterministicDeck::new(vec![
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Spades),
        ]));
        let mut rng = RandomNumberGenerator::new();
        let mut dealer_hand = DealerHand::new(&vec![]);

        let result = dealer_hand.play(&mut deck, &mut rng);

        assert_eq!(result, -1); // Dealer busts

        // Test case: Hand with cards that add up to soft 17
        let mut deck: Box<dyn Deck> = Box::new(DeterministicDeck::new(vec![
            Card::new(Rank::Six, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Diamonds),
        ]));
        let mut rng = RandomNumberGenerator::new();
        let mut dealer_hand = DealerHand::new(&vec![]);

        let result = dealer_hand.play(&mut deck, &mut rng);

        assert_eq!(result, 17); // Dealer stands at 17

        // Test case: Hand with cards that add up to hard 17
        let mut deck: Box<dyn Deck> = Box::new(DeterministicDeck::new(vec![
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Diamonds),
        ]));
        let mut rng = RandomNumberGenerator::new();
        let mut dealer_hand = DealerHand::new(&vec![]);

        let result = dealer_hand.play(&mut deck, &mut rng);

        assert_eq!(result, 17); // Dealer stands at 17
    }

    #[test]
    fn test_open_card() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Deuce, Suit::Diamonds),
        ];
        let dealer_hand = DealerHand::new(&cards);
        let open_card = dealer_hand.open_card();

        assert_eq!(open_card, BlackjackRank::new(Rank::Ace));
    }
}



