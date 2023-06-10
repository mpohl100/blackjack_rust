use std::collections::BTreeMap;

pub use crate::blackjack::traits::BlackjackStrategyTrait;
pub use crate::blackjack::blackjack_situation::HandSituation;
pub use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::blackjack_strategy::CountedBlackjackStrategy;
use crate::blackjack::blackjack_challenge::BlackjackChallenge;
use crate::blackjack::blackjack_challenge::BlackjackChallengeType;
use crate::blackjack::card::BlackjackRank;
use crate::blackjack::hand::PlayerHand;
use crate::blackjack::deck::CountedDeck;
use crate::blackjack::card::Card;
use crate::blackjack::card::Rank;
use crate::blackjack::card::Suit;
use crate::blackjack::evaluate_blackjack_hand::evaluate_blackjack_hand;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

struct BlackjackGameSituation<'a> {
    pub hand_situation: Option<HandSituation>,
    pub is_draw: bool,
    pub split_situation: Option<SplitSituation>,
    pub strat:  &'a mut dyn BlackjackStrategyTrait,
}

fn get_dealer_rank(challenge_type: BlackjackChallengeType, situation: &BlackjackGameSituation) -> BlackjackRank {
    match challenge_type {
        BlackjackChallengeType::Split => match situation.split_situation {
            Some(value) => { value.dealer_card() }
            None => panic!("no split situation found.")
        }
        _ => match situation.hand_situation {
            Some(value) => { value.dealer_card() }
            None => panic!("no hand situation found.")
        }
    }
}

fn get_player_hand(challenge_type: BlackjackChallengeType, situation: &BlackjackGameSituation) -> PlayerHand {
    let mut ret = PlayerHand::default();
    match challenge_type {
        BlackjackChallengeType::Split => {
            let representative_card = match situation.split_situation { 
                Some(value) => { value.situation().get_representative_card() }
                None => panic!("no split situation found")
            };
            ret.add_card(&representative_card.clone());
            ret.add_card(&representative_card.clone());
        },
        _ => {
            let mut goal_points = match situation.hand_situation {
                Some(value) => { value.situation().lower() }
                None => panic!("no hand situation found")
            };
            let upper_points = match situation.hand_situation {
                Some(value) => { value.situation().upper() }
                None => panic!("no hand situation found")
            };
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
                        _ => panic!("Unexpected goal points in generation of player hand: {}", goal_points),
                    }
                    break;
                }
            }
            let to_check = evaluate_blackjack_hand(&ret.get_blackjack_hand());
            if to_check != match situation.hand_situation {
                Some(value) => { value.situation() } 
                None => panic!("no hand_situation found!")
            } {
                panic!("incorrect player hand formed.");
            }
        },
    }
    ret
}


fn optimize_situation(situation: &mut BlackjackGameSituation, deck: &CountedDeck) -> bool
{
    let situationtype = if situation.split_situation.is_some() {
        BlackjackChallengeType::Split
    } else if situation.is_draw {
        BlackjackChallengeType::Draw
    } else {
        BlackjackChallengeType::DoubleDown
    };
    let boxed_deck = Box::new(deck.clone());
    let mut challenge = BlackjackChallenge::new(situationtype.clone(), get_dealer_rank(situationtype.clone(), situation), get_player_hand(situationtype.clone(), situation), situation.strat, boxed_deck);
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

fn calculate_draw<BlackjackStrategyType>(hand_situations: Vec<HandSituation>, deck: CountedDeck, blackjack_strategy: BlackjackStrategyType) -> BlackjackStrategyType
where BlackjackStrategyType: BlackjackStrategyTrait + Clone
{
    let mut result = blackjack_strategy.clone();
    for hand_situation in hand_situations.iter().rev() {
        let mut situation = BlackjackGameSituation {
            is_draw: true,
            strat: &mut result.clone(),
            hand_situation: Some(*hand_situation),
            split_situation: None,
        };
        result.add_draw(*hand_situation, optimize_situation(&mut situation, &deck));        
    }
    result
}

fn optimize_draw<BlackjackStrategyType>(blackjack_strategy: BlackjackStrategyType, card_count: i32) -> BlackjackStrategyType
where BlackjackStrategyType: BlackjackStrategyTrait + Clone + Send + 'static
{
    let mut result = blackjack_strategy.clone();
    let deck = CountedDeck::new( card_count );
    // first optimize drawing
    let all_situations = HandSituation::create_all();
    // bucketize
    let mut buckets = BTreeMap::<BlackjackRank, Vec<HandSituation>>::new();
    for sit in all_situations{
        let val = buckets.get_mut(&sit.dealer_card());
        if let Some(b) = val{
            b.push(sit);
        }
        else{
            buckets.insert(sit.dealer_card(), vec![sit]);
        }
    }
    // schedule work
    let pool = ThreadPool::new(2);
    let (transaction, receiver) = channel();
    for (_, bucket ) in buckets.iter(){
        let tr_clone = transaction.clone();
        let bucket_clone = bucket.clone();
        let deck_clone = deck.clone();
        let result_clone = result.clone();
        pool.execute(move || {
            let bucket_result = calculate_draw(bucket_clone, deck_clone, result_clone);
            tr_clone.send(bucket_result).expect("Could not send bucket");
        });
    }
    // receive results
    for (_,_) in buckets.iter(){
        let bucket_result = receiver.recv().expect("Did not receive blackjack strategy bucket calculation");
        result.combine(&bucket_result.dump());
    }
    result
}

pub fn optimize_blackjack<BlackjackStrategyType>(blackjack_strategy: BlackjackStrategyType, card_count: i32) -> impl BlackjackStrategyTrait
where BlackjackStrategyType: BlackjackStrategyTrait + Clone + Send + 'static
{
    let mut result = optimize_draw(blackjack_strategy, card_count).clone();
    let deck = CountedDeck::new( card_count );
    
    // then optimize double down
    for hand_situation in HandSituation::create_all().iter().rev() {
        let mut situation = BlackjackGameSituation {
            is_draw: false,
            strat: &mut result.clone(),
            hand_situation: Some(*hand_situation),
            split_situation: None,
        };
        result.add_double_down(*hand_situation, optimize_situation(&mut situation, &deck));
    }

    // then optimize split
    for split_situation in SplitSituation::create_all().iter() {
        let mut situation = BlackjackGameSituation {
            is_draw: false,
            strat: &mut result.clone(),
            hand_situation: None,
            split_situation: Some(*split_situation),
        };
        result.add_split(*split_situation, optimize_situation(&mut situation, &deck));
    }
    result
}

pub fn optimize_counted<BlackjackStrategyType>(blackjack_strategy:BlackjackStrategyType) -> impl BlackjackStrategyTrait
where BlackjackStrategyType: BlackjackStrategyTrait + Clone + 'static + Send
{
    let mut data = BTreeMap::<i32, Box<dyn BlackjackStrategyTrait>>::new();
    for i in -10..11{
        let strat = optimize_blackjack(blackjack_strategy.clone(), i);
        data.insert(i, Box::new(strat));
    }
    CountedBlackjackStrategy::new(data)
}
