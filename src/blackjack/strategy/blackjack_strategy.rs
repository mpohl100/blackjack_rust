use crate::blackjack::blackjack_situation::GameSituation;
use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::deck::Deck;
use crate::blackjack::strategy::blackjack_strategy_map::BlackjackStrategy;
use crate::blackjack::strategy::blackjack_strategy_map::BlackjackStrategyData;
pub use crate::blackjack::traits::BlackjackStrategyTrait;
use core::panic;
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct BlackjackStrategyCombinedOrderedHashMap {
    data: BTreeMap<GameSituation, bool>,
}

impl BlackjackStrategyCombinedOrderedHashMap {
    pub fn new() -> BlackjackStrategyCombinedOrderedHashMap {
        BlackjackStrategyCombinedOrderedHashMap {
            data: BTreeMap::new(),
        }
    }
}

impl BlackjackStrategyTrait for BlackjackStrategyCombinedOrderedHashMap {
    fn add_draw(&mut self, situation: HandSituation, do_it: bool) {
        self.data.insert(GameSituation::Draw(situation), do_it);
    }

    fn add_double_down(&mut self, situation: HandSituation, do_it: bool) {
        self.data
            .insert(GameSituation::DoubleDown(situation), do_it);
    }

    fn add_split(&mut self, situation: SplitSituation, do_it: bool) {
        self.data.insert(GameSituation::Split(situation), do_it);
    }

    fn get_draw(&self, situation: HandSituation, _deck: &Box<dyn Deck>) -> bool {
        match self.data.get(&GameSituation::Draw(situation)) {
            Some(value) => *value,
            _ => panic!("Couldn't find draw hand situation in drawing percentages"),
        }
    }

    fn get_double_down(&self, situation: HandSituation, _deck: &Box<dyn Deck>) -> bool {
        match self.data.get(&GameSituation::DoubleDown(situation)) {
            Some(value) => *value,
            _ => panic!("Couldn't find double down situation in double down percentages"),
        }
    }

    fn get_split(&self, situation: SplitSituation, _deck: &Box<dyn Deck>) -> bool {
        match self.data.get(&GameSituation::Split(situation)) {
            Some(value) => *value,
            _ => panic!("Couldn't find split situation in split percentages"),
        }
    }

    fn to_string_mat2(&self) -> String {
        let mut blackjack_strategy = BlackjackStrategy::new(false);
        for (situation, do_it) in &self.data {
            match situation {
                GameSituation::Draw(hand_situation) => {
                    blackjack_strategy.add_draw(*hand_situation, *do_it);
                }
                GameSituation::DoubleDown(hand_situation) => {
                    blackjack_strategy.add_double_down(*hand_situation, *do_it);
                }
                GameSituation::Split(split_situation) => {
                    blackjack_strategy.add_split(*split_situation, *do_it);
                }
            }
        }
        blackjack_strategy.to_string_mat2()
    }

    fn combine(&mut self, blackjack_strategy: &BlackjackStrategyData) {
        for (sit, do_it) in blackjack_strategy.drawing_percentages.iter() {
            self.add_draw(*sit, *do_it);
        }

        for (sit, do_it) in blackjack_strategy.double_down_percentages.iter() {
            self.add_double_down(*sit, *do_it);
        }

        for (sit, do_it) in blackjack_strategy.split_percentages.iter() {
            self.add_split(*sit, *do_it);
        }
    }

    fn dump(&self) -> BlackjackStrategyData {
        let mut result = BlackjackStrategyData::default();

        for (situation, do_it) in &self.data {
            match situation {
                GameSituation::Draw(hand_situation) => {
                    result.drawing_percentages.insert(*hand_situation, *do_it);
                }
                GameSituation::DoubleDown(hand_situation) => {
                    result
                        .double_down_percentages
                        .insert(*hand_situation, *do_it);
                }
                GameSituation::Split(split_situation) => {
                    result.split_percentages.insert(*split_situation, *do_it);
                }
            }
        }
        result
    }
}

#[derive(Default, Clone)]
pub struct BlackjackStrategyCombinedVec {
    data: Vec<(GameSituation, bool)>,
    reversed: bool,
}

impl BlackjackStrategyCombinedVec {
    pub fn new(reversed: bool) -> BlackjackStrategyCombinedVec {
        BlackjackStrategyCombinedVec {
            data: Vec::new(),
            reversed,
        }
    }
}

impl BlackjackStrategyTrait for BlackjackStrategyCombinedVec {
    fn add_draw(&mut self, situation: HandSituation, do_it: bool) {
        let mut iter = self.data.iter_mut();
        let res = iter.find(|x| x.0 == GameSituation::Draw(situation));
        match res {
            Some(value) => {
                value.1 = do_it;
            }
            _ => {
                self.data.push((GameSituation::Draw(situation), do_it));
            }
        }
    }

    fn add_double_down(&mut self, situation: HandSituation, do_it: bool) {
        let mut iter = self.data.iter_mut();
        let res = iter.find(|x| x.0 == GameSituation::DoubleDown(situation));
        match res {
            Some(value) => {
                value.1 = do_it;
            }
            _ => {
                self.data
                    .push((GameSituation::DoubleDown(situation), do_it));
            }
        }
    }

    fn add_split(&mut self, situation: SplitSituation, do_it: bool) {
        let mut iter = self.data.iter_mut();
        let res = iter.find(|x| x.0 == GameSituation::Split(situation));
        match res {
            Some(value) => {
                value.1 = do_it;
            }
            _ => {
                self.data.push((GameSituation::Split(situation), do_it));
            }
        }
    }

    fn get_draw(&self, situation: HandSituation, _deck: &Box<dyn Deck>) -> bool {
        if !self.reversed {
            let mut iter = self.data.iter();
            let res = iter.find(|x| x.0 == GameSituation::Draw(situation));
            match res {
                Some(value) => value.1,
                _ => panic!("Couldn't find hand situation in drawing percentages"),
            }
        } else {
            let mut iter = self.data.iter().rev();
            let res = iter.find(|x| x.0 == GameSituation::Draw(situation));
            match res {
                Some(value) => value.1,
                _ => panic!("Couldn't find hand situation in drawing percentages"),
            }
        }
    }

    fn get_double_down(&self, situation: HandSituation, _deck: &Box<dyn Deck>) -> bool {
        if !self.reversed {
            let mut iter = self.data.iter();
            let res = iter.find(|x| x.0 == GameSituation::DoubleDown(situation));
            match res {
                Some(value) => value.1,
                _ => panic!("Couldn't find hand situation in double down percentages"),
            }
        } else {
            let mut iter = self.data.iter().rev();
            let res = iter.find(|x| x.0 == GameSituation::DoubleDown(situation));
            match res {
                Some(value) => value.1,
                _ => panic!("Couldn't find hand situation in double down percentages"),
            }
        }
    }

    fn get_split(&self, situation: SplitSituation, _deck: &Box<dyn Deck>) -> bool {
        if !self.reversed {
            let mut iter = self.data.iter();
            let res = iter.find(|x| x.0 == GameSituation::Split(situation));
            match res {
                Some(value) => value.1,
                _ => panic!("Couldn't find split situation in split percentages"),
            }
        } else {
            let mut iter = self.data.iter().rev();
            let res = iter.find(|x| x.0 == GameSituation::Split(situation));
            match res {
                Some(value) => value.1,
                _ => panic!("Couldn't find split situation in split percentages"),
            }
        }
    }

    fn to_string_mat2(&self) -> String {
        let mut blackjack_strategy = BlackjackStrategy::new(false);
        for (situation, do_it) in &self.data {
            match situation {
                GameSituation::Draw(hand_situation) => {
                    blackjack_strategy.add_draw(*hand_situation, *do_it);
                }
                GameSituation::DoubleDown(hand_situation) => {
                    blackjack_strategy.add_double_down(*hand_situation, *do_it);
                }
                GameSituation::Split(split_situation) => {
                    blackjack_strategy.add_split(*split_situation, *do_it);
                }
            }
        }
        blackjack_strategy.to_string_mat2()
    }

    fn combine(&mut self, blackjack_strategy: &BlackjackStrategyData) {
        for (sit, do_it) in blackjack_strategy.drawing_percentages.iter() {
            self.add_draw(*sit, *do_it);
        }

        for (sit, do_it) in blackjack_strategy.double_down_percentages.iter() {
            self.add_double_down(*sit, *do_it);
        }

        for (sit, do_it) in blackjack_strategy.split_percentages.iter() {
            self.add_split(*sit, *do_it);
        }
    }

    fn dump(&self) -> BlackjackStrategyData {
        let mut result = BlackjackStrategyData::default();

        for (situation, do_it) in &self.data {
            match situation {
                GameSituation::Draw(hand_situation) => {
                    result.drawing_percentages.insert(*hand_situation, *do_it);
                }
                GameSituation::DoubleDown(hand_situation) => {
                    result
                        .double_down_percentages
                        .insert(*hand_situation, *do_it);
                }
                GameSituation::Split(split_situation) => {
                    result.split_percentages.insert(*split_situation, *do_it);
                }
            }
        }
        result
    }
}
