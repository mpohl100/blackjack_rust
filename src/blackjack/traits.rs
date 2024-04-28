use crate::blackjack::blackjack_situation::HandSituation;
use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::deck::WrappedDeck;
use crate::blackjack::strategy::blackjack_strategy_combined_hash_map::BlackjackStrategyCombinedHashMap;
use crate::blackjack::strategy::blackjack_strategy_combined_ordered_hash_map::BlackjackStrategyCombinedOrderedHashMap;
use crate::blackjack::strategy::blackjack_strategy_combined_vec::BlackjackStrategyCombinedVec;
use crate::blackjack::strategy::blackjack_strategy_map::BlackjackStrategy;
use crate::blackjack::strategy::blackjack_strategy_map::BlackjackStrategyData;
use crate::blackjack::strategy::blackjack_strategy_vec::BlackjackStrategyVec;
use crate::blackjack::strategy::counted_blackjack_strategy::CountedBlackjackStrategy;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
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

#[async_trait]
pub trait BlackjackStrategyTrait: BlackjackGame {
    fn as_any(&self) -> &dyn std::any::Any;
    fn upcast_mut(&mut self) -> &mut dyn BlackjackGame;
    async fn add_draw(&mut self, situation: HandSituation, do_it: bool);
    async fn add_double_down(&mut self, situation: HandSituation, do_it: bool);
    async fn add_split(&mut self, situation: SplitSituation, do_it: bool);

    async fn to_string_mat2(&self) -> String;

    async fn combine(&mut self, blackjack_strategy: &BlackjackStrategyData);
    async fn dump(&self) -> BlackjackStrategyData;
}

#[derive(Clone)]
pub struct WrappedStrategy {
    strat: Arc<Mutex<Box<dyn BlackjackStrategyTrait + Send>>>,
}

impl WrappedStrategy {
    pub fn new<BlackjackStrategyType>(strat: BlackjackStrategyType) -> WrappedStrategy
    where
        BlackjackStrategyType: BlackjackStrategyTrait + Send + 'static,
    {
        let b = Box::new(strat) as Box<dyn BlackjackStrategyTrait + Send>;
        WrappedStrategy {
            strat: Arc::new(Mutex::new(b)),
        }
    }

    pub async fn get_draw(&self, situation: HandSituation, deck: &mut WrappedDeck) -> bool {
        let mut strat = self.strat.lock().await;
        strat.get_draw(situation, deck).await
    }

    pub async fn get_double_down(&self, situation: HandSituation, deck: &mut WrappedDeck) -> bool {
        let mut strat = self.strat.lock().await;
        strat.get_double_down(situation, deck).await
    }

    pub async fn get_split(&self, situation: SplitSituation, deck: &mut WrappedDeck) -> bool {
        let mut strat = self.strat.lock().await;
        strat.get_split(situation, deck).await
    }

    pub async fn add_draw(&self, situation: HandSituation, do_it: bool) {
        let mut strat = self.strat.lock().await;
        strat.add_draw(situation, do_it).await;
    }

    pub async fn add_double_down(&self, situation: HandSituation, do_it: bool) {
        let mut strat = self.strat.lock().await;
        strat.add_double_down(situation, do_it).await;
    }

    pub async fn add_split(&self, situation: SplitSituation, do_it: bool) {
        let mut strat = self.strat.lock().await;
        strat.add_split(situation, do_it).await;
    }

    pub async fn to_string_mat2(&self) -> String {
        let strat = self.strat.lock().await;
        strat.to_string_mat2().await
    }

    pub async fn combine(&self, blackjack_strategy: &BlackjackStrategyData) {
        let mut strat = self.strat.lock().await;
        strat.combine(blackjack_strategy).await;
    }

    pub async fn dump(&self) -> BlackjackStrategyData {
        let strat = self.strat.lock().await;
        strat.dump().await
    }

    pub fn get(&mut self) -> Arc<Mutex<Box<dyn BlackjackStrategyTrait + Send>>> {
        self.strat.clone()
    }
}

#[derive(Clone)]
pub struct WrappedGame {
    game: Arc<Mutex<Box<dyn BlackjackGame + Send>>>,
}

impl WrappedGame {
    pub fn new<BlackjackGameType>(game: BlackjackGameType) -> WrappedGame
    where
        BlackjackGameType: BlackjackGame + Send + 'static,
    {
        let b = Box::new(game) as Box<dyn BlackjackGame + Send>;
        WrappedGame {
            game: Arc::new(Mutex::new(b)),
        }
    }

    pub async fn new_from_strat(arc_strat: &mut WrappedStrategy) -> WrappedGame {
        let arc_strategy = arc_strat.get();
        let strat = arc_strategy.lock().await;
        let game_map = match strat.as_any().downcast_ref::<BlackjackStrategy>() {
            Some(strategy) => Some(strategy),
            None => None,
        };
        let game_vec = match strat.as_any().downcast_ref::<BlackjackStrategyVec>() {
            Some(strategy) => Some(strategy),
            None => None,
        };
        let game_combined_hash_map = match strat
            .as_any()
            .downcast_ref::<BlackjackStrategyCombinedHashMap>()
        {
            Some(strategy) => Some(strategy),
            None => None,
        };
        let game_combined_ordered_hash_map = match strat
            .as_any()
            .downcast_ref::<BlackjackStrategyCombinedOrderedHashMap>(
        ) {
            Some(strategy) => Some(strategy),
            None => None,
        };
        let game_combined_vec = match strat
            .as_any()
            .downcast_ref::<BlackjackStrategyCombinedVec>()
        {
            Some(strategy) => Some(strategy),
            None => None,
        };
        let game_counted = match strat.as_any().downcast_ref::<CountedBlackjackStrategy>() {
            Some(strategy) => Some(strategy),
            None => None,
        };
        match game_map {
            Some(game_map_inner) => WrappedGame::new(game_map_inner.clone()),
            None => match game_vec {
                Some(game_vec_inner) => WrappedGame::new(game_vec_inner.clone()),
                None => match game_combined_hash_map {
                    Some(game_combined_hash_map_inner) => {
                        WrappedGame::new(game_combined_hash_map_inner.clone())
                    }
                    None => match game_combined_ordered_hash_map {
                        Some(game_combined_ordered_hash_map_inner) => {
                            WrappedGame::new(game_combined_ordered_hash_map_inner.clone())
                        }
                        None => match game_combined_vec {
                            Some(game_combined_vec_inner) => {
                                WrappedGame::new(game_combined_vec_inner.clone())
                            }
                            None => match game_counted {
                                Some(game_counted_inner) => {
                                    WrappedGame::new(game_counted_inner.clone())
                                }
                                None => panic!("Unknown strategy type"),
                            },
                        },
                    },
                },
            },
        }
    }

    pub async fn get_draw(&self, situation: HandSituation, deck: &mut WrappedDeck) -> bool {
        let mut game = self.game.lock().await;
        game.get_draw(situation, deck).await
    }

    pub async fn get_double_down(&self, situation: HandSituation, deck: &mut WrappedDeck) -> bool {
        let mut game = self.game.lock().await;
        game.get_double_down(situation, deck).await
    }

    pub async fn get_split(&self, situation: SplitSituation, deck: &mut WrappedDeck) -> bool {
        let mut game = self.game.lock().await;
        game.get_split(situation, deck).await
    }
}
