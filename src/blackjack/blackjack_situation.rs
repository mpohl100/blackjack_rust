use crate::blackjack::blackjack_points::Points;
use crate::blackjack::card::BlackjackRank;
use crate::blackjack::card::Rank;
use crate::blackjack::traits::Allable;
use crate::blackjack::traits::Stringable;

#[derive(Debug, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

    pub fn create_all() -> Vec<BlackjackSituation<T>> {
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

#[cfg(test)]
mod hand_situation_tests {
    use super::*;
    
    #[test]
    fn test_situation() {
        let situation = Points::new(10, 20);
        let dealer_card = BlackjackRank::new(Rank::Deuce);
        let blackjack_situation = HandSituation::new(situation, dealer_card);
        
        assert_eq!(blackjack_situation.situation(), situation);
    }
    
    #[test]
    fn test_dealer_card() {
        let situation = Points::new(10, 20);
        let dealer_card = BlackjackRank::new(Rank::Deuce);
        let blackjack_situation = HandSituation::new(situation, dealer_card);
        
        assert_eq!(blackjack_situation.dealer_card(), dealer_card);
    }
    
    #[test]
    fn test_to_string_internal() {
        let situation = Points::new(10, 20);
        let dealer_card = BlackjackRank::new(Rank::Deuce);
        let blackjack_situation = HandSituation::new(situation, dealer_card);
        
        let expected_string = "10/20|2".to_string();
        assert_eq!(blackjack_situation.to_string_internal(), expected_string);
    }
    
    #[test]
    fn test_new() {
        let situation = Points::new(10, 20);
        let dealer_card = BlackjackRank::new(Rank::Deuce);
        let blackjack_situation = HandSituation::new(situation, dealer_card);
        
        assert_eq!(blackjack_situation.situation(), situation);
        assert_eq!(blackjack_situation.dealer_card(), dealer_card);
    }
    
    #[test]
    fn test_create_all() {
        let all_situations = HandSituation::create_all();
        
        // Add assertions to check the expected number of situations
        let expected_len = 30 * 10;
        assert_eq!(all_situations.len(), expected_len);
        
        let mut index = 0;
        for sit in Points::create_all(){
            for dc in BlackjackRank::create_all(){
                assert_eq!(all_situations[index], HandSituation::new(sit, dc));
                index += 1;
            }
        }
    }
}

#[cfg(test)]
mod split_situation_tests {
    use super::*;
    
    #[test]
    fn test_situation() {
        let situation = BlackjackRank::new(Rank::Ten);
        let dealer_card = BlackjackRank::new(Rank::Deuce);
        let blackjack_situation = SplitSituation::new(situation, dealer_card);
        
        assert_eq!(blackjack_situation.situation(), situation);
    }
    
    #[test]
    fn test_dealer_card() {
        let situation = BlackjackRank::new(Rank::Ten);
        let dealer_card = BlackjackRank::new(Rank::Deuce);
        let blackjack_situation = SplitSituation::new(situation, dealer_card);
        
        assert_eq!(blackjack_situation.dealer_card(), dealer_card);
    }
    
    #[test]
    fn test_to_string_internal() {
        let situation = BlackjackRank::new(Rank::Ten);
        let dealer_card = BlackjackRank::new(Rank::Deuce);
        let blackjack_situation = SplitSituation::new(situation, dealer_card);
        
        let expected_string = "10|2".to_string();
        assert_eq!(blackjack_situation.to_string_internal(), expected_string);
    }
    
    #[test]
    fn test_new() {
        let situation = BlackjackRank::new(Rank::Ten);
        let dealer_card = BlackjackRank::new(Rank::Deuce);
        let blackjack_situation = SplitSituation::new(situation, dealer_card);
        
        assert_eq!(blackjack_situation.situation(), situation);
        assert_eq!(blackjack_situation.dealer_card(), dealer_card);
    }
    
    #[test]
    fn test_create_all() {
        let all_situations = SplitSituation::create_all();
        
        // Add assertions to check the expected number of situations
        let expected_len = 10 * 10;
        assert_eq!(all_situations.len(), expected_len);
        
        let mut index = 0;
        for sit in BlackjackRank::create_all(){
            for dc in BlackjackRank::create_all(){
                assert_eq!(all_situations[index], SplitSituation::new(sit, dc));
                index += 1;
            }
        }
    }
}

