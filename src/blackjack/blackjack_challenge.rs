use super::deck::WrappedDeck;
use super::rng::RandomNumberGenerator;

use crate::blackjack::blackjack_situation::GameSituation;
use crate::blackjack::card::BlackjackRank;
use crate::blackjack::evaluate_blackjack_hand::evaluate_blackjack_hand;
use crate::blackjack::hand::DealerHand;
use crate::blackjack::hand::PlayerHand;
use crate::blackjack::play_blackjack_hand::play_blackjack_hand;
use crate::blackjack::play_blackjack_hand::PlayMode;

use crate::blackjack::traits::WrappedGame;
use crate::blackjack::traits::WrappedStrategy;

pub struct BlackjackChallenge {
    game_situation_: GameSituation,
    dealer_rank: BlackjackRank,
    player_hand: PlayerHand,
    strat: WrappedStrategy,
    deck: WrappedDeck,
}

impl BlackjackChallenge {
    pub fn new(
        game_situation: GameSituation,
        dealer_card: BlackjackRank,
        player_hand: PlayerHand,
        strat: WrappedStrategy,
        deck: WrappedDeck,
    ) -> BlackjackChallenge {
        BlackjackChallenge {
            game_situation_: game_situation,
            dealer_rank: dealer_card,
            player_hand: player_hand.clone(),
            strat,
            deck,
        }
    }

    pub async fn score(&mut self, do_it: bool) -> f64 {
        let _points = evaluate_blackjack_hand(&self.player_hand.get_blackjack_hand());
        match self.game_situation_ {
            GameSituation::Draw(hand_situation) => self.strat.add_draw(hand_situation, do_it).await,
            GameSituation::DoubleDown(hand_situation) => {
                self.strat.add_double_down(hand_situation, do_it).await
            }
            GameSituation::Split(split_situation) => {
                self.strat.add_split(split_situation, do_it).await
            }
        }
        let mut rng = RandomNumberGenerator::new();
        let mut result = 0.0;
        let play_mode = self.get_play_mode();
        let blackjack_game = WrappedGame::new_from_strat(&mut self.strat).await;
        for _ in 0..2000 {
            let dealer_hand = DealerHand::new(&[
                self.dealer_rank.get_representative_card(),
                self.deck.deal_card(&mut rng),
            ]);
            result += play_blackjack_hand(
                1.0,
                self.player_hand.clone(),
                dealer_hand,
                &mut self.deck,
                blackjack_game.clone(),
                &mut rng,
                play_mode,
            )
            .await;
        }
        result
    }

    fn get_play_mode(&self) -> PlayMode {
        match self.game_situation_ {
            GameSituation::Draw(_) => PlayMode::Draw,
            GameSituation::DoubleDown(_) => PlayMode::DoubleDown,
            GameSituation::Split(_) => PlayMode::All,
        }
    }
}
