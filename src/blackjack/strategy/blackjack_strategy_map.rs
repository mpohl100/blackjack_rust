use crate::blackjack::blackjack_points::Points;
use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::card::BlackjackRank;
use crate::blackjack::deck::WrappedDeck;
use crate::blackjack::traits::Allable;
use crate::blackjack::traits::BlackjackGame;
use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::blackjack::traits::Stringable;
use async_trait::async_trait;
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct BlackjackStrategyData {
    pub drawing_decisions: BTreeMap<HandSituation, bool>,
    pub double_down_decisions: BTreeMap<HandSituation, bool>,
    pub split_decisions: BTreeMap<SplitSituation, bool>,
}

#[derive(Default, Clone)]
struct BlackjackStrategyDataHash {
    pub drawing_decisions: HashMap<HandSituation, bool>,
    pub double_down_decisions: HashMap<HandSituation, bool>,
    pub split_decisions: HashMap<SplitSituation, bool>,
}

#[derive(Default, Clone)]
pub struct BlackjackStrategy {
    data: BlackjackStrategyData,
    data_hash: BlackjackStrategyDataHash,
    use_hash: bool,
}

impl BlackjackStrategy {
    pub fn new(use_hash: bool) -> BlackjackStrategy {
        BlackjackStrategy {
            data: BlackjackStrategyData::default(),
            data_hash: BlackjackStrategyDataHash::default(),
            use_hash,
        }
    }
}

#[async_trait]
impl BlackjackGame for BlackjackStrategy {
    async fn get_draw(&mut self, situation: HandSituation, _deck: &mut WrappedDeck) -> bool {
        let it = if !self.use_hash {
            self.data.drawing_decisions.get(&situation)
        } else {
            self.data_hash.drawing_decisions.get(&situation)
        };
        if it.is_none() {
            panic!(
                "Drawing strategy not found {} ; {}",
                situation.situation().to_string_internal(),
                situation.dealer_card().to_string_internal()
            );
        }

        *it.unwrap()
    }

    async fn get_double_down(&mut self, situation: HandSituation, _deck: &mut WrappedDeck) -> bool {
        let it = if !self.use_hash {
            self.data.double_down_decisions.get(&situation)
        } else {
            self.data_hash.double_down_decisions.get(&situation)
        };
        if it.is_none() {
            panic!(
                "Double down strategy not found {} ; {}",
                situation.situation().to_string_internal(),
                situation.dealer_card().to_string_internal()
            );
        }

        *it.unwrap()
    }

    async fn get_split(&mut self, situation: SplitSituation, _deck: &mut WrappedDeck) -> bool {
        let it = if !self.use_hash {
            self.data.split_decisions.get(&situation)
        } else {
            self.data_hash.split_decisions.get(&situation)
        };
        if it.is_none() {
            panic!(
                "Split strategy not found for rank {} ; {}",
                situation.situation().to_string_internal(),
                situation.dealer_card().to_string_internal()
            )
        }

        *it.unwrap()
    }
}

#[async_trait]
impl BlackjackStrategyTrait for BlackjackStrategy {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn upcast_mut(&mut self) -> &mut dyn BlackjackGame {
        self
    }

    async fn to_string_mat2(&self) -> String {
        let mut hard_strat: BTreeMap<HandSituation, String> = BTreeMap::new();
        let mut soft_strat: BTreeMap<HandSituation, String> = BTreeMap::new();

        for (situation, do_it) in self.data.double_down_decisions.iter() {
            let points = situation.situation();
            if points.upper() == points.lower() {
                if *do_it {
                    hard_strat.insert(*situation, "D".to_string());
                } else if *self.data.drawing_decisions.get(situation).unwrap() {
                    hard_strat.insert(*situation, "H".to_string());
                } else {
                    hard_strat.insert(*situation, "S".to_string());
                }
            } else if *do_it {
                soft_strat.insert(*situation, "D".to_string());
            } else if *self.data.drawing_decisions.get(situation).unwrap() {
                soft_strat.insert(*situation, "H".to_string());
            } else {
                soft_strat.insert(*situation, "S".to_string());
            };
        }

        let mut ret = "Hard hands strategy:\n".to_string();
        let mut first_points = Points::default();
        ret.push(';');
        for rank in BlackjackRank::create_all() {
            ret.push_str(&(rank.to_string_internal() + ";"));
        }
        for (situation, action) in hard_strat {
            let points = situation.situation();
            if points != first_points {
                ret.push('\n');
                ret.push_str(&(points.to_string_internal() + ";"));
                first_points = points;
            }
            ret.push_str(&(action + ";"));
        }

        ret.push_str("\nSoft hands strategy:\n");
        first_points = Points::default();
        ret.push(';');
        for rank in BlackjackRank::create_all() {
            ret.push_str(&(rank.to_string_internal() + ";"));
        }
        for (situation, action) in soft_strat {
            let points = situation.situation();
            if points != first_points {
                ret.push('\n');
                ret.push_str(&(points.to_string_internal() + ";"));
                first_points = points;
            }
            ret.push_str(&(action + ";"));
        }

        ret.push_str("\nSplitting Strategy:\n");
        let mut first_rank = BlackjackRank::default();
        ret.push(';');
        for rank in BlackjackRank::create_all() {
            ret.push_str(&(rank.to_string_internal() + ";"));
        }
        for (situation, do_it) in self.data.split_decisions.iter() {
            let hand_rank = situation.situation();
            if hand_rank != first_rank {
                ret.push('\n');
                ret.push_str(&(hand_rank.to_string_internal() + ";"));
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

    async fn add_draw(&mut self, situation: HandSituation, do_it: bool) {
        self.data.drawing_decisions.insert(situation, do_it);
        self.data_hash.drawing_decisions.insert(situation, do_it);
    }

    async fn add_double_down(&mut self, situation: HandSituation, do_it: bool) {
        self.data.double_down_decisions.insert(situation, do_it);
        self.data_hash
            .double_down_decisions
            .insert(situation, do_it);
    }

    async fn add_split(&mut self, situation: SplitSituation, do_it: bool) {
        self.data.split_decisions.insert(situation, do_it);
        self.data_hash.split_decisions.insert(situation, do_it);
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
        self.data.clone()
    }
}
