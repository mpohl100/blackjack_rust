use crate::blackjack::play_blackjack_hand::play_blackjack_hand;
use crate::blackjack::play_blackjack_hand::PlayMode;
use crate::blackjack::deck::CountedDeck;
use crate::blackjack::hand::PlayerHand;
use crate::blackjack::hand::DealerHand;
use crate::blackjack::rng::RandomNumberGenerator;
use crate::blackjack::deck::Deck;
use crate::blackjack::traits::BlackjackStrategyTrait;
use crate::blackjack::deck::EightDecks;

pub fn play_blackjack(n: i32, blackjack_strategy: &dyn BlackjackStrategyTrait, play_normal: bool) -> f64{
    let mut boxed_deck: Box<dyn Deck>;
    if play_normal{
        boxed_deck = Box::new(EightDecks::new());
    } 
    else {
        boxed_deck = Box::new(CountedDeck::new(0));
    }
    let mut rng = RandomNumberGenerator::new();
    let mut result = 0.0;
    for _ in 0..n{
        let player_hand = PlayerHand::new(&vec![boxed_deck.deal_card(&mut rng), boxed_deck.deal_card(&mut rng)]);
        let dealer_hand = DealerHand::new(&vec![boxed_deck.deal_card(&mut rng), boxed_deck.deal_card(&mut rng)]);
        result += play_blackjack_hand(1.0,player_hand, dealer_hand, &mut boxed_deck, blackjack_strategy, &mut rng, PlayMode::All);
    }
    result
}