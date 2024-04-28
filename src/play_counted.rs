use blackjack_rust::blackjack::analysis::counted::optimize_counted;
use blackjack_rust::blackjack::play_blackjack::play_blackjack;
use blackjack_rust::blackjack::strategy::blackjack_strategy_map::BlackjackStrategy;
use blackjack_rust::blackjack::traits::BlackjackStrategyTrait;
use blackjack_rust::blackjack::traits::WrappedStrategy;
use blackjack_rust::commandline_params::get_commandline_params;
use blackjack_rust::commandline_params::get_play_config;
use blackjack_rust::commandline_params::get_strat_config;

#[tokio::main]
async fn main() {
    let description =
        "Play as many hands as possible with the optimal blackjack strategy with card counting";
    let app = get_commandline_params("play_normal".to_string(), description);
    let strat_config = get_strat_config(app.clone());
    let mut counted_strat =
        optimize_counted(BlackjackStrategy::new(true)).await;
    println!("{}", counted_strat.to_string_mat2().await);
    let mut play_config = get_play_config(app);
    play_config.play_normal = true;
    let mut wrapped_strat = WrappedStrategy::new(counted_strat);
    let result = play_blackjack(play_config.clone(), &mut wrapped_strat).await;
    println!("result: {} after {} hands", result, play_config.nb_hands);
}
