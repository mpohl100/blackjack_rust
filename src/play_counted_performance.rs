
mod blackjack;

use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::blackjack::blackjack_strategy::BlackjackStrategy;
use crate::blackjack::blackjack_strategy::BlackjackStrategyVec;
use std::time::Instant;


fn play<BlackjackStrategyType>(blackjack_strategy: BlackjackStrategyType, n: i32, description: String)
where BlackjackStrategyType: BlackjackStrategyTrait + Clone + 'static
{
    let strat = blackjack::blackjack_analysis::optimize_counted(blackjack_strategy);
    let start = Instant::now();
    let result = blackjack::play_blackjack::play_blackjack(n, &strat, true);
    let duration = start.elapsed();
    // Print the elapsed time
    println!("{:?} time: {:?}", description, duration);
    println!("result: {} after {} hands", result, n);
}

fn main() {
    println!("Measure performance:");
    let n = 1000000;
    play(BlackjackStrategy::new(true), n, "HashMap".to_string());
    play(BlackjackStrategy::new(false), n, "OrderedMap".to_string());
    play(BlackjackStrategyVec::new(false), n, "ReversedVec".to_string());
    play(BlackjackStrategyVec::new(true), n, "Vec".to_string());
}
