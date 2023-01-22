use std::collections::BTreeMap;
use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::blackjack_points::Points;
use crate::blackjack::card::BlackjackRank;
use crate::blackjack::traits::Allable;

#[derive(Default, Clone)]
pub struct BlackjackStrategy {
    pub drawing_percentages: BTreeMap<HandSituation, bool>,
    pub double_down_percentages: BTreeMap<HandSituation, bool>,
    pub split_percentages: BTreeMap<SplitSituation, bool>,
}

impl BlackjackStrategy{
    pub fn to_string_mat2(&self) -> String
    {
        let mut hard_strat: BTreeMap<HandSituation, String> = BTreeMap::new();
        let mut soft_strat: BTreeMap<HandSituation, String> = BTreeMap::new();

        for (situation, do_it) in self.double_down_percentages.iter() {
            let points = situation.situation();
            let target = if points.upper() == points.lower() { &mut hard_strat } else { &mut soft_strat };
            if *do_it {
                target.insert(situation.clone(), "D".to_string());
            } else if *self.drawing_percentages.get(situation).unwrap() {
                target.insert(situation.clone(), "H".to_string());
            } else {
                target.insert(situation.clone(), "S".to_string());
            }
        }

        let mut ret = "Hard hands strategy:\n".to_string();
        let mut first_points = Points::default();
        ret.push_str(";");
        for rank in BlackjackRank::create_all() {
            ret.push_str(&(rank.to_string() + &";".to_string()));
        }
        for (situation, action) in hard_strat {
            let points = situation.situation();
            if points != first_points {
                ret.push_str("\n");
                ret.push_str(&(points.to_string() + &";".to_string()));
                first_points = points;
            }
            ret.push_str(&(action + &";".to_string()));
        }

        ret.push_str("\nSoft hands strategy:\n");
        first_points = Points::default();
        ret.push_str(";");
        for rank in BlackjackRank::create_all() {
            ret.push_str(&(rank.to_string() + &";".to_string()));
        }
        for (situation, action) in soft_strat {
            let points = situation.situation();
            if points != first_points {
                ret.push_str("\n");
                ret.push_str(&(points.to_string() + &";".to_string()));
                first_points = points;
            }
            ret.push_str(&(action + &";".to_string()));
        }
        
        ret.push_str("\nSplitting Strategy:\n");
        let mut first_rank = BlackjackRank::default();
        ret.push_str(";");
        for rank in BlackjackRank::create_all() {
            ret.push_str(&(rank.to_string() + &";".to_string()));
        }
        for (situation, do_it) in self.split_percentages.iter() {
            let hand_rank = situation.situation();
            if hand_rank != first_rank {
                ret.push_str("\n");
                ret.push_str(&(hand_rank.to_string() + &";".to_string()));
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
}