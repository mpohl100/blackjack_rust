
mod blackjack;
mod commandline_params;

use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::blackjack::blackjack_strategy::BlackjackStrategy;
use crate::blackjack::blackjack_strategy::BlackjackStrategyVec;
use crate::commandline_params::PlayConfiguration;
use crate::commandline_params::StrategyConfiguration;
use std::time::Instant;


fn play<BlackjackStrategyType>(blackjack_strategy: BlackjackStrategyType, play_config: PlayConfiguration, strat_config: StrategyConfiguration, description: String)
where BlackjackStrategyType: BlackjackStrategyTrait + Clone + Send + 'static
{
    let strat_start = Instant::now();
    let strat = blackjack::blackjack_analysis::optimize_blackjack(blackjack_strategy, strat_config.clone(), 0);
    let strat_duration = strat_start.elapsed();
    let start = Instant::now();
    let result = blackjack::play_blackjack::play_blackjack(play_config.clone(), &strat);
    let duration = start.elapsed();
    // Print the elapsed time
    println!("strategy time: {:?} on {:?} threads", strat_duration, strat_config.nb_threads);
    println!("{:?} time: {:?}", description, duration);
    println!("result: {} after {} hands", result, play_config.nb_hands);
}

fn main() {
    let description = "Play as many hands as specified with the optimal blackjack strategy and compare the performance of the storage approaches of the blackjack strategy";
    let app = commandline_params::get_commandline_params("play_normal".to_string(), &description);
    println!("Measure performance:");
    let mut play_config = commandline_params::get_play_config(app.clone());
    play_config.play_normal = false;
    let strat_config = commandline_params::get_strat_config(app);
    play(BlackjackStrategy::new(true), play_config.clone(), strat_config.clone(), "HashMap".to_string());
    play(BlackjackStrategy::new(false), play_config.clone(), strat_config.clone(),"OrderedMap".to_string());
    play(BlackjackStrategyVec::new(false), play_config.clone(), strat_config.clone(),"ReversedVec".to_string());
    play(BlackjackStrategyVec::new(true), play_config, strat_config,"Vec".to_string());
}
