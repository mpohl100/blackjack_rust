
use crate::blackjack::analysis::blackjack_analysis::optimize_blackjack;
use crate::blackjack::strategy::counted_blackjack_strategy::CountedBlackjackStrategy;
use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::blackjack::traits::WrappedStrategy;
use std::collections::BTreeMap;
use tokio::sync::mpsc::channel;

pub async fn optimize_counted<BlackjackStrategyType>(
    blackjack_strategy: BlackjackStrategyType,
) -> CountedBlackjackStrategy
where
    BlackjackStrategyType: BlackjackStrategyTrait + Clone + 'static + Send,
{
    let mut data = BTreeMap::<i32, WrappedStrategy>::new();
    let (transaction, mut receiver) = channel::<(i32, WrappedStrategy)>(32);
    let blackjack_strategy_clone = blackjack_strategy;
    for i in -10..11 {
        let tr = transaction.clone();
        let wrapped_blackjack_strategy = WrappedStrategy::new(blackjack_strategy_clone.clone());
        tokio::spawn(async move {
            let strat = optimize_blackjack(wrapped_blackjack_strategy, i).await;
            let _ = tr.send((i, strat)).await;
        });
    }
    for _ in -10..11 {
        let (i, strat) = match receiver.recv().await {
            Some(result) => result,
            None => panic!("Did not receive blackjack"),
        };
        data.insert(i, strat);
    }
    CountedBlackjackStrategy::new(data)
}
