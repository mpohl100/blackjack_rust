mod blackjack;
mod commandline_params;



use threadpool::ThreadPool;

use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::blackjack::blackjack_strategy::BlackjackStrategy;
fn main() {
    let description = "Play as many hands as possible with the optimal blackjack strategy with card counting";
    let app = commandline_params::get_commandline_params("play_normal".to_string(), &description);
    let strat_config = commandline_params::get_strat_config(app.clone());
    let thread_pool = ThreadPool::new(strat_config.nb_threads.try_into().unwrap());
    let counted_strat = blackjack::blackjack_analysis::optimize_counted(BlackjackStrategy::new(true), strat_config, &thread_pool);
    println!("{}", counted_strat.to_string_mat2());
    let mut play_config = commandline_params::get_play_config(app);
    play_config.play_normal = true;
    let result = blackjack::play_blackjack::play_blackjack(play_config.clone(), &counted_strat);
    println!("result: {} after {} hands", result, play_config.nb_hands);
}