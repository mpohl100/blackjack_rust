pub use crate::blackjack::card::Card;
pub use rand::Rng;

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

    pub fn deal_card<R: Rng>(&self, rng: &mut R) -> Card {
        // implementation of the dealCard method
        Card::default()
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}