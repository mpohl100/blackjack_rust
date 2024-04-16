use crate::blackjack::blackjack_configuration::StrategyConfiguration;
use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::blackjack::analysis::blackjack_analysis::optimize_blackjack;
use crate::blackjack::strategy::counted_blackjack_strategy::CountedBlackjackStrategy;
use std::collections::BTreeMap;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

pub fn optimize_counted<BlackjackStrategyType>(
    blackjack_strategy: BlackjackStrategyType,
    strat_config: StrategyConfiguration,
    thread_pool: &ThreadPool,
) -> impl BlackjackStrategyTrait
where
    BlackjackStrategyType: BlackjackStrategyTrait + Clone + 'static + Send,
{
    let mut data = BTreeMap::<i32, Box<dyn BlackjackStrategyTrait>>::new();
    let (transaction, receiver) = channel();
    for i in -10..11 {
        let tr = transaction.clone();
        let strat_config_clone = strat_config.clone();
        let blackjack_strategy_clone = blackjack_strategy.clone();
        thread_pool.execute(move || {
            let pool = ThreadPool::new(strat_config_clone.nb_threads.try_into().unwrap());
            let strat = optimize_blackjack(blackjack_strategy_clone, &pool, i);
            tr.send((i, strat)).expect("Could not send strategy");
        });
    }
    for _ in -10..11 {
        let (i, strat) = receiver.recv().expect("Could not receive strategy");
        data.insert(i, Box::new(strat));
    }
    CountedBlackjackStrategy::new(data)
}
