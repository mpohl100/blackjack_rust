use crate::blackjack::card::BlackjackRank;
use crate::blackjack::hand::PlayerHand;
use crate::blackjack::hand::DealerHand;
use crate::blackjack::deck::Deck;
use crate::blackjack::play_blackjack_hand::PlayMode;
use crate::blackjack::evaluate_blackjack_hand::evaluate_blackjack_hand;
use crate::blackjack::play_blackjack_hand::play_blackjack_hand;
use crate::blackjack::traits::BlackjackStrategyTrait;
use super::{blackjack_analysis::{HandSituation, SplitSituation}, rng::RandomNumberGenerator};

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Debug)]
pub enum BlackjackChallengeType {
    Draw,
    DoubleDown,
    Split,
}

pub struct BlackjackChallenge<'a> {
    type_: BlackjackChallengeType,
    dealer_rank: BlackjackRank,
    player_hand: PlayerHand,
    strat: &'a mut dyn BlackjackStrategyTrait,
    deck: Box<dyn Deck>,
}

impl BlackjackChallenge<'_>{
    pub fn new(
        situationtype: BlackjackChallengeType,
        dealer_card: BlackjackRank,
        player_hand: PlayerHand,
        strat: &mut dyn BlackjackStrategyTrait,
        deck: Box<dyn Deck>,
    ) -> BlackjackChallenge {
        BlackjackChallenge{
            type_: situationtype,
            dealer_rank: dealer_card,
            player_hand: player_hand.clone(),
            strat,
            deck,
        }
    }

    pub fn score(&mut self, do_it: bool) -> f64
    {
        let points = evaluate_blackjack_hand(&self.player_hand.get_blackjack_hand());
        if self.type_ == BlackjackChallengeType::Draw {
            self.strat.add_draw(HandSituation::new(points, self.dealer_rank), do_it);
        } else if self.type_ == BlackjackChallengeType::DoubleDown {
            self.strat.add_double_down(HandSituation::new(points, self.dealer_rank), do_it);
        } else if self.type_ == BlackjackChallengeType::Split {
            self.strat.add_split(SplitSituation::new(BlackjackRank::new(self.player_hand.get_cards()[0].rank()), self.dealer_rank), do_it);
        }
        let mut rng = RandomNumberGenerator::new();
        let mut result = 0.0;
        for _ in 0..2000 {
            let dealer_hand = DealerHand::new(&vec![self.dealer_rank.get_representative_card(), self.deck.deal_card(&mut rng)]);
            let play_mode = self.get_play_mode();
            result += play_blackjack_hand(1.0, self.player_hand.clone(), dealer_hand, &mut self.deck, self.strat, &mut rng, play_mode);
        }
        result        
    }

    fn get_play_mode(&self) -> PlayMode
    {
        match self.type_{
            BlackjackChallengeType::Draw => PlayMode::Draw,
            BlackjackChallengeType::DoubleDown => PlayMode::DoubleDown,
            BlackjackChallengeType::Split => PlayMode::All,
        }
    }
}
