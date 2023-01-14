use std::collections::BTreeMap;

pub use crate::blackjack::blackjack_strategy::BlackjackStrategy;
pub use crate::blackjack::blackjack_situation::HandSituation;
pub use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::blackjack_points::Points;
use crate::blackjack::card::BlackjackRank;
use crate::blackjack::deck::CountedDeck;
use crate::blackjack::traits::Allable;
struct BlackjackGameSituation {
    pub hand_situation: Option<HandSituation>,
    pub is_draw: bool,
    pub split_situation: Option<SplitSituation>,
    pub strat: BlackjackStrategy,
}

fn optimize_situation(situation: &BlackjackGameSituation, deck: &CountedDeck) -> bool
{
    return false;
}

pub fn optimize_blackjack(card_count: i32) -> BlackjackStrategy
{
    let mut result = BlackjackStrategy::default();
    let deck = CountedDeck::new( card_count );
    // first optimize drawing
    for i in (2..=21).rev() {
        let blackjack_ranks = BlackjackRank::create_all();
        for dealer_rank in blackjack_ranks {
            let mut situation = BlackjackGameSituation {
                is_draw: true,
                strat: result.clone(),
                hand_situation: None,
                split_situation: None,
            };
            let hand_situation = HandSituation::new(
                Points::new(i, i),
                dealer_rank,
            );
            situation.hand_situation = Some(hand_situation);
            result
                .drawing_percentages
                .insert(hand_situation, optimize_situation(&situation, &deck));
            let hand_situation_upper = HandSituation::new(
                Points::new(i, i + 10),
                dealer_rank,
            );
            situation.hand_situation = Some(hand_situation_upper);
            result
                .drawing_percentages
                .insert(hand_situation_upper, optimize_situation(&situation, &deck));
        }
    }

    // then optimize double down
    for i in (2..=21).rev() {
        let blackjack_ranks = BlackjackRank::create_all();
        for dealer_rank in blackjack_ranks {
            let mut situation = BlackjackGameSituation {
                is_draw: false,
                strat: result.clone(),
                hand_situation: None,
                split_situation: None,
            };
            let hand_situation = HandSituation::new(
                Points::new(i, i),
                dealer_rank,
            );
            situation.hand_situation = Some(hand_situation);
            result
                .double_down_percentages
                .insert(hand_situation, optimize_situation(&situation, &deck));
            let hand_situation_upper = HandSituation::new(
                Points::new(i, i + 10),
                dealer_rank,
            );
            situation.hand_situation = Some(hand_situation_upper);
            result
                .double_down_percentages
                .insert(hand_situation_upper, optimize_situation(&situation, &deck));
        }
    }

    // then optimize split
    let blackjack_ranks = BlackjackRank::create_all();
    for split_rank in blackjack_ranks.clone() {
        for dealer_rank in blackjack_ranks.clone() {
            let mut situation = BlackjackGameSituation {
                is_draw: false,
                strat: result.clone(),
                hand_situation: None,
                split_situation: None,
            };
            let split_situation = SplitSituation::new(
                split_rank,
                dealer_rank,
            );
            situation.split_situation = Some(split_situation);
            result
            .split_percentages
            .insert(split_situation, optimize_situation(&situation, &deck));
        }
    }
    result
}

