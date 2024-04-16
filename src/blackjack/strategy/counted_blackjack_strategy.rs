use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::deck::Deck;
use crate::blackjack::strategy::blackjack_strategy_map::BlackjackStrategyData;
use crate::blackjack::traits::BlackjackStrategyTrait;
use std::collections::BTreeMap;

pub struct CountedBlackjackStrategy {
    counted_strategies: BTreeMap<i32, Box<dyn BlackjackStrategyTrait>>,
    max_count: i32,
    min_count: i32,
}

impl CountedBlackjackStrategy {
    pub fn new(data: BTreeMap<i32, Box<dyn BlackjackStrategyTrait>>) -> CountedBlackjackStrategy {
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

fn get_clamped_count(deck: &Box<dyn Deck>, min_count: i32, max_count: i32) -> i32 {
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

impl BlackjackStrategyTrait for CountedBlackjackStrategy {
    fn get_draw(&self, situation: HandSituation, deck: &Box<dyn Deck>) -> bool {
        match self
            .counted_strategies
            .get(&get_clamped_count(deck, self.min_count, self.max_count))
        {
            Some(strat) => strat.get_draw(situation, deck),
            _ => panic!("Count {} not found in counted_strategies", deck.get_count()),
        }
    }

    fn get_double_down(&self, situation: HandSituation, deck: &Box<dyn Deck>) -> bool {
        match self
            .counted_strategies
            .get(&get_clamped_count(deck, self.min_count, self.max_count))
        {
            Some(strat) => strat.get_double_down(situation, deck),
            _ => panic!("Count {} not found in counted_strategies", deck.get_count()),
        }
    }
    fn get_split(&self, situation: SplitSituation, deck: &Box<dyn Deck>) -> bool {
        match self
            .counted_strategies
            .get(&get_clamped_count(deck, self.min_count, self.max_count))
        {
            Some(strat) => strat.get_split(situation, deck),
            _ => panic!("Count {} not found in counted_strategies", deck.get_count()),
        }
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