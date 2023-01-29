mod blackjack;

use crate::blackjack::traits::BlackjackStrategyTrait;
fn main() {
    let counted_strat = blackjack::blackjack_analysis::optimize_counted();
    println!("{}", counted_strat.to_string_mat2());
    let n = 1000000;
    let result = blackjack::play_blackjack::play_blackjack(n, &counted_strat, true);
    println!("result: {} after {} hands", result, n);
}