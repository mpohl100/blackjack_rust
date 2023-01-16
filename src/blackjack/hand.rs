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
    pub fn new(cards: &Vec<Card>) -> BlackjackHand {
        BlackjackHand { cards: cards.clone() }
    }

    pub fn from_string(str: &str) -> BlackjackHand {
        // Implementation for fromString method
        BlackjackHand::default()
    }

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
        let mut result = 0;
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

    pub fn add_card(&mut self, card: &Card) {
        self.blackjack_hand.add_card(card);
    }
}
