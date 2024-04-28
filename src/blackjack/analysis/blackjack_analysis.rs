use std::collections::BTreeMap;

use crate::blackjack::blackjack_challenge::BlackjackChallenge;
use crate::blackjack::blackjack_situation::GameSituation;
pub use crate::blackjack::blackjack_situation::HandSituation;
pub use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::card::BlackjackRank;
use crate::blackjack::card::Card;
use crate::blackjack::card::Rank;
use crate::blackjack::card::Suit;
use crate::blackjack::deck::CountedDeck;
use crate::blackjack::deck::WrappedDeck;
use crate::blackjack::evaluate_blackjack_hand::evaluate_blackjack_hand;
use crate::blackjack::hand::PlayerHand;
pub use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::blackjack::traits::WrappedStrategy;

use tokio::sync::mpsc::channel;

struct BlackjackGameSituation {
    pub game_situation: GameSituation,
    pub strat: WrappedStrategy,
}

fn get_dealer_rank(game_situation: GameSituation) -> BlackjackRank {
    match game_situation {
        GameSituation::Draw(hand_situation) => hand_situation.dealer_card(),
        GameSituation::DoubleDown(hand_situation) => hand_situation.dealer_card(),
        GameSituation::Split(split_situation) => split_situation.dealer_card(),
    }
}

fn get_player_hand(game_situation: GameSituation) -> PlayerHand {
    let mut ret = PlayerHand::default();
    match game_situation {
        GameSituation::Split(value) => {
            let representative_card = value.situation().get_representative_card();
            ret.add_card(&representative_card.clone());
            ret.add_card(&representative_card.clone());
        }
        GameSituation::Draw(value) | GameSituation::DoubleDown(value) => {
            let mut goal_points = value.situation().lower();
            let upper_points = value.situation().upper();
            if goal_points != upper_points {
                ret.add_card(&Card::new(Rank::Ace, Suit::Hearts));
                goal_points -= 1;
            }
            while goal_points > 0 {
                if goal_points >= 10 {
                    ret.add_card(&Card::new(Rank::Eight, Suit::Hearts));
                    goal_points -= 8;
                } else {
                    match goal_points {
                        1 => ret.add_card(&Card::new(Rank::Ace, Suit::Spades)),
                        2 => ret.add_card(&Card::new(Rank::Deuce, Suit::Spades)),
                        3 => ret.add_card(&Card::new(Rank::Three, Suit::Spades)),
                        4 => ret.add_card(&Card::new(Rank::Four, Suit::Spades)),
                        5 => ret.add_card(&Card::new(Rank::Five, Suit::Spades)),
                        6 => ret.add_card(&Card::new(Rank::Six, Suit::Spades)),
                        7 => ret.add_card(&Card::new(Rank::Seven, Suit::Spades)),
                        8 => ret.add_card(&Card::new(Rank::Eight, Suit::Spades)),
                        9 => ret.add_card(&Card::new(Rank::Nine, Suit::Spades)),
                        _ => panic!(
                            "Unexpected goal points in generation of player hand: {}",
                            goal_points
                        ),
                    }
                    break;
                }
            }
            let to_check = evaluate_blackjack_hand(&ret.get_blackjack_hand());
            if to_check != value.situation() {
                panic!("incorrect player hand formed.");
            }
        }
    }
    ret
}

async fn optimize_situation(situation: &mut BlackjackGameSituation, deck: &CountedDeck) -> bool {
    let boxed_deck = WrappedDeck::new(Box::new(deck.clone()));
    let mut challenge = BlackjackChallenge::new(
        situation.game_situation,
        get_dealer_rank(situation.game_situation),
        get_player_hand(situation.game_situation),
        situation.strat.clone(),
        boxed_deck,
    );
    let dont = false;
    let do_it = true;
    let score_dont = challenge.score(dont).await;
    let score_do_it = challenge.score(do_it).await;

    if score_do_it > score_dont {
        do_it
    } else {
        dont
    }
}

async fn calculate_draw(
    hand_situations: Vec<HandSituation>,
    deck: CountedDeck,
    blackjack_strategy: WrappedStrategy,
) -> WrappedStrategy {
    let result = blackjack_strategy;
    for hand_situation in hand_situations.iter().rev() {
        let mut situation = BlackjackGameSituation {
            game_situation: GameSituation::Draw(*hand_situation),
            strat: result.clone(),
        };
        result
            .add_draw(
                *hand_situation,
                optimize_situation(&mut situation, &deck).await,
            )
            .await;
    }
    result
}

async fn optimize_draw(blackjack_strategy: WrappedStrategy, card_count: i32) -> WrappedStrategy {
    let result = blackjack_strategy;
    let deck = CountedDeck::new(card_count);
    // first optimize drawing
    let all_situations = HandSituation::create_all();
    // bucketize
    let mut buckets = BTreeMap::<BlackjackRank, Vec<HandSituation>>::new();
    for sit in all_situations {
        let val = buckets.get_mut(&sit.dealer_card());
        if let Some(b) = val {
            b.push(sit);
        } else {
            buckets.insert(sit.dealer_card(), vec![sit]);
        }
    }
    // schedule work
    let (transaction, mut receiver) = channel(32);
    for (_, bucket) in buckets.iter() {
        let tr_clone = transaction.clone();
        let bucket_clone = bucket.clone();
        let deck_clone = deck.clone();
        let result_clone = result.clone();
        tokio::spawn(async move {
            let bucket_result = calculate_draw(bucket_clone, deck_clone, result_clone).await;
            let _ = tr_clone.send(bucket_result).await;
        });
    }
    // receive results
    for (_, _) in buckets.iter() {
        let bucket_result = receiver.recv().await;
        match bucket_result {
            Some(bucket_result) => result.combine(&bucket_result.dump().await).await,
            None => panic!("Error receiving draw result"),
        }
    }
    result
}

async fn optimize_double_down(
    blackjack_strategy: WrappedStrategy,
    card_count: i32,
) -> WrappedStrategy {
    let result = blackjack_strategy.clone();
    let deck = CountedDeck::new(card_count);
    let (transaction, mut receiver) = channel(32);
    for hand_situation in HandSituation::create_all() {
        let tr_clone = transaction.clone();
        let deck_clone = deck.clone();
        let result_clone = result.clone();
        let hand_situation_clone = hand_situation;
        tokio::spawn(async move {
            let mut situation = BlackjackGameSituation {
                game_situation: GameSituation::DoubleDown(hand_situation_clone),
                strat: result_clone.clone(),
            };
            let do_it = optimize_situation(&mut situation, &deck_clone).await;
            let _ = tr_clone.send((hand_situation_clone, do_it)).await;
        });
    }
    for _ in HandSituation::create_all() {
        let (hand_situation, do_it) = match receiver.recv().await {
            Some(result) => result,
            None => panic!("Did not receive blackjack strategy double down calculation"),
        };
        result.add_double_down(hand_situation, do_it).await;
    }
    result
}

async fn optimize_split(blackjack_strategy: WrappedStrategy, card_count: i32) -> WrappedStrategy {
    let result = blackjack_strategy.clone();
    let deck = CountedDeck::new(card_count);
    let (transaction, mut receiver) = channel(32);
    for split_situation in SplitSituation::create_all() {
        let tr_clone = transaction.clone();
        let deck_clone = deck.clone();
        let result_clone = result.clone();
        let split_situation_clone = split_situation;
        tokio::spawn(async move {
            let mut situation = BlackjackGameSituation {
                game_situation: GameSituation::Split(split_situation_clone),
                strat: result_clone,
            };
            let do_it = optimize_situation(&mut situation, &deck_clone).await;
            let _ = tr_clone.send((split_situation_clone, do_it)).await;
        });
    }
    for _ in SplitSituation::create_all() {
        let (split_situation, do_it) = match receiver.recv().await {
            Some(result) => result,
            None => panic!("Did not receive blackjack strategy split calculation"),
        };
        result.add_split(split_situation, do_it).await;
    }
    result
}

pub async fn optimize_blackjack(
    blackjack_strategy: WrappedStrategy,
    card_count: i32,
) -> WrappedStrategy {
    let mut result = optimize_draw(blackjack_strategy, card_count).await;
    let _deck = CountedDeck::new(card_count);

    // then optimize double down
    result = optimize_double_down(result.clone(), card_count).await;

    // then optimize split
    optimize_split(result.clone(), card_count).await
}
