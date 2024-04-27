use crate::blackjack::analysis::blackjack_analysis::optimize_blackjack;
use crate::blackjack::strategy::counted_blackjack_strategy::CountedBlackjackStrategy;
use crate::blackjack::traits::BlackjackStrategyTrait;
use std::collections::BTreeMap;
use tokio::sync::mpsc::channel;

pub async fn optimize_counted<BlackjackStrategyType>(
    blackjack_strategy: BlackjackStrategyType,
) -> CountedBlackjackStrategy
where
    BlackjackStrategyType: BlackjackStrategyTrait + Clone + 'static + Send,
{
    let mut data = BTreeMap::<i32, Box<dyn BlackjackStrategyTrait + Send>>::new();
    let (transaction, mut receiver) = channel(32);
    for i in -10..11 {
        let tr = transaction.clone();
        let blackjack_strategy_clone = blackjack_strategy.clone();
        tokio::spawn(async move {
            let strat = optimize_blackjack(blackjack_strategy_clone, i).await;
            tr.send((i, strat));
        });
    }
    for _ in -10..11 {
        let (i, strat) = match receiver.recv().await {
            Some(result) => result,
            None => panic!("Did not receive blackjack"),
        };
        data.insert(i, Box::new(strat));
    }
    CountedBlackjackStrategy::new(data)
}
