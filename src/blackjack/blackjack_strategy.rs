use std::collections::BTreeMap;
use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::blackjack_points::Points;
use crate::blackjack::card::BlackjackRank;
use crate::blackjack::traits::Allable;
use crate::blackjack::traits::Stringable;
pub use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::blackjack::deck::Deck;



#[derive(Default, Clone)]
struct BlackjackStrategyData {
    pub drawing_percentages: BTreeMap<HandSituation, bool>,
    pub double_down_percentages: BTreeMap<HandSituation, bool>,
    pub split_percentages: BTreeMap<SplitSituation, bool>,
}

#[derive(Default, Clone)]
pub struct BlackjackStrategy{
    data: BlackjackStrategyData,
}

impl BlackjackStrategy{
    pub fn new() -> BlackjackStrategy{
        BlackjackStrategy{data: BlackjackStrategyData::default()}
    }
}

impl BlackjackStrategyTrait for BlackjackStrategy{
    fn to_string_mat2(&self) -> String
    {
        let mut hard_strat: BTreeMap<HandSituation, String> = BTreeMap::new();
        let mut soft_strat: BTreeMap<HandSituation, String> = BTreeMap::new();

        for (situation, do_it) in self.data.double_down_percentages.iter() {
            let points = situation.situation();
            if points.upper() == points.lower() { 
                if *do_it {
                    hard_strat.insert(situation.clone(), "D".to_string());
                } else if *self.data.drawing_percentages.get(situation).unwrap() {
                    hard_strat.insert(situation.clone(), "H".to_string());
                } else {
                    hard_strat.insert(situation.clone(), "S".to_string());
                }
            } 
            else { 
                if *do_it {
                    soft_strat.insert(situation.clone(), "D".to_string());
                } else if *self.data.drawing_percentages.get(situation).unwrap() {
                    soft_strat.insert(situation.clone(), "H".to_string());
                } else {
                    soft_strat.insert(situation.clone(), "S".to_string());
                }
            };

        }

        let mut ret = "Hard hands strategy:\n".to_string();
        let mut first_points = Points::default();
        ret.push_str(";");
        for rank in BlackjackRank::create_all() {
            ret.push_str(&(rank.to_string_internal() + &";".to_string()));
        }
        for (situation, action) in hard_strat {
            let points = situation.situation();
            if points != first_points {
                ret.push_str("\n");
                ret.push_str(&(points.to_string_internal() + &";".to_string()));
                first_points = points;
            }
            ret.push_str(&(action + &";".to_string()));
        }

        ret.push_str("\nSoft hands strategy:\n");
        first_points = Points::default();
        ret.push_str(";");
        for rank in BlackjackRank::create_all() {
            ret.push_str(&(rank.to_string_internal() + &";".to_string()));
        }
        for (situation, action) in soft_strat {
            let points = situation.situation();
            if points != first_points {
                ret.push_str("\n");
                ret.push_str(&(points.to_string_internal() + &";".to_string()));
                first_points = points;
            }
            ret.push_str(&(action + &";".to_string()));
        }
        
        ret.push_str("\nSplitting Strategy:\n");
        let mut first_rank = BlackjackRank::default();
        ret.push_str(";");
        for rank in BlackjackRank::create_all() {
            ret.push_str(&(rank.to_string_internal() + &";".to_string()));
        }
        for (situation, do_it) in self.data.split_percentages.iter() {
            let hand_rank = situation.situation();
            if hand_rank != first_rank {
                ret.push_str("\n");
                ret.push_str(&(hand_rank.to_string_internal() + &";".to_string()));
                first_rank = hand_rank;
            }
            if *do_it {
                ret.push_str("P;");
            } else {
                ret.push_str("-;");
            }
        }
        
        ret
    }

    fn get_draw(&self, situation: HandSituation, _deck: &Box<dyn Deck>) -> bool
    {
        let it = self.data.drawing_percentages.get(&situation);
        if it == None {
            panic!("Drawing strategy not found {} ; {}", situation.situation().to_string_internal(), situation.dealer_card().to_string_internal());
        }
        let draw = *it.unwrap();
        draw
    }

    fn get_double_down(&self, situation: HandSituation, _deck: &Box<dyn Deck>) -> bool
    {
        let it = self.data.double_down_percentages.get(&situation);
        if it == None {
            panic!("Double down strategy not found {} ; {}", situation.situation().to_string_internal(), situation.dealer_card().to_string_internal());
        }
        let only_draw_once = *it.unwrap(); 
        only_draw_once
    }

    fn get_split(&self, situation: SplitSituation, _deck: &Box<dyn Deck>) -> bool
    {
        let it = self.data.split_percentages.get(&situation);
        if it == None {
            panic!("Split strategy not found for rank {} ; {}", situation.situation().to_string_internal(), situation.dealer_card().to_string_internal())
        }
        let do_split = *it.unwrap();
        do_split
    }

    fn add_draw(&mut self, situation: HandSituation, do_it: bool)
    {
        self.data.drawing_percentages.insert(situation, do_it);
    }

    fn add_double_down(&mut self, situation: HandSituation, do_it: bool)
    {
        self.data.double_down_percentages.insert(situation, do_it);
    }

    fn add_split(&mut self, situation: SplitSituation, do_it: bool)
    {
        self.data.split_percentages.insert(situation, do_it);
    }
}