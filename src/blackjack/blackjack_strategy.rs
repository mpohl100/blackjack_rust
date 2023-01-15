use std::collections::BTreeMap;
use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;

#[derive(Default, Clone)]
pub struct BlackjackStrategy {
    pub drawing_percentages: BTreeMap<HandSituation, bool>,
    pub double_down_percentages: BTreeMap<HandSituation, bool>,
    pub split_percentages: BTreeMap<SplitSituation, bool>,
}

impl BlackjackStrategy{
    fn to_string(&self) -> String
    {
        return String::default();
    }

    fn to_string_mat(&self) -> String
    {
        return String::default();
    }

    pub fn to_string_mat2(&self) -> String
    {
        return String::default();
    }

    //fn create_test(draw: &Percentage, double_down: &Percentage, split: &Percentage) -> BlackjackStrategy
    //{
    //    return {};
    //}
}