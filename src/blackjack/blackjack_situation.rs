use crate::blackjack::blackjack_points::Points;
use crate::blackjack::card::BlackjackRank;
use crate::blackjack::traits::Allable;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct BlackjackSituation<T> {
    situation: T,
    dealer_card: BlackjackRank,
}

impl<T: Allable> BlackjackSituation<T> {
    fn new(sit: T, dealer_card: BlackjackRank) -> BlackjackSituation<T> {
        BlackjackSituation { situation: sit, dealer_card }
    }

    fn create_all() -> Vec<BlackjackSituation<T>> {
        let all_sits = T::create_all();
        let dealer_cards = BlackjackRank::create_all();
        let mut ret = vec![];
        for sit in all_sits {
            for dealer_card in dealer_cards {
                ret.push(BlackjackSituation::new(sit, dealer_card));
            }
        }
        ret
    }
}

impl<T: Default> Default for BlackjackSituation<T> {
    fn default() -> BlackjackSituation<T> {
        BlackjackSituation { situation: T::default(), dealer_card: BlackjackRank::default() }
    }
}

impl<T: Clone> Clone for BlackjackSituation<T> {
    fn clone(&self) -> BlackjackSituation<T> {
        BlackjackSituation { situation: self.situation.clone(), dealer_card: self.dealer_card.clone() }
    }
}

impl<T: Copy> Copy for BlackjackSituation<T> {}

pub type HandSituation = BlackjackSituation<Points>;
pub type SplitSituation = BlackjackSituation<BlackjackRank>;