mod blackjack;
mod commandline_params;

use threadpool::ThreadPool;

use crate::blackjack::blackjack_analysis::optimize_blackjack;
use crate::blackjack::strategy::blackjack_strategy::BlackjackStrategy;
use crate::blackjack::play_blackjack::play_blackjack;
use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::commandline_params::get_commandline_params;
use crate::commandline_params::get_play_config;
use crate::commandline_params::get_strat_config;

fn main() {
    let description = "Play as many hands as specified with the optimal blackjack strategy";
    let app = get_commandline_params("play_normal".to_string(), description);
    let strat_config = get_strat_config(app.clone());
    let thread_pool = ThreadPool::new(strat_config.nb_threads.try_into().unwrap());
    let strat = optimize_blackjack(BlackjackStrategy::new(true), &thread_pool, 0);
    println!("Card count {}", 0);
    println!("{}\n", strat.to_string_mat2());
    let mut play_config = get_play_config(app);
    play_config.play_normal = false;
    let result = play_blackjack(play_config.clone(), &strat);
    println!("result: {} after {} hands", result, play_config.nb_hands);
}
