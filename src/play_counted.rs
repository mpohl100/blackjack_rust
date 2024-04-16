use threadpool::ThreadPool;

use blackjack_rust::blackjack::strategy::blackjack_strategy_map::BlackjackStrategy;
use blackjack_rust::blackjack::traits::BlackjackStrategyTrait;
use blackjack_rust::blackjack::analysis::counted::optimize_counted;
use blackjack_rust::blackjack::play_blackjack::play_blackjack;
use blackjack_rust::commandline_params::get_commandline_params;
use blackjack_rust::commandline_params::get_play_config;
use blackjack_rust::commandline_params::get_strat_config;
fn main() {
    let description =
        "Play as many hands as possible with the optimal blackjack strategy with card counting";
    let app = get_commandline_params("play_normal".to_string(), description);
    let strat_config = get_strat_config(app.clone());
    let thread_pool = ThreadPool::new(strat_config.nb_threads.try_into().unwrap());
    let counted_strat = optimize_counted(
        BlackjackStrategy::new(true),
        strat_config,
        &thread_pool,
    );
    println!("{}", counted_strat.to_string_mat2());
    let mut play_config = get_play_config(app);
    play_config.play_normal = true;
    let result = play_blackjack(play_config.clone(), &counted_strat);
    println!("result: {} after {} hands", result, play_config.nb_hands);
}
