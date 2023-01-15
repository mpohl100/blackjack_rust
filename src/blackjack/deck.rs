pub use crate::blackjack::card::Card;
use crate::blackjack::rng::RandomNumberGenerator;

pub trait Deck{
    fn deal_card(&self, rng: &mut RandomNumberGenerator) -> Card;
}

#[derive(Default, Clone)]
pub struct CountedDeck {
    count: i32,
    deck: Vec<Card>,
}

impl CountedDeck {
    pub fn new(count: i32) -> CountedDeck {
        CountedDeck {
            count,
            deck: vec![],
        }
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}

impl Deck for CountedDeck{
    fn deal_card(&self, rng: &mut RandomNumberGenerator) -> Card {
        // implementation of the dealCard method
        Card::default()
    }
}