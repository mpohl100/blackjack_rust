use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::deck::Deck;
use crate::blackjack::strategy::blackjack_strategy_map::BlackjackStrategy;
use crate::blackjack::strategy::blackjack_strategy_map::BlackjackStrategyData;
use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::blackjack::traits::BlackjackGame;

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

impl BlackjackGame for BlackjackStrategyVec{
    fn get_draw(&self, situation: HandSituation, _deck: &dyn Deck) -> bool {
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

    fn get_double_down(&self, situation: HandSituation, _deck: &dyn Deck) -> bool {
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

    fn get_split(&self, situation: SplitSituation, _deck: &dyn Deck) -> bool {
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
}

impl BlackjackStrategyTrait for BlackjackStrategyVec {
    fn upcast(&self) -> &dyn BlackjackGame {
        self
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
        for (sit, do_it) in blackjack_strategy.drawing_decisions.iter() {
            self.add_draw(*sit, *do_it);
        }

        for (sit, do_it) in blackjack_strategy.double_down_decisions.iter() {
            self.add_double_down(*sit, *do_it);
        }

        for (sit, do_it) in blackjack_strategy.split_decisions.iter() {
            self.add_split(*sit, *do_it);
        }
    }

    fn dump(&self) -> BlackjackStrategyData {
        let mut result = BlackjackStrategyData::default();

        for hand_situation in &self.data.drawing_percentages {
            result
                .drawing_decisions
                .insert(hand_situation.situation, hand_situation.do_it);
        }

        for hand_situation in &self.data.double_down_percentages {
            result
                .double_down_decisions
                .insert(hand_situation.situation, hand_situation.do_it);
        }

        for split_situation in &self.data.split_percentages {
            result
                .split_decisions
                .insert(split_situation.situation, split_situation.do_it);
        }
        result
    }
}
