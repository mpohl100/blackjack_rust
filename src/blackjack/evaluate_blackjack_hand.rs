use crate::blackjack::hand::BlackjackHand;
use crate::blackjack::blackjack_points::Points;
use crate::blackjack::card::Rank;

pub fn evaluate_blackjack_hand(hand: &BlackjackHand) -> Points {
    let mut encountered_ace = false;
    let mut lower = 0;
    let mut upper = 0;
    for card in &hand.cards {
        lower += card.to_blackjack_score();
        upper += card.to_blackjack_score();
        if !encountered_ace && card.rank() == Rank::Ace {
            upper += 10;
            encountered_ace = true;
        }
    }
    Points::new(lower, upper)
}