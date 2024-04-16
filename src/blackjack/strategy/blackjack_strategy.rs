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

#[derive(Default, Clone, Copy)]
struct SituationStrategy<T> {
    pub situation: T,
    pub do_it: bool,
}

type HandSituationStrategy = SituationStrategy<HandSituation>;
type SplitSituationStrategy = SituationStrategy<SplitSituation>;

#[derive(Default, Clone)]
struct BlackjackStrategyDataVec {
    pub drawing_percentages: Vec<HandSituationStrategy>,
    pub double_down_percentages: Vec<HandSituationStrategy>,
    pub split_percentages: Vec<SplitSituationStrategy>,
}

#[derive(Default, Clone)]
pub struct BlackjackStrategyVec {
    data: BlackjackStrategyDataVec,
    reversed: bool,
}

impl BlackjackStrategyVec {
    pub fn new(reversed: bool) -> BlackjackStrategyVec {
        BlackjackStrategyVec {
            data: BlackjackStrategyDataVec::default(),
            reversed,
        }
    }
}

impl BlackjackStrategyTrait for BlackjackStrategyVec {
    fn get_draw(&self, situation: HandSituation, _deck: &Box<dyn Deck>) -> bool {
        if !self.reversed {
            let mut iter = self.data.drawing_percentages.iter();
            let res = iter.find(|x| x.situation == situation);
            match res {
                Some(value) => value.do_it,
                _ => panic!("Couldn't find hand situation in drawing percentages"),
            }
        } else {
            let mut iter = self.data.drawing_percentages.iter().rev();
            let res = iter.find(|x| x.situation == situation);
            match res {
                Some(value) => value.do_it,
                _ => panic!("Couldn't find hand situation in drawing percentages"),
            }
        }
    }

    fn get_double_down(&self, situation: HandSituation, _deck: &Box<dyn Deck>) -> bool {
        if !self.reversed {
            let mut iter = self.data.double_down_percentages.iter();
            let res = iter.find(|x| x.situation == situation);
            match res {
                Some(value) => value.do_it,
                _ => panic!("Couldn't find hand situation in double down percentages"),
            }
        } else {
            let mut iter = self.data.double_down_percentages.iter().rev();
            let res = iter.find(|x| x.situation == situation);
            match res {
                Some(value) => value.do_it,
                _ => panic!("Couldn't find hand situation in double down percentages"),
            }
        }
    }

    fn get_split(&self, situation: SplitSituation, _deck: &Box<dyn Deck>) -> bool {
        if !self.reversed {
            let mut iter = self.data.split_percentages.iter();
            let res = iter.find(|x| x.situation == situation);
            match res {
                Some(value) => value.do_it,
                _ => panic!("Couldn't find split situation in split percentages"),
            }
        } else {
            let mut iter = self.data.split_percentages.iter().rev();
            let res = iter.find(|x| x.situation == situation);
            match res {
                Some(value) => value.do_it,
                _ => panic!("Couldn't find split situation in split percentages"),
            }
        }
    }

    fn add_draw(&mut self, situation: HandSituation, do_it: bool) {
        let mut iter = self.data.drawing_percentages.iter_mut();
        let res = iter.find(|x| x.situation == situation);
        match res {
            Some(value) => {
                value.do_it = do_it;
            }
            _ => self
                .data
                .drawing_percentages
                .push(HandSituationStrategy { situation, do_it }),
        }
    }

    fn add_double_down(&mut self, situation: HandSituation, do_it: bool) {
        let mut iter = self.data.double_down_percentages.iter_mut();
        let res = iter.find(|x| x.situation == situation);
        match res {
            Some(value) => {
                value.do_it = do_it;
            }
            _ => self
                .data
                .double_down_percentages
                .push(HandSituationStrategy { situation, do_it }),
        }
    }

    fn add_split(&mut self, situation: SplitSituation, do_it: bool) {
        let mut iter = self.data.split_percentages.iter_mut();
        let res = iter.find(|x| x.situation == situation);
        match res {
            Some(value) => {
                value.do_it = do_it;
            }
            _ => self
                .data
                .split_percentages
                .push(SplitSituationStrategy { situation, do_it }),
        }
    }

    fn to_string_mat2(&self) -> String {
        let mut blackjack_strategy_sorted = BlackjackStrategy::new(false);
        for situation_strategy in &self.data.drawing_percentages {
            blackjack_strategy_sorted
                .add_draw(situation_strategy.situation, situation_strategy.do_it);
        }
        for situation_strategy in &self.data.double_down_percentages {
            blackjack_strategy_sorted
                .add_double_down(situation_strategy.situation, situation_strategy.do_it);
        }
        for situation_strategy in &self.data.split_percentages {
            blackjack_strategy_sorted
                .add_split(situation_strategy.situation, situation_strategy.do_it);
        }
        blackjack_strategy_sorted.to_string_mat2()
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

        for hand_situation in &self.data.drawing_percentages {
            result
                .drawing_percentages
                .insert(hand_situation.situation, hand_situation.do_it);
        }

        for hand_situation in &self.data.double_down_percentages {
            result
                .double_down_percentages
                .insert(hand_situation.situation, hand_situation.do_it);
        }

        for split_situation in &self.data.split_percentages {
            result
                .split_percentages
                .insert(split_situation.situation, split_situation.do_it);
        }
        result
    }
}

#[derive(Default, Clone)]
pub struct BlackjackStrategyCombinedHashMap {
    data: HashMap<GameSituation, bool>,
}

impl BlackjackStrategyCombinedHashMap {
    pub fn new() -> BlackjackStrategyCombinedHashMap {
        BlackjackStrategyCombinedHashMap {
            data: HashMap::new(),
        }
    }
}

impl BlackjackStrategyTrait for BlackjackStrategyCombinedHashMap {
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