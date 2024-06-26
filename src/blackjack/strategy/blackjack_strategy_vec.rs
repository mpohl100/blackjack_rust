use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::deck::WrappedDeck;
use crate::blackjack::strategy::blackjack_strategy_map::BlackjackStrategy;
use crate::blackjack::strategy::blackjack_strategy_map::BlackjackStrategyData;
use crate::blackjack::traits::BlackjackGame;
use crate::blackjack::traits::BlackjackStrategyTrait;

use async_trait::async_trait;

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

#[async_trait]
impl BlackjackGame for BlackjackStrategyVec {
    async fn get_draw(&mut self, situation: HandSituation, _deck: &mut WrappedDeck) -> bool {
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

    async fn get_double_down(&mut self, situation: HandSituation, _deck: &mut WrappedDeck) -> bool {
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

    async fn get_split(&mut self, situation: SplitSituation, _deck: &mut WrappedDeck) -> bool {
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

#[async_trait]
impl BlackjackStrategyTrait for BlackjackStrategyVec {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn upcast_mut(&mut self) -> &mut dyn BlackjackGame {
        self
    }

    async fn add_draw(&mut self, situation: HandSituation, do_it: bool) {
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

    async fn add_double_down(&mut self, situation: HandSituation, do_it: bool) {
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

    async fn add_split(&mut self, situation: SplitSituation, do_it: bool) {
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

    async fn to_string_mat2(&self) -> String {
        let mut blackjack_strategy_sorted = BlackjackStrategy::new(false);
        for situation_strategy in &self.data.drawing_percentages {
            blackjack_strategy_sorted
                .add_draw(situation_strategy.situation, situation_strategy.do_it)
                .await;
        }
        for situation_strategy in &self.data.double_down_percentages {
            blackjack_strategy_sorted
                .add_double_down(situation_strategy.situation, situation_strategy.do_it)
                .await;
        }
        for situation_strategy in &self.data.split_percentages {
            blackjack_strategy_sorted
                .add_split(situation_strategy.situation, situation_strategy.do_it)
                .await;
        }
        blackjack_strategy_sorted.to_string_mat2().await
    }

    async fn combine(&mut self, blackjack_strategy: &BlackjackStrategyData) {
        for (sit, do_it) in blackjack_strategy.drawing_decisions.iter() {
            self.add_draw(*sit, *do_it).await;
        }

        for (sit, do_it) in blackjack_strategy.double_down_decisions.iter() {
            self.add_double_down(*sit, *do_it).await;
        }

        for (sit, do_it) in blackjack_strategy.split_decisions.iter() {
            self.add_split(*sit, *do_it).await;
        }
    }

    async fn dump(&self) -> BlackjackStrategyData {
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
