use crate::blackjack::blackjack_situation::GameSituation;
use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::deck::WrappedDeck;
use crate::blackjack::strategy::blackjack_strategy_map::BlackjackStrategy;
use crate::blackjack::strategy::blackjack_strategy_map::BlackjackStrategyData;
use crate::blackjack::traits::BlackjackGame;
use crate::blackjack::traits::BlackjackStrategyTrait;
use std::collections::HashMap;

use async_trait::async_trait;

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

#[async_trait]
impl BlackjackGame for BlackjackStrategyCombinedHashMap {
    async fn get_draw(&mut self, situation: HandSituation, _deck: &mut WrappedDeck) -> bool {
        match self.data.get(&GameSituation::Draw(situation)) {
            Some(value) => *value,
            _ => panic!("Couldn't find draw hand situation in drawing percentages"),
        }
    }

    async fn get_double_down(&mut self, situation: HandSituation, _deck: &mut WrappedDeck) -> bool {
        match self.data.get(&GameSituation::DoubleDown(situation)) {
            Some(value) => *value,
            _ => panic!("Couldn't find double down situation in double down percentages"),
        }
    }

    async fn get_split(&mut self, situation: SplitSituation, _deck: &mut WrappedDeck) -> bool {
        match self.data.get(&GameSituation::Split(situation)) {
            Some(value) => *value,
            _ => panic!("Couldn't find split situation in split percentages"),
        }
    }
}

impl BlackjackStrategyTrait for BlackjackStrategyCombinedHashMap {
    fn upcast_mut(&mut self) -> &mut dyn BlackjackGame {
        self
    }

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

        for (situation, do_it) in &self.data {
            match situation {
                GameSituation::Draw(hand_situation) => {
                    result.drawing_decisions.insert(*hand_situation, *do_it);
                }
                GameSituation::DoubleDown(hand_situation) => {
                    result.double_down_decisions.insert(*hand_situation, *do_it);
                }
                GameSituation::Split(split_situation) => {
                    result.split_decisions.insert(*split_situation, *do_it);
                }
            }
        }
        result
    }
}
