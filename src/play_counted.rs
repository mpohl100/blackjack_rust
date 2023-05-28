mod blackjack;
mod commandline_params;

use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::blackjack::blackjack_strategy::BlackjackStrategy;
fn main() {
    let description = "Play as many hands as possible with the optimal blackjack strategy with card counting";
    let app = commandline_params::get_commandline_params("play_normal".to_string(), &description);
    let counted_strat = blackjack::blackjack_analysis::optimize_counted(BlackjackStrategy::new(true));
    println!("{}", counted_strat.to_string_mat2());
    let n = commandline_params::get_number_hands(app).try_into().unwrap();
    let result = blackjack::play_blackjack::play_blackjack(n, &counted_strat, true);
    println!("result: {} after {} hands", result, n);
}