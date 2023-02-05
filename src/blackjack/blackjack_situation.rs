use crate::blackjack::blackjack_points::Points;
use crate::blackjack::card::BlackjackRank;
use crate::blackjack::traits::Allable;
use crate::blackjack::traits::Stringable;

#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlackjackSituation<T> {
    situation: T,
    dealer_card: BlackjackRank,
}

impl<T: Copy> BlackjackSituation<T>{
    pub fn situation(&self) -> T{
        self.situation
    }

    pub fn dealer_card(&self) -> BlackjackRank{
        self.dealer_card
    }
}

impl<T: Copy + Stringable> Stringable for BlackjackSituation<T>{
    fn to_string_internal(&self) -> String{
        <T as Stringable>::to_string_internal(&self.situation()) + &"|" + &self.dealer_card().to_string_internal()
    }
}

impl<T: Allable + Clone> BlackjackSituation<T> {
    pub fn new(sit: T, dealer_card: BlackjackRank) -> BlackjackSituation<T> {
        BlackjackSituation { situation: sit, dealer_card }
    }

    fn create_all() -> Vec<BlackjackSituation<T>> {
        let all_sits = T::create_all();
        let dealer_cards = BlackjackRank::create_all();
        let mut ret = vec![];
        for sit in all_sits {
            for dealer_card in dealer_cards.clone() {
                ret.push(BlackjackSituation::new(sit.clone(), dealer_card));
            }
        }
        ret
    }
}

pub type HandSituation = BlackjackSituation<Points>;
pub type SplitSituation = BlackjackSituation<BlackjackRank>;