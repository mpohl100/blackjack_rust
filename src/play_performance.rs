mod blackjack;
mod commandline_params;

use crate::blackjack::analysis::blackjack_analysis::optimize_blackjack;
use crate::blackjack::play_blackjack::play_blackjack;
use crate::blackjack::strategy::blackjack_strategy_combined_vec::BlackjackStrategyCombinedVec;
use crate::blackjack::strategy::blackjack_strategy_combined_hash_map::BlackjackStrategyCombinedHashMap;
use crate::blackjack::strategy::blackjack_strategy_combined_ordered_hash_map::BlackjackStrategyCombinedOrderedHashMap;
use crate::blackjack::strategy::blackjack_strategy_map::BlackjackStrategy;
use crate::blackjack::strategy::blackjack_strategy_vec::BlackjackStrategyVec;
use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::commandline_params::PlayConfiguration;
use crate::commandline_params::StrategyConfiguration;
use crate::commandline_params::{get_commandline_params, get_play_config, get_strat_config};
use std::time::Instant;
use threadpool::ThreadPool;

pub fn play<BlackjackStrategyType>(
    blackjack_strategy: BlackjackStrategyType,
    play_config: PlayConfiguration,
    strat_config: StrategyConfiguration,
    thread_pool: &ThreadPool,
    description: String,
) -> f64
where
    BlackjackStrategyType: BlackjackStrategyTrait + Clone + Send + 'static,
{
    let strat_start = Instant::now();
    let strat = optimize_blackjack(blackjack_strategy, thread_pool, 0);
    let strat_duration = strat_start.elapsed();
    let start = Instant::now();
    let result = play_blackjack(play_config.clone(), &strat);
    let duration = start.elapsed();
    // Print the elapsed time
    println!(
        "strategy time: {:?} on {:?} threads",
        strat_duration, strat_config.nb_threads
    );
    println!("{:?} time: {:?}", description, duration);
    println!("result: {} after {} hands", result, play_config.nb_hands);
    result
}

fn main() {
    let description = "Play as many hands as specified with the optimal blackjack strategy and compare the performance of the storage approaches of the blackjack strategy";
    let app = get_commandline_params("play_normal".to_string(), description);
    println!("Measure performance:");
    let mut play_config = get_play_config(app.clone());
    play_config.play_normal = false;
    let strat_config = get_strat_config(app);
    let thread_pool = ThreadPool::new(strat_config.nb_threads.try_into().unwrap());
    play(
        BlackjackStrategy::new(true),
        play_config.clone(),
        strat_config.clone(),
        &thread_pool,
        "HashMap".to_string(),
    );
    play(
        BlackjackStrategy::new(false),
        play_config.clone(),
        strat_config.clone(),
        &thread_pool,
        "OrderedMap".to_string(),
    );
    play(
        BlackjackStrategyVec::new(false),
        play_config.clone(),
        strat_config.clone(),
        &thread_pool,
        "ReversedVec".to_string(),
    );
    play(
        BlackjackStrategyVec::new(true),
        play_config.clone(),
        strat_config.clone(),
        &thread_pool,
        "Vec".to_string(),
    );
    play(
        BlackjackStrategyCombinedHashMap::new(),
        play_config.clone(),
        strat_config.clone(),
        &thread_pool,
        "CombinedHashMap".to_string(),
    );
    play(
        BlackjackStrategyCombinedOrderedHashMap::new(),
        play_config.clone(),
        strat_config.clone(),
        &thread_pool,
        "CombinedOrderedHashMap".to_string(),
    );
    play(
        BlackjackStrategyCombinedVec::new(false),
        play_config.clone(),
        strat_config.clone(),
        &thread_pool,
        "CombinedVec".to_string(),
    );
    play(
        BlackjackStrategyCombinedVec::new(true),
        play_config,
        strat_config,
        &thread_pool,
        "CombinedReversedVec".to_string(),
    );
}
