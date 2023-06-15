pub use crate::blackjack::card::Card;
use crate::blackjack::rng::RandomNumberGenerator;
use crate::blackjack::card::BlackjackRank;
use crate::blackjack::card::Rank;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub trait Deck{
    fn deal_card(&mut self, rng: &mut RandomNumberGenerator) -> Card;
    fn get_count(&self) -> i32;
    fn get_nb_cards(&self) -> i32;
}

#[derive(Default, Clone)]
pub struct CountedDeck {
    deck: Vec<Card>,
    count: i32,
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
                if cnt > 0 && BlackjackRank::new(card.rank()) == BlackjackRank::new(Rank::Ten) || BlackjackRank::new(card.rank()) == BlackjackRank::new(Rank::Ace) {
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
            count
        }
    }
}

impl Deck for CountedDeck{
    fn deal_card(&mut self, rng: &mut RandomNumberGenerator) -> Card {
        let max = (self.deck.len() - 1).try_into().unwrap();
        let i = rng.fetch_uniform(0, max, 1).pop();
        let card = Card::new_with_int(match i{
            Some(value) => { value },
            None => panic!("converting usize to i32 failed"),
        });
        card
    }

    fn get_count(&self) -> i32 {
        self.count
    }

    fn get_nb_cards(&self) -> i32 {
        52
    }
}

pub struct EightDecks{
    decks: Vec<Card>,
    count: i32,
}

impl EightDecks{
    pub fn new() -> EightDecks{
        let mut deck = EightDecks{decks: Vec::<Card>::new(), count: 0};
        deck.init();
        deck
    }

    fn init(&mut self){
        self.decks = Vec::<Card>::new();
        for _ in 0..8{
            for i in 0..52{
                let card = Card::new_with_int(i);
                //println!("{:?} {:?}", card.rank().to_string_internal(), card.suit().to_string_internal());
                self.decks.push(card);
            }
        }
        //println!();
        self.decks.shuffle(&mut thread_rng());
        //for card in &self.decks{
        //    println!("{:?} {:?}", card.rank().to_string_internal(), card.suit().to_string_internal());
        //}
        self.count = 0;
    }
}

impl Deck for EightDecks{
    fn deal_card(&mut self, _rng: &mut RandomNumberGenerator) -> Card {
        if self.decks.is_empty(){
            self.init();
        }
        let card;
        match self.decks.pop(){
            Some(value) => {card = value},
            _ => panic!("found empty deck"),
        }
        if vec![Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace].contains(&card.rank()){
            self.count += 1;
        }
        else if vec![Rank::Deuce, Rank::Three, Rank::Four, Rank::Five, Rank::Six].contains(&card.rank()) {
            self.count -= 1;
        }
        card
    }

    fn get_count(&self) -> i32 {
        self.count
    }

    fn get_nb_cards(&self) -> i32 {
        self.decks.len().try_into().unwrap()
    }
}