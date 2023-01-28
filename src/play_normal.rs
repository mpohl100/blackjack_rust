mod blackjack;

fn main() {
    let strat = blackjack::blackjack_analysis::optimize_blackjack(0);
    println!("Card count {}", 0);
    println!("{}\n", strat.to_string_mat2());
    let n = 1000000;
    let result = blackjack::play_blackjack::play_blackjack(n, &strat);
    println!("result: {} after {} hands", result, n);
}