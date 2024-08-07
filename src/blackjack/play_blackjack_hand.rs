use crate::blackjack::card::BlackjackRank;
use crate::blackjack::deck::WrappedDeck;
use crate::blackjack::evaluate_blackjack_hand::evaluate_blackjack_hand;
use crate::blackjack::hand::DealerHand;
use crate::blackjack::hand::PlayerHand;
use crate::blackjack::rng::RandomNumberGenerator;
use crate::blackjack::traits::WrappedGame;

use super::analysis::blackjack_analysis::HandSituation;
use super::analysis::blackjack_analysis::SplitSituation;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlayMode {
    All,
    DoubleDown,
    Draw,
}

async fn get_play_result(
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
    get_play_result(player_bet, player_result, dealer_result, player_hand).await
}

pub struct PlayerHandData {
    pub player_hand: PlayerHand,
    pub player_bet: f64,
    pub is_won: Option<bool>,
}

impl PlayerHandData {
    pub fn new(player_hand: PlayerHand, player_bet: f64) -> PlayerHandData {
        PlayerHandData {
            player_hand,
            player_bet,
            is_won: None,
        }
    }
}

#[async_trait]
pub trait HandData {
    async fn play_dealer_hand(
        &mut self,
        deck: &mut WrappedDeck,
        rng: &mut RandomNumberGenerator,
    ) -> i32;
    async fn get_active_hand(&mut self) -> &mut PlayerHand;
    async fn get_dealer_hand(&mut self) -> &mut DealerHand;
    async fn remove_active_hand(&mut self) -> PlayerHandData;
    async fn add_player_hand(&mut self, hand: PlayerHandData);
    async fn set_active_hand(&mut self, index: i32);
    async fn get_active_index(&self) -> i32;
    async fn set_active_bet(&mut self, bet: f64);
    async fn get_active_bet(&self) -> f64;
    async fn send_game_info(&mut self, active_hand_finished: bool);
    async fn book_amount(&mut self, amount: f64);
    async fn play_dealer(&mut self, deck: &mut WrappedDeck, rng: &mut RandomNumberGenerator);
    async fn get_current_balance(&self) -> f64;
}

pub struct WrappedHandData {
    pub hand_data: Arc<Mutex<Box<dyn HandData + Send>>>,
}

impl WrappedHandData {
    pub fn new(data: Box<dyn HandData + Send>) -> WrappedHandData {
        WrappedHandData {
            hand_data: Arc::new(Mutex::new(data)),
        }
    }
}

pub struct HandInfo {
    player_hands: Vec<PlayerHandData>,
    active_hand: i32,
    dealer_hand: DealerHand,
    current_balance: f64,
}

impl HandInfo {
    pub fn new(
        player_bet: f64,
        current_balance: f64,
        deck: &mut WrappedDeck,
        rng: &mut RandomNumberGenerator,
    ) -> HandInfo {
        let player_hand = PlayerHand::new(&[deck.deal_card(rng), deck.deal_card(rng)]);
        let dealer_hand = DealerHand::new(&[deck.deal_card(rng), deck.deal_card(rng)]);

        HandInfo {
            player_hands: vec![PlayerHandData::new(player_hand, player_bet)],
            active_hand: 0,
            dealer_hand,
            current_balance,
        }
    }

    pub fn get_player_hands(&self) -> &Vec<PlayerHandData> {
        &self.player_hands
    }
}

#[async_trait]
impl HandData for HandInfo {
    async fn play_dealer_hand(
        &mut self,
        deck: &mut WrappedDeck,
        rng: &mut RandomNumberGenerator,
    ) -> i32 {
        self.dealer_hand.play(deck, rng)
    }

    async fn get_active_hand(&mut self) -> &mut PlayerHand {
        if self.active_hand < 0 || self.active_hand >= self.player_hands.len().try_into().unwrap() {
            panic!("Invalid active hand index");
        }
        let index = self.active_hand as usize;
        &mut self.player_hands[index].player_hand
    }

    async fn get_dealer_hand(&mut self) -> &mut DealerHand {
        &mut self.dealer_hand
    }

    async fn remove_active_hand(&mut self) -> PlayerHandData {
        if self.active_hand < 0 || self.active_hand >= self.player_hands.len().try_into().unwrap() {
            panic!("Invalid active hand index");
        }
        let removed_hand = self.player_hands.remove(self.active_hand as usize);
        self.active_hand -= 1;
        removed_hand
    }

    async fn add_player_hand(&mut self, hand: PlayerHandData) {
        if self.active_hand < -1 || self.active_hand >= self.player_hands.len().try_into().unwrap()
        {
            panic!("Invalid active hand index");
        }
        // add player hand to the right of the active hand
        self.player_hands
            .insert((self.active_hand + 1) as usize, hand);
    }

    async fn set_active_hand(&mut self, index: i32) {
        if index < 0 || index >= self.player_hands.len().try_into().unwrap() {
            panic!("Invalid active hand index");
        }
        self.active_hand = index;
    }

    async fn get_active_index(&self) -> i32 {
        self.active_hand
    }

    async fn set_active_bet(&mut self, bet: f64) {
        if self.active_hand < 0 || self.active_hand >= self.player_hands.len().try_into().unwrap() {
            panic!("Invalid active hand index");
        }
        self.player_hands[self.active_hand as usize].player_bet = bet;
    }

    async fn get_active_bet(&self) -> f64 {
        if self.active_hand < 0 || self.active_hand >= self.player_hands.len().try_into().unwrap() {
            panic!("Invalid active hand index");
        }
        self.player_hands[self.active_hand as usize].player_bet
    }

    async fn send_game_info(&mut self, _active_hand_finished: bool) {
        // do nothing
    }

    async fn book_amount(&mut self, amount: f64) {
        self.current_balance += amount;
    }

    async fn play_dealer(&mut self, deck: &mut WrappedDeck, rng: &mut RandomNumberGenerator) {
        let dealer_result = self.dealer_hand.play(deck, rng);
        for hand in &mut self.player_hands {
            let player_points = evaluate_blackjack_hand(&hand.player_hand.get_blackjack_hand());
            let player_result = player_points.upper();
            let result = get_play_result(
                hand.player_bet,
                player_result,
                dealer_result,
                hand.player_hand.clone(),
            )
            .await;
            hand.is_won = Some(result > 0.0);
            if result > 0.0 {
                self.current_balance += hand.player_bet + result;
            }
        }
    }

    async fn get_current_balance(&self) -> f64 {
        self.current_balance
    }
}

pub async fn play_blackjack_hand_new(
    hand_data: &mut WrappedHandData,
    deck: &mut WrappedDeck,
    player_strategy: WrappedGame,
    rng: &mut RandomNumberGenerator,
    play_mode: PlayMode,
) {
    let initial_bet = hand_data.hand_data.lock().await.get_active_bet().await;
    hand_data
        .hand_data
        .lock()
        .await
        .book_amount(-initial_bet)
        .await;
    hand_data.hand_data.lock().await.send_game_info(false).await;

    // add code for splitting here
    if play_mode == PlayMode::All
        && hand_data
            .hand_data
            .lock()
            .await
            .get_active_hand()
            .await
            .is_pair()
    {
        // splitting hands is allowed
        let rank = BlackjackRank::new(
            hand_data
                .hand_data
                .lock()
                .await
                .get_active_hand()
                .await
                .get_cards()[0]
                .rank(),
        );
        let do_split = player_strategy.get_split(
            SplitSituation::new(
                rank,
                hand_data
                    .hand_data
                    .lock()
                    .await
                    .get_dealer_hand()
                    .await
                    .open_card(),
            ),
            deck,
        );
        if do_split.await {
            let first = PlayerHand::new(&[
                hand_data
                    .hand_data
                    .lock()
                    .await
                    .get_active_hand()
                    .await
                    .get_cards()[0],
                deck.deal_card(rng),
            ]);
            let second = PlayerHand::new(&[
                hand_data
                    .hand_data
                    .lock()
                    .await
                    .get_active_hand()
                    .await
                    .get_cards()[1],
                deck.deal_card(rng),
            ]);
            let active_index = hand_data.hand_data.lock().await.get_active_index().await;
            let old_active_hand = hand_data.hand_data.lock().await.remove_active_hand().await;
            hand_data
                .hand_data
                .lock()
                .await
                .add_player_hand(PlayerHandData::new(first, old_active_hand.player_bet))
                .await;
            hand_data
                .hand_data
                .lock()
                .await
                .set_active_hand(active_index)
                .await;
            hand_data.hand_data.lock().await.send_game_info(false).await;
            hand_data
                .hand_data
                .lock()
                .await
                .add_player_hand(PlayerHandData::new(second, old_active_hand.player_bet))
                .await;
            // refund the initial bet as they are booked again inside the function calls
            hand_data
                .hand_data
                .lock()
                .await
                .book_amount(initial_bet)
                .await;
            hand_data.hand_data.lock().await.send_game_info(false).await;
            Box::pin(play_blackjack_hand_new(
                hand_data,
                deck,
                player_strategy.clone(),
                rng,
                play_mode,
            ))
            .await;
            let active_index = hand_data.hand_data.lock().await.get_active_index().await;
            hand_data
                .hand_data
                .lock()
                .await
                .set_active_hand(active_index + 1)
                .await;
            hand_data.hand_data.lock().await.send_game_info(false).await;
            Box::pin(play_blackjack_hand_new(
                hand_data,
                deck,
                player_strategy,
                rng,
                play_mode,
            ))
            .await;
            return;
        }
    }

    let player_points;
    let mut only_draw_once = false;
    if play_mode == PlayMode::All || play_mode == PlayMode::DoubleDown {
        player_points = evaluate_blackjack_hand(
            &hand_data
                .hand_data
                .lock()
                .await
                .get_active_hand()
                .await
                .get_blackjack_hand(),
        );
        only_draw_once = player_strategy
            .get_double_down(
                HandSituation::new(
                    player_points,
                    hand_data
                        .hand_data
                        .lock()
                        .await
                        .get_dealer_hand()
                        .await
                        .open_card(),
                ),
                deck,
            )
            .await;
        if only_draw_once {
            let current_active_bet = hand_data.hand_data.lock().await.get_active_bet().await;
            hand_data
                .hand_data
                .lock()
                .await
                .set_active_bet(current_active_bet * 2.0)
                .await;
            hand_data
                .hand_data
                .lock()
                .await
                .book_amount(-current_active_bet)
                .await;
            hand_data.hand_data.lock().await.send_game_info(false).await;
        }
    }

    if only_draw_once {
        hand_data
            .hand_data
            .lock()
            .await
            .get_active_hand()
            .await
            .add_card(&deck.deal_card(rng));
        hand_data.hand_data.lock().await.send_game_info(false).await;
    } else {
        loop {
            let player_points = evaluate_blackjack_hand(
                &hand_data
                    .hand_data
                    .lock()
                    .await
                    .get_active_hand()
                    .await
                    .get_blackjack_hand(),
            );
            if player_points.lower() > 21 {
                break;
            }
            let draw = player_strategy
                .get_draw(
                    HandSituation::new(
                        player_points,
                        hand_data
                            .hand_data
                            .lock()
                            .await
                            .get_dealer_hand()
                            .await
                            .open_card(),
                    ),
                    deck,
                )
                .await;
            if !draw {
                break;
            }
            hand_data
                .hand_data
                .lock()
                .await
                .get_active_hand()
                .await
                .add_card(&deck.deal_card(rng));
            hand_data.hand_data.lock().await.send_game_info(false).await;
        }
    }

    // send game info after the hand is played
    hand_data.hand_data.lock().await.send_game_info(true).await;
}
