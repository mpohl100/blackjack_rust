use crate::blackjack::card::BlackjackRank;
use crate::blackjack::hand::PlayerHand;
use crate::blackjack::blackjack_strategy::BlackjackStrategy;
use crate::blackjack::deck::Deck;
use crate::blackjack::play_blackjack_hand::PlayMode;

#[derive(Clone)]
pub enum BlackjackChallengeType {
    Draw,
    DoubleDown,
    Split,
}

pub struct BlackjackChallenge {
    type_: BlackjackChallengeType,
    dealer_rank: BlackjackRank,
    player_hand: PlayerHand,
    strat: BlackjackStrategy,
    deck: Box<dyn Deck>,
}

impl BlackjackChallenge{
    pub fn new(
        situationtype: BlackjackChallengeType,
        dealer_card: BlackjackRank,
        player_hand: PlayerHand,
        strat: BlackjackStrategy,
        deck: Box<dyn Deck>,
    ) -> BlackjackChallenge {
        BlackjackChallenge{
            type_: situationtype,
            dealer_rank: dealer_card,
            player_hand: player_hand.clone(),
            strat: strat.clone(),
            deck: deck,
        }
    }

    pub fn score(&self, doIt: bool) -> f64
    {
        // implementation
        0.0
    }

    fn get_play_mode(&self) -> PlayMode
    {
        // implementation
        PlayMode::All
    }
}
