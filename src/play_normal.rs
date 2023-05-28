mod blackjack;
mod commandline_params;

use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::blackjack::blackjack_strategy::BlackjackStrategy;
fn main() {
    let description = "Play as many hands as specified with the optimal blackjack strategy";
    let app = commandline_params::get_commandline_params("play_normal".to_string(), &description);
    let strat = blackjack::blackjack_analysis::optimize_blackjack(BlackjackStrategy::new(true), 0);
    println!("Card count {}", 0);
    println!("{}\n", strat.to_string_mat2());
    let n = commandline_params::get_number_hands(app).try_into().unwrap();
    let result = blackjack::play_blackjack::play_blackjack(n, &strat, false);
    println!("result: {} after {} hands", result, n);
}