use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::deck::WrappedDeck;
use crate::blackjack::strategy::blackjack_strategy_map::BlackjackStrategyData;
use crate::blackjack::traits::BlackjackGame;
use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::blackjack::traits::WrappedStrategy;
use std::collections::BTreeMap;
use async_trait::async_trait;

#[derive(Default, Clone)]
pub struct CountedBlackjackStrategy {
    counted_strategies: BTreeMap<i32, WrappedStrategy>,
    max_count: i32,
    min_count: i32,
}

impl CountedBlackjackStrategy {
    pub fn new(data: BTreeMap<i32, WrappedStrategy>) -> CountedBlackjackStrategy {
        let max_count = match data.keys().next_back() {
            Some(value) => *value,
            _ => panic!("max_count not found in data"),
        };

        let min_count = match data.keys().next() {
            Some(value) => *value,
            _ => panic!("min_count not found in data"),
        };
        CountedBlackjackStrategy {
            counted_strategies: data,
            max_count,
            min_count,
        }
    }
}

fn get_clamped_count(deck: &mut WrappedDeck, min_count: i32, max_count: i32) -> i32 {
    let nb_cards = deck.get_nb_cards();
    let count = deck.get_count();
    let ratio = (count as f64) / (nb_cards as f64);
    let count = (ratio * 52.0) as i32;
    if count > max_count {
        max_count
    } else if count < min_count {
        min_count
    } else {
        count
    }
}

#[async_trait]
impl BlackjackGame for CountedBlackjackStrategy {
    async fn get_draw(&mut self, situation: HandSituation, deck: &mut WrappedDeck) -> bool {
        match self.counted_strategies.get_mut(&get_clamped_count(
            deck,
            self.min_count,
            self.max_count,
        )) {
            Some(strat) => strat.get_draw(situation, deck).await,
            _ => panic!("Count {} not found in counted_strategies", deck.get_count()),
        }
    }

    async fn get_double_down(&mut self, situation: HandSituation, deck: &mut WrappedDeck) -> bool {
        match self.counted_strategies.get_mut(&get_clamped_count(
            deck,
            self.min_count,
            self.max_count,
        )) {
            Some(strat) => strat.get_double_down(situation, deck).await,
            _ => panic!("Count {} not found in counted_strategies", deck.get_count()),
        }
    }
    async fn get_split(&mut self, situation: SplitSituation, deck: &mut WrappedDeck) -> bool {
        match self.counted_strategies.get_mut(&get_clamped_count(
            deck,
            self.min_count,
            self.max_count,
        )) {
            Some(strat) => strat.get_split(situation, deck).await,
            _ => panic!("Count {} not found in counted_strategies", deck.get_count()),
        }
    }
}

impl BlackjackStrategyTrait for CountedBlackjackStrategy {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn upcast_mut(&mut self) -> &mut dyn BlackjackGame {
        self
    }

    fn add_draw(&mut self, _situation: HandSituation, _do_it: bool) {
        unimplemented!()
    }

    fn add_double_down(&mut self, _situation: HandSituation, _do_it: bool) {
        unimplemented!()
    }

    fn add_split(&mut self, _situation: SplitSituation, _do_it: bool) {
        unimplemented!()
    }

    fn to_string_mat2(&self) -> String {
        let mut ret = String::new();
        for (count, strat) in &self.counted_strategies {
            ret.push_str(&("Count ".to_owned() + &count.to_string()));
            ret.push_str(&("Strategy: ".to_owned() + &strat.to_string_mat2() + "\n\n"));
        }
        ret
    }

    fn combine(&mut self, _blackjack_strategy: &BlackjackStrategyData) {
        unimplemented!()
    }

    fn dump(&self) -> BlackjackStrategyData {
        unimplemented!()
    }
}
