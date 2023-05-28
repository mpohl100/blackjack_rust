
mod blackjack;
mod commandline_params;

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
    let description = "Play as many hands as specified with the optimal blackjack strategy with card counting and compare the performance of the storage approaches of the blackjack strategy";
    let app = commandline_params::get_commandline_params("play_normal".to_string(), &description);
    println!("Measure performance:");
    let n = commandline_params::get_number_hands(app).try_into().unwrap();
    play(BlackjackStrategy::new(true), n, "HashMap".to_string());
    play(BlackjackStrategy::new(false), n, "OrderedMap".to_string());
    play(BlackjackStrategyVec::new(false), n, "ReversedVec".to_string());
    play(BlackjackStrategyVec::new(true), n, "Vec".to_string());
}
