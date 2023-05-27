mod blackjack;

use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::blackjack::blackjack_strategy::BlackjackStrategy;
fn main() {
    //for i in -10..=10 {
        let strat = blackjack::blackjack_analysis::optimize_blackjack(BlackjackStrategy::new(false), 0);
        println!("Card count {}", 0);
        println!("{}\n", strat.to_string_mat2());
    //}
}
