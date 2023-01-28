pub use crate::blackjack::traits::BlackjackStrategyTrait;
pub use crate::blackjack::blackjack_situation::HandSituation;
pub use crate::blackjack::blackjack_situation::SplitSituation;
use crate::blackjack::blackjack_strategy::BlackjackStrategy;
use crate::blackjack::blackjack_challenge::BlackjackChallenge;
use crate::blackjack::blackjack_challenge::BlackjackChallengeType;
use crate::blackjack::blackjack_points::Points;
use crate::blackjack::card::BlackjackRank;
use crate::blackjack::hand::PlayerHand;
use crate::blackjack::deck::CountedDeck;
use crate::blackjack::traits::Allable;
use crate::blackjack::card::Card;
use crate::blackjack::card::Rank;
use crate::blackjack::card::Suit;
use crate::blackjack::evaluate_blackjack_hand::evaluate_blackjack_hand;

use std::rc::Rc;
struct BlackjackGameSituation {
    pub hand_situation: Option<HandSituation>,
    pub is_draw: bool,
    pub split_situation: Option<SplitSituation>,
    pub strat:  Rc<dyn BlackjackStrategyTrait>,
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
    let mut challenge = BlackjackChallenge::new(situationtype.clone(), get_dealer_rank(situationtype.clone(), situation), get_player_hand(situationtype.clone(), situation), Rc::clone(&situation.strat), boxed_deck);
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

pub fn optimize_blackjack(card_count: i32) -> Rc<dyn BlackjackStrategyTrait>
{
    let mut result: Rc<dyn BlackjackStrategyTrait> = Rc::new(BlackjackStrategy::new());
    let deck = CountedDeck::new( card_count );
    // first optimize drawing
    for i in (2..=21).rev() {
        let blackjack_ranks = BlackjackRank::create_all();
        for dealer_rank in blackjack_ranks {
            let mut situation = BlackjackGameSituation {
                is_draw: true,
                strat: Rc::clone(&result),
                hand_situation: None,
                split_situation: None,
            };
            let hand_situation = HandSituation::new(
                Points::new(i, i),
                dealer_rank,
            );
            situation.hand_situation = Some(hand_situation);
            result.add_draw(hand_situation, optimize_situation(&mut situation, &deck));

            let hand_situation_upper = HandSituation::new(
                Points::new(i, i + 10),
                dealer_rank,
            );
            situation.hand_situation = Some(hand_situation_upper);
            result.add_draw(hand_situation_upper, optimize_situation(&mut situation, &deck));
        }
    }

    // then optimize double down
    for i in (2..=21).rev() {
        let blackjack_ranks = BlackjackRank::create_all();
        for dealer_rank in blackjack_ranks {
            let mut situation = BlackjackGameSituation {
                is_draw: false,
                strat: Rc::clone(&result),
                hand_situation: None,
                split_situation: None,
            };
            let hand_situation = HandSituation::new(
                Points::new(i, i),
                dealer_rank,
            );
            situation.hand_situation = Some(hand_situation);
            result.add_double_down(hand_situation, optimize_situation(&mut situation, &deck));

            let hand_situation_upper = HandSituation::new(
                Points::new(i, i + 10),
                dealer_rank,
            );
            situation.hand_situation = Some(hand_situation_upper);
            result.add_double_down(hand_situation_upper, optimize_situation(&mut situation, &deck));
        }
    }

    // then optimize split
    let blackjack_ranks = BlackjackRank::create_all();
    for split_rank in blackjack_ranks.clone() {
        for dealer_rank in blackjack_ranks.clone() {
            let mut situation = BlackjackGameSituation {
                is_draw: false,
                strat: Rc::clone(&result),
                hand_situation: None,
                split_situation: None,
            };
            let split_situation = SplitSituation::new(
                split_rank,
                dealer_rank,
            );
            situation.split_situation = Some(split_situation);
            result.add_split(split_situation, optimize_situation(&mut situation, &deck));
        }
    }
    result
}

