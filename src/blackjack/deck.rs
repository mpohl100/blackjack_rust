pub use crate::blackjack::card::Card;
use crate::blackjack::rng::RandomNumberGenerator;
use crate::blackjack::card::BlackjackRank;
use crate::blackjack::card::Rank;

pub trait Deck{
    fn deal_card(&self, rng: &mut RandomNumberGenerator) -> Card;
}

#[derive(Default, Clone)]
pub struct CountedDeck {
    deck: Vec<Card>,
}

impl CountedDeck {
    pub fn new(count: i32) -> CountedDeck {
        let mut deck = Vec::<Card>::default();
        for i in 0..52 {
            deck.push(Card::new_with_int(i));
        }
        if count > 0 {
            let mut cnt = count;
            deck = deck.into_iter().filter(|card| {
                if cnt > 0 && BlackjackRank::new(card.rank()) == BlackjackRank::new(Rank::Ten) {
                    cnt -= 1;
                    return false;
                }
                return true;
            }).collect();
        } else if count < 0 {
            let mut cnt = -count;
            deck = deck.into_iter().filter(|card| {
                let blackjack_rank = BlackjackRank::new(card.rank());
                if cnt > 0 && blackjack_rank >= BlackjackRank::new(Rank::Deuce) && blackjack_rank <= BlackjackRank::new(Rank::Six) {
                    cnt -= 1;
                    return false;
                }
                return true;
            }).collect();
        }

        CountedDeck {
            deck,
        }
    }
}

impl Deck for CountedDeck{
    fn deal_card(&self, rng: &mut RandomNumberGenerator) -> Card {
        // implementation of the dealCard method
        let max = (self.deck.len() - 1).try_into().unwrap();
        let i = rng.fetch_uniform(0, max, 1).pop();
        let card = Card::new_with_int(match i{
            Some(value) => { value },
            None => panic!("converting usize to i32 failed"),
        });
        card
    }
}