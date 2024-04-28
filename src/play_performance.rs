use blackjack_rust::blackjack::analysis::blackjack_analysis::optimize_blackjack;
use blackjack_rust::blackjack::play_blackjack::play_blackjack;
use blackjack_rust::blackjack::strategy::blackjack_strategy_combined_hash_map::BlackjackStrategyCombinedHashMap;
use blackjack_rust::blackjack::strategy::blackjack_strategy_combined_ordered_hash_map::BlackjackStrategyCombinedOrderedHashMap;
use blackjack_rust::blackjack::strategy::blackjack_strategy_combined_vec::BlackjackStrategyCombinedVec;
use blackjack_rust::blackjack::strategy::blackjack_strategy_map::BlackjackStrategy;
use blackjack_rust::blackjack::strategy::blackjack_strategy_vec::BlackjackStrategyVec;
use blackjack_rust::blackjack::traits::BlackjackStrategyTrait;
use blackjack_rust::blackjack::traits::WrappedStrategy;
use blackjack_rust::commandline_params::PlayConfiguration;
use blackjack_rust::commandline_params::StrategyConfiguration;
use blackjack_rust::commandline_params::{
    get_commandline_params, get_play_config, get_strat_config,
};
use std::time::Instant;

pub async fn play<BlackjackStrategyType>(
    blackjack_strategy: BlackjackStrategyType,
    play_config: PlayConfiguration,
    strat_config: StrategyConfiguration,
    description: String,
) -> f64
where
    BlackjackStrategyType: BlackjackStrategyTrait + Clone + Send + 'static,
{
    let strat_start = Instant::now();
    let mut strat = optimize_blackjack(WrappedStrategy::new(blackjack_strategy), 0).await;
    let strat_duration = strat_start.elapsed();
    let start = Instant::now();
    let result = play_blackjack(play_config.clone(), &mut strat).await;
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

#[tokio::main]
async fn main() {
    let description = "Play as many hands as specified with the optimal blackjack strategy and compare the performance of the storage approaches of the blackjack strategy";
    let app = get_commandline_params("play_normal".to_string(), description);
    println!("Measure performance:");
    let mut play_config = get_play_config(app.clone());
    play_config.play_normal = false;
    let strat_config = get_strat_config(app);
    play(
        BlackjackStrategy::new(true),
        play_config.clone(),
        strat_config.clone(),
        "HashMap".to_string(),
    ).await;
    play(
        BlackjackStrategy::new(false),
        play_config.clone(),
        strat_config.clone(),
        "OrderedMap".to_string(),
    ).await;
    play(
        BlackjackStrategyVec::new(false),
        play_config.clone(),
        strat_config.clone(),
        "ReversedVec".to_string(),
    ).await;
    play(
        BlackjackStrategyVec::new(true),
        play_config.clone(),
        strat_config.clone(),
        "Vec".to_string(),
    ).await;
    play(
        BlackjackStrategyCombinedHashMap::new(),
        play_config.clone(),
        strat_config.clone(),
        "CombinedHashMap".to_string(),
    ).await;
    play(
        BlackjackStrategyCombinedOrderedHashMap::new(),
        play_config.clone(),
        strat_config.clone(),
        "CombinedOrderedHashMap".to_string(),
    ).await;
    play(
        BlackjackStrategyCombinedVec::new(false),
        play_config.clone(),
        strat_config.clone(),
        "CombinedVec".to_string(),
    ).await;
    play(
        BlackjackStrategyCombinedVec::new(true),
        play_config,
        strat_config,
        "CombinedReversedVec".to_string(),
    ).await;
}
