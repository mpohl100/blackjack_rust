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

pub struct CountedBlackjackStrategy{
    counted_strategies: BTreeMap<i32, Box<dyn BlackjackStrategyTrait>>,
    max_count: i32,
    min_count: i32,
}

impl CountedBlackjackStrategy{
    pub fn new(data: BTreeMap<i32, Box<dyn BlackjackStrategyTrait>>) -> CountedBlackjackStrategy{
        let max_count;
        match data.keys().next_back(){
            Some(value) => {max_count = *value},
            _ => panic!("max_count not found in data"),
        };
        let min_count;
        match data.keys().next(){
            Some(value) => {min_count = *value},
            _ => panic!("min_count not found in data"),
        };
        CountedBlackjackStrategy{counted_strategies: data, max_count: max_count, min_count: min_count}
    }
}

fn get_clamped_count(deck: &Box<dyn Deck>, min_count: i32, max_count: i32) -> i32{
    let nb_cards = deck.get_nb_cards();
    let count = deck.get_count();
    let ratio = (count as f64) / (nb_cards as f64);
    let count = (ratio * 52.0) as i32;
    if count > max_count {
        max_count
    }
    else if count < min_count {
        min_count
    } else {
        count
    }
}

impl BlackjackStrategyTrait for CountedBlackjackStrategy{
    fn get_draw(&self, situation: HandSituation, deck: &Box<dyn Deck>) -> bool
    {
        match self.counted_strategies.get(&get_clamped_count(&deck, self.min_count, self.max_count)){
            Some(strat) => {strat.get_draw(situation, deck)},
            _ => panic!("Count {} not found in counted_strategies", deck.get_count()),
        }
    }

    fn get_double_down(&self, situation: HandSituation, deck: &Box<dyn Deck>) -> bool
    {
        match self.counted_strategies.get(&get_clamped_count(&deck, self.min_count, self.max_count)){
            Some(strat) => {strat.get_double_down(situation, deck)},
            _ => panic!("Count {} not found in counted_strategies", deck.get_count()),
        }
    }
    fn get_split(&self, situation: SplitSituation, deck: &Box<dyn Deck>) -> bool
    {
        match self.counted_strategies.get(&get_clamped_count(&deck, self.min_count, self.max_count)){
            Some(strat) => {strat.get_split(situation, deck)},
            _ => panic!("Count {} not found in counted_strategies", deck.get_count()),
        }
    }

    fn add_draw(&mut self, _situation: HandSituation, _do_it: bool)
    {
        unimplemented!()
    }

    fn add_double_down(&mut self, _situation: HandSituation, _do_it: bool)
    {
        unimplemented!()
    }

    fn add_split(&mut self, _situation: SplitSituation, _do_it: bool)
    {
        unimplemented!()
    }

    fn to_string_mat2(&self) -> String
    {
        let mut ret = String::new();
        for (count, strat) in &self.counted_strategies{
            ret.push_str(&("Count ".to_owned() + &count.to_string()));
            ret.push_str(&("Strategy: ".to_owned() + &strat.to_string_mat2() + &"\n\n".to_string()));
        }
        ret
    }
}