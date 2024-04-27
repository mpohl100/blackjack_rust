use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::deck::WrappedDeck;
use crate::blackjack::strategy::blackjack_strategy_map::BlackjackStrategyData;

use async_trait::async_trait;
use std::sync::{Arc, Mutex};
pub trait Allable {
    fn create_all() -> Vec<Self>
    where
        Self: Sized;
}

pub trait Stringable {
    fn to_string_internal(&self) -> String;
}

#[async_trait]
pub trait BlackjackGame {
    async fn get_draw(&mut self, situation: HandSituation, deck: &mut WrappedDeck) -> bool;
    async fn get_double_down(&mut self, situation: HandSituation, deck: &mut WrappedDeck) -> bool;
    async fn get_split(&mut self, situation: SplitSituation, deck: &mut WrappedDeck) -> bool;
}

pub trait BlackjackStrategyTrait: BlackjackGame {
    fn upcast_mut(&mut self) -> &mut dyn BlackjackGame;
    fn add_draw(&mut self, situation: HandSituation, do_it: bool);
    fn add_double_down(&mut self, situation: HandSituation, do_it: bool);
    fn add_split(&mut self, situation: SplitSituation, do_it: bool);

    fn to_string_mat2(&self) -> String;

    fn combine(&mut self, blackjack_strategy: &BlackjackStrategyData);
    fn dump(&self) -> BlackjackStrategyData;
}

pub struct WrappedStrategy<BlackjackStrategyType> where BlackjackStrategyType: BlackjackStrategyTrait + Send{
    strat: Arc<Mutex<BlackjackStrategyType>>,
}

impl<BlackjackStrategyType> WrappedStrategy<BlackjackStrategyType> where BlackjackStrategyType: BlackjackStrategyTrait + Send{
    pub fn new(strat: BlackjackStrategyType) -> WrappedStrategy<BlackjackStrategyType> {
        WrappedStrategy {
            strat: Arc::new(Mutex::new(strat)),
        }
    }

    pub async fn get_draw(&self, situation: HandSituation, deck: &mut WrappedDeck) -> bool {
        let mut strat = self.strat.lock().unwrap();
        strat.get_draw(situation, deck).await
    }

    pub async fn get_double_down(&self, situation: HandSituation, deck: &mut WrappedDeck) -> bool {
        let mut strat = self.strat.lock().unwrap();
        strat.get_double_down(situation, deck).await
    }

    pub async fn get_split(&self, situation: SplitSituation, deck: &mut WrappedDeck) -> bool {
        let mut strat = self.strat.lock().unwrap();
        strat.get_split(situation, deck).await
    }

    pub fn add_draw(&self, situation: HandSituation, do_it: bool) {
        let mut strat = self.strat.lock().unwrap();
        strat.add_draw(situation, do_it);
    }

    pub fn add_double_down(&self, situation: HandSituation, do_it: bool) {
        let mut strat = self.strat.lock().unwrap();
        strat.add_double_down(situation, do_it);
    }

    pub fn add_split(&self, situation: SplitSituation, do_it: bool) {
        let mut strat = self.strat.lock().unwrap();
        strat.add_split(situation, do_it);
    }

    pub fn to_string_mat2(&self) -> String {
        let strat = self.strat.lock().unwrap();
        strat.to_string_mat2()
    }

    pub fn combine(&self, blackjack_strategy: &BlackjackStrategyData) {
        let mut strat = self.strat.lock().unwrap();
        strat.combine(blackjack_strategy);
    }

    pub fn dump(&self) -> BlackjackStrategyData {
        let strat = self.strat.lock().unwrap();
        strat.dump()
    }
}
