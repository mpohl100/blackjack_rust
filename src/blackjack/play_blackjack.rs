use crate::blackjack::blackjack_configuration::PlayConfiguration;
use crate::blackjack::deck::CountedDeck;
use crate::blackjack::deck::Deck;
use crate::blackjack::deck::EightDecks;
use crate::blackjack::hand::DealerHand;
use crate::blackjack::hand::PlayerHand;
use crate::blackjack::play_blackjack_hand::play_blackjack_hand;
use crate::blackjack::play_blackjack_hand::PlayMode;
use crate::blackjack::rng::RandomNumberGenerator;
use crate::blackjack::traits::BlackjackStrategyTrait;

pub fn play_blackjack(
    play_config: PlayConfiguration,
    blackjack_strategy: &dyn BlackjackStrategyTrait,
) -> f64 {
    let mut boxed_deck: Box<dyn Deck> = match play_config.play_normal {
        true => Box::new(EightDecks::new()),
        false => Box::new(CountedDeck::new(0)),
    };
    let mut rng = RandomNumberGenerator::new();
    let mut result = 0.0;
    for _ in 0..play_config.nb_hands {
        let player_hand = PlayerHand::new(&[
            boxed_deck.deal_card(&mut rng),
            boxed_deck.deal_card(&mut rng),
        ]);
        let dealer_hand = DealerHand::new(&[
            boxed_deck.deal_card(&mut rng),
            boxed_deck.deal_card(&mut rng),
        ]);
        result += play_blackjack_hand(
            1.0,
            player_hand,
            dealer_hand,
            &mut *boxed_deck,
            blackjack_strategy,
            &mut rng,
            PlayMode::All,
        );
    }
    result
}
