mod blackjack;

fn main() {
    //for i in -10..=10 {
        let strat = blackjack::blackjack_analysis::optimize_blackjack(0);
        println!("Card count {}", 0);
        println!("{}\n", strat.to_string_mat2());
    //}
}
