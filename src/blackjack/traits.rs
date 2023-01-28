use crate::blackjack::blackjack_analysis::HandSituation;
use crate::blackjack::blackjack_analysis::SplitSituation;
use crate::blackjack::deck::Deck;
pub trait Allable{
    fn create_all() -> Vec<Self> where Self: Sized;
}

pub trait Stringable{
    fn to_string_internal(&self) -> String;
}

pub trait BlackjackStrategyTrait{
    fn get_draw(&self, situation: HandSituation, deck: &Box<dyn Deck>) -> bool;
    fn get_double_down(&self, situation: HandSituation, deck: &Box<dyn Deck>) -> bool;
    fn get_split(&self, situation: SplitSituation, deck: &Box<dyn Deck>) -> bool;

    fn add_draw(&mut self, situation: HandSituation, do_it: bool);
    fn add_double_down(&mut self, situation: HandSituation, do_it: bool);
    fn add_split(&mut self, situation: SplitSituation, do_it: bool);

    fn to_string_mat2(&self) -> String;
}