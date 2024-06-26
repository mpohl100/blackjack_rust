use crate::blackjack::card::BlackjackRank;
use crate::blackjack::deck::WrappedDeck;
use crate::blackjack::evaluate_blackjack_hand::evaluate_blackjack_hand;
use crate::blackjack::hand::DealerHand;
use crate::blackjack::hand::PlayerHand;
use crate::blackjack::rng::RandomNumberGenerator;
use crate::blackjack::traits::WrappedGame;

use super::analysis::blackjack_analysis::HandSituation;
use super::analysis::blackjack_analysis::SplitSituation;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlayMode {
    All,
    DoubleDown,
    Draw,
}

fn get_play_result(
    player_bet: f64,
    player_result: i32,
    dealer_result: i32,
    player_hand: PlayerHand,
) -> f64 {
    if player_result > 21 {
        return -player_bet;
    }
    if player_result == 21 && player_hand.get_cards().len() == 2 {
        return 1.5 * player_bet;
    }
    if dealer_result == -1 {
        return player_bet;
    }
    if player_result > dealer_result {
        return player_bet;
    }
    if player_result == dealer_result {
        return 0.0;
    }
    -player_bet
}

pub async fn play_blackjack_hand(
    mut player_bet: f64,
    mut player_hand: PlayerHand,
    mut dealer_hand: DealerHand,
    deck: &mut WrappedDeck,
    player_strategy: WrappedGame,
    rng: &mut RandomNumberGenerator,
    play_mode: PlayMode,
) -> f64 {
    // play dealer hand at the beginning so that recursive versions for splitting use the same dealer outcome
    let dealer_result = dealer_hand.play(deck, rng);

    // add code for splitting here
    if play_mode == PlayMode::All && player_hand.is_pair() {
        // splitting hands is allowed
        let rank = BlackjackRank::new(player_hand.get_cards()[0].rank());
        let do_split =
            player_strategy.get_split(SplitSituation::new(rank, dealer_hand.open_card()), deck);
        if do_split.await {
            let first = PlayerHand::new(&[player_hand.get_cards()[0], deck.deal_card(rng)]);
            let second = PlayerHand::new(&[player_hand.get_cards()[1], deck.deal_card(rng)]);
            let mut overall_result = 0.0;
            overall_result += Box::pin(play_blackjack_hand(
                player_bet,
                first,
                dealer_hand.clone(),
                deck,
                player_strategy.clone(),
                rng,
                play_mode,
            ))
            .await;
            overall_result += Box::pin(play_blackjack_hand(
                player_bet,
                second,
                dealer_hand.clone(),
                deck,
                player_strategy,
                rng,
                play_mode,
            ))
            .await;
            return overall_result;
        }
    }

    let mut player_points;
    let mut only_draw_once = false;
    if play_mode == PlayMode::All || play_mode == PlayMode::DoubleDown {
        player_points = evaluate_blackjack_hand(&player_hand.get_blackjack_hand());
        only_draw_once = player_strategy
            .get_double_down(
                HandSituation::new(player_points, dealer_hand.open_card()),
                deck,
            )
            .await;
        if only_draw_once {
            player_bet *= 2.0;
        }
    }

    if only_draw_once {
        player_hand.add_card(&deck.deal_card(rng));
        player_points = evaluate_blackjack_hand(&player_hand.get_blackjack_hand());
    } else {
        loop {
            player_points = evaluate_blackjack_hand(&player_hand.get_blackjack_hand());
            if player_points.lower() > 21 {
                break;
            }
            let draw = player_strategy
                .get_draw(
                    HandSituation::new(player_points, dealer_hand.open_card()),
                    deck,
                )
                .await;
            if !draw {
                break;
            }
            player_hand.add_card(&deck.deal_card(rng));
        }
    }
    // deduce player result
    let player_result = player_points.upper();

    // compare player and dealer hands
    get_play_result(player_bet, player_result, dealer_result, player_hand)
}
