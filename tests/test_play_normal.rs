use std::time::Instant;

use blackjack_rust::{
    blackjack::analysis::blackjack_analysis::optimize_blackjack,
    blackjack::blackjack_configuration::PlayConfiguration,
    blackjack::blackjack_configuration::StrategyConfiguration,
    blackjack::play_blackjack::play_blackjack,
    blackjack::strategy::blackjack_strategy_map::BlackjackStrategy,
    blackjack::traits::BlackjackStrategyTrait, blackjack::traits::WrappedStrategy,
};

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

#[tokio::test]
async fn play_blackjack_normal() {
    let play_configuration = PlayConfiguration {
        nb_hands: 100000,
        play_normal: true,
    };
    let strategy_configuration = StrategyConfiguration { nb_threads: 4 };
    let result_hash_map = play(
        BlackjackStrategy::new(true),
        play_configuration.clone(),
        strategy_configuration.clone(),
        "HashMap".to_string(),
    )
    .await;
    //let result_ordinary_map = play(BlackjackStrategy::new(false), play_configuration.clone(), strategy_configuration.clone(), &thread_pool,"OrderedMap".to_string());
    //let result_vec = play(BlackjackStrategyVec::new(false), play_configuration.clone(), strategy_configuration.clone(), &thread_pool,"ReversedVec".to_string());
    //let result_vec_reversed = play(BlackjackStrategyVec::new(true), play_configuration, strategy_configuration.clone(), &thread_pool,"Vec".to_string());
    assert!(result_hash_map > -1000.0);
    //assert!(result_ordinary_map > -5000.0);
    //assert!(result_vec > -5000.0);
    //assert!(result_vec_reversed > -5000.0);
}
