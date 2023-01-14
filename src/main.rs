mod blackjack;

fn main() {
    for i in -10..=10 {
        let strat = blackjack::blackjack_analysis::optimize_blackjack(i);
        println!("Card count {}", i);
        println!("{}\n", strat.to_string_mat2());
    }
}
