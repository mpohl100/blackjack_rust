mod blackjack;

use crate::blackjack::traits::BlackjackStrategyTrait;
fn main() {
    let strat = blackjack::blackjack_analysis::optimize_blackjack(0);
    println!("Card count {}", 0);
    println!("{}\n", strat.to_string_mat2());
    let n = 10000000;
    let result = blackjack::play_blackjack::play_blackjack(n, &strat, false);
    println!("result: {} after {} hands", result, n);
}