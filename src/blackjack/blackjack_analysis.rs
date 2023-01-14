
use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
pub use crate::blackjack::blackjack_strategy::BlackjackStrategy;
struct BlackjackGameSituation {
    hand_situation: Option<HandSituation>,
    is_draw: bool,
    split_situation: Option<SplitSituation>,
    strat: BlackjackStrategy,
    nb_generations: i32,
    log_level: i32,
}

fn optimize_situation(situation: &BlackjackGameSituation) -> bool
{
    return false;
}

pub fn optimize_blackjack(card_count: i32) -> BlackjackStrategy
{
    return BlackjackStrategy::default();
}

