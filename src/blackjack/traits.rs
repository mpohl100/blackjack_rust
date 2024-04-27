use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::deck::WrappedDeck;
use crate::blackjack::strategy::blackjack_strategy_map::BlackjackStrategyData;

use async_trait::async_trait;

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
