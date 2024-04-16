use std::collections::BTreeMap;

use crate::blackjack::blackjack_challenge::BlackjackChallenge;
use crate::blackjack::blackjack_configuration::StrategyConfiguration;
use crate::blackjack::blackjack_situation::GameSituation;
pub use crate::blackjack::blackjack_situation::HandSituation;
pub use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::card::BlackjackRank;
use crate::blackjack::card::Card;
use crate::blackjack::card::Rank;
use crate::blackjack::card::Suit;
use crate::blackjack::deck::CountedDeck;
use crate::blackjack::evaluate_blackjack_hand::evaluate_blackjack_hand;
use crate::blackjack::hand::PlayerHand;
use crate::blackjack::strategy::blackjack_strategy::CountedBlackjackStrategy;
pub use crate::blackjack::traits::BlackjackStrategyTrait;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

struct BlackjackGameSituation<'a> {
    pub game_situation: GameSituation,
    pub strat: &'a mut dyn BlackjackStrategyTrait,
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

fn optimize_situation(situation: &mut BlackjackGameSituation, deck: &CountedDeck) -> bool {
    let boxed_deck = Box::new(deck.clone());
    let mut challenge = BlackjackChallenge::new(
        situation.game_situation,
        get_dealer_rank(situation.game_situation),
        get_player_hand(situation.game_situation),
        situation.strat,
        boxed_deck,
    );
    let dont = false;
    let do_it = true;
    let score_dont = challenge.score(dont);
    let score_do_it = challenge.score(do_it);

    if score_do_it > score_dont {
        do_it
    } else {
        dont
    }
}

fn calculate_draw<BlackjackStrategyType>(
    hand_situations: Vec<HandSituation>,
    deck: CountedDeck,
    blackjack_strategy: BlackjackStrategyType,
) -> BlackjackStrategyType
where
    BlackjackStrategyType: BlackjackStrategyTrait + Clone,
{
    let mut result = blackjack_strategy.clone();
    for hand_situation in hand_situations.iter().rev() {
        let mut situation = BlackjackGameSituation {
            game_situation: GameSituation::Draw(*hand_situation),
            strat: &mut result.clone(),
        };
        result.add_draw(*hand_situation, optimize_situation(&mut situation, &deck));
    }
    result
}

fn optimize_draw<BlackjackStrategyType>(
    blackjack_strategy: BlackjackStrategyType,
    thread_pool: &ThreadPool,
    card_count: i32,
) -> BlackjackStrategyType
where
    BlackjackStrategyType: BlackjackStrategyTrait + Clone + Send + 'static,
{
    let mut result = blackjack_strategy.clone();
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
    let (transaction, receiver) = channel();
    for (_, bucket) in buckets.iter() {
        let tr_clone = transaction.clone();
        let bucket_clone = bucket.clone();
        let deck_clone = deck.clone();
        let result_clone = result.clone();
        thread_pool.execute(move || {
            let bucket_result = calculate_draw(bucket_clone, deck_clone, result_clone);
            tr_clone.send(bucket_result).expect("Could not send bucket");
        });
    }
    // receive results
    for (_, _) in buckets.iter() {
        let bucket_result = receiver
            .recv()
            .expect("Did not receive blackjack strategy bucket calculation");
        result.combine(&bucket_result.dump());
    }
    result
}

fn optimize_double_down<BlackjackStrategyType>(
    blackjack_strategy: BlackjackStrategyType,
    thread_pool: &ThreadPool,
    card_count: i32,
) -> BlackjackStrategyType
where
    BlackjackStrategyType: BlackjackStrategyTrait + Clone + Send + 'static,
{
    let mut result = blackjack_strategy.clone();
    let deck = CountedDeck::new(card_count);
    let (transaction, receiver) = channel();
    for hand_situation in HandSituation::create_all() {
        let tr_clone = transaction.clone();
        let deck_clone = deck.clone();
        let result_clone = result.clone();
        let hand_situation_clone = hand_situation;
        thread_pool.execute(move || {
            let mut situation = BlackjackGameSituation {
                game_situation: GameSituation::DoubleDown(hand_situation_clone),
                strat: &mut result_clone.clone(),
            };
            let do_it = optimize_situation(&mut situation, &deck_clone);
            tr_clone
                .send((hand_situation_clone, do_it))
                .expect("Could not send double down result")
        });
    }
    for _ in HandSituation::create_all() {
        let (hand_situation, do_it) = receiver
            .recv()
            .expect("Did not receive blackjack strategy double down calculation");
        result.add_double_down(hand_situation, do_it);
    }
    result
}

fn optimize_split<BlackjackStrategyType>(
    blackjack_strategy: BlackjackStrategyType,
    thread_pool: &ThreadPool,
    card_count: i32,
) -> BlackjackStrategyType
where
    BlackjackStrategyType: BlackjackStrategyTrait + Clone + Send + 'static,
{
    let mut result = blackjack_strategy.clone();
    let deck = CountedDeck::new(card_count);
    let (transaction, receiver) = channel();
    for split_situation in SplitSituation::create_all() {
        let tr_clone = transaction.clone();
        let deck_clone = deck.clone();
        let result_clone = result.clone();
        let split_situation_clone = split_situation;
        thread_pool.execute(move || {
            let mut situation = BlackjackGameSituation {
                game_situation: GameSituation::Split(split_situation_clone),
                strat: &mut result_clone.clone(),
            };
            let do_it = optimize_situation(&mut situation, &deck_clone);
            tr_clone
                .send((split_situation_clone, do_it))
                .expect("Could not send split result")
        });
    }
    for _ in SplitSituation::create_all() {
        let (split_situation, do_it) = receiver
            .recv()
            .expect("Did not receive blackjack strategy split calculation");
        result.add_split(split_situation, do_it);
    }
    result
}

pub fn optimize_blackjack<BlackjackStrategyType>(
    blackjack_strategy: BlackjackStrategyType,
    thread_pool: &ThreadPool,
    card_count: i32,
) -> impl BlackjackStrategyTrait
where
    BlackjackStrategyType: BlackjackStrategyTrait + Clone + Send + 'static,
{
    let mut result = optimize_draw(blackjack_strategy, thread_pool, card_count).clone();
    let _deck = CountedDeck::new(card_count);

    // then optimize double down
    result = optimize_double_down(result.clone(), thread_pool, card_count);

    // then optimize split
    optimize_split(result.clone(), thread_pool, card_count)
}

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
