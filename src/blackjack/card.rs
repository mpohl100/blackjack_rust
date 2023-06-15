use crate::blackjack::traits::Allable;
use crate::blackjack::traits::Stringable;

#[derive(Debug, Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Rank {
    Deuce,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn new_from_int(i: i32) -> Rank{
        match i{
            0 => return Rank::Deuce,
            1 => return Rank::Three,
            2 => return Rank::Four,
            3 => return Rank::Five,
            4 => return Rank::Six,
            5 => return Rank::Seven,
            6 => return Rank::Eight,
            7 => return Rank::Nine,
            8 => return Rank::Ten,
            9 => return Rank::Jack,
            10 => return Rank::Queen,
            11 => return Rank::King,
            12 => return Rank::Ace,
            _ => panic!("wrong int for Rank"),
        }
    } 

    fn to_blackjack_score(&self) -> i32 {
        match self {
            Rank::Deuce => return 2,
            Rank::Three => return 3,
            Rank::Four => return 4,
            Rank::Five => return 5,
            Rank::Six => return 6,
            Rank::Seven => return 7,
            Rank::Eight => return 8,
            Rank::Nine => return 9,
            Rank::Ten => return 10,
            Rank::Jack => return 10,
            Rank::Queen => return 10,
            Rank::King => return 10,
            Rank::Ace => return 11, // case eleven is handled outside
        }
    }
}

impl Stringable for Rank{
    fn to_string_internal(&self) -> String {
        match self {
            Rank::Deuce => return "2".to_string(),
            Rank::Three => return "3".to_string(),
            Rank::Four => return "4".to_string(),
            Rank::Five => return "5".to_string(),
            Rank::Six => return "6".to_string(),
            Rank::Seven => return "7".to_string(),
            Rank::Eight => return "8".to_string(),
            Rank::Nine => return "9".to_string(),
            Rank::Ten => return "T".to_string(),
            Rank::Jack => return "J".to_string(),
            Rank::Queen => return "Q".to_string(),
            Rank::King => return "K".to_string(),
            Rank::Ace => return "A".to_string(),
        }
    }
}

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl Suit {
    fn new_from_int(i: i32) -> Suit{
        match i {
            0 => return Suit::Hearts,
            1 => return Suit::Diamonds,
            2 => return Suit::Spades,
            3 => return Suit::Clubs,
            _ => panic!("wrong i32 for Suit"),
        }
    }
}

impl Stringable for Suit{
    fn to_string_internal(&self) -> String{
        match self {
            Suit::Hearts => return "h".to_string(),
            Suit::Diamonds => return "d".to_string(),
            Suit::Spades => return "s".to_string(),
            Suit::Clubs => return "c".to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    pub fn new_with_int(i: i32) -> Card{
        Card::new(Rank::new_from_int(i % 13), Suit::new_from_int(i / 13))
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn to_blackjack_score(&self) -> i32 {
        match self.rank {
            Rank::Ace => 1, // case 11 is handled outside
            Rank::Deuce => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlackjackRank {
    val: i32,
}

impl BlackjackRank {
    pub fn new(rank: Rank) -> BlackjackRank {
        BlackjackRank {
            val: rank.to_blackjack_score(),
        }
    }

    pub fn get_representative_card(&self) -> Card {
        match self.val {
            2 => Card::new(Rank::Deuce, Suit::Spades),
            3 => Card::new(Rank::Three, Suit::Spades),
            4 => Card::new(Rank::Four, Suit::Spades),
            5 => Card::new(Rank::Five, Suit::Spades),
            6 => Card::new(Rank::Six, Suit::Spades),
            7 => Card::new(Rank::Seven, Suit::Spades),
            8 => Card::new(Rank::Eight, Suit::Spades),
            9 => Card::new(Rank::Nine, Suit::Spades),
            10 => Card::new(Rank::Ten, Suit::Spades),
            11 => Card::new(Rank::Ace, Suit::Spades),
            _ => Card::new(Rank::Deuce, Suit::Spades),

        }
    }
}

impl Stringable for BlackjackRank{
    fn to_string_internal(&self) -> String {
        match self.val{
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "10",
            11 => "11",
            _ => "invalid",
        }.to_string()
    }
}

impl Allable for BlackjackRank{
    fn create_all() -> Vec<BlackjackRank> {
        vec![BlackjackRank::new(Rank::Deuce),
        BlackjackRank::new(Rank::Three),
        BlackjackRank::new(Rank::Four),
        BlackjackRank::new(Rank::Five),
        BlackjackRank::new(Rank::Six),
        BlackjackRank::new(Rank::Seven),
        BlackjackRank::new(Rank::Eight),
        BlackjackRank::new(Rank::Nine),
        BlackjackRank::new(Rank::Ten),
        BlackjackRank::new(Rank::Ace),]
    }
}

#[cfg(test)]
mod rank_tests {
    use super::*;

    #[test]
    fn test_new_from_int() {
        assert_eq!(Rank::new_from_int(0), Rank::Deuce);
        assert_eq!(Rank::new_from_int(1), Rank::Three);
        assert_eq!(Rank::new_from_int(2), Rank::Four);
        assert_eq!(Rank::new_from_int(3), Rank::Five);
        assert_eq!(Rank::new_from_int(4), Rank::Six);
        assert_eq!(Rank::new_from_int(5), Rank::Seven);
        assert_eq!(Rank::new_from_int(6), Rank::Eight);
        assert_eq!(Rank::new_from_int(7), Rank::Nine);
        assert_eq!(Rank::new_from_int(8), Rank::Ten);
        assert_eq!(Rank::new_from_int(9), Rank::Jack);
        assert_eq!(Rank::new_from_int(10), Rank::Queen);
        assert_eq!(Rank::new_from_int(11), Rank::King);
        assert_eq!(Rank::new_from_int(12), Rank::Ace);
    }

    #[test]
    #[should_panic]
    fn test_new_from_int_panic()
    {
        // Test for an invalid input
        let rank = Rank::new_from_int(13);
    }

    #[test]
    fn test_to_blackjack_score() {
        assert_eq!(Rank::Deuce.to_blackjack_score(), 2);
        assert_eq!(Rank::Three.to_blackjack_score(), 3);
        assert_eq!(Rank::Four.to_blackjack_score(), 4);
        assert_eq!(Rank::Five.to_blackjack_score(), 5);
        assert_eq!(Rank::Six.to_blackjack_score(), 6);
        assert_eq!(Rank::Seven.to_blackjack_score(), 7);
        assert_eq!(Rank::Eight.to_blackjack_score(), 8);
        assert_eq!(Rank::Nine.to_blackjack_score(), 9);
        assert_eq!(Rank::Ten.to_blackjack_score(), 10);
        assert_eq!(Rank::Jack.to_blackjack_score(), 10);
        assert_eq!(Rank::Queen.to_blackjack_score(), 10);
        assert_eq!(Rank::King.to_blackjack_score(), 10);
        assert_eq!(Rank::Ace.to_blackjack_score(), 11);
    }

    #[test]
    fn test_to_string_internal() {
        assert_eq!(Rank::Deuce.to_string_internal(), "2");
        assert_eq!(Rank::Three.to_string_internal(), "3");
        assert_eq!(Rank::Four.to_string_internal(), "4");
        assert_eq!(Rank::Five.to_string_internal(), "5");
        assert_eq!(Rank::Six.to_string_internal(), "6");
        assert_eq!(Rank::Seven.to_string_internal(), "7");
        assert_eq!(Rank::Eight.to_string_internal(), "8");
        assert_eq!(Rank::Nine.to_string_internal(), "9");
        assert_eq!(Rank::Ten.to_string_internal(), "T");
        assert_eq!(Rank::Jack.to_string_internal(), "J");
        assert_eq!(Rank::Queen.to_string_internal(), "Q");
        assert_eq!(Rank::King.to_string_internal(), "K");
        assert_eq!(Rank::Ace.to_string_internal(), "A");
    }
}

#[cfg(test)]
mod suit_tests {
    use super::*;

    #[test]
    fn test_new_from_int() {
        assert_eq!(Suit::new_from_int(0), Suit::Hearts);
        assert_eq!(Suit::new_from_int(1), Suit::Diamonds);
        assert_eq!(Suit::new_from_int(2), Suit::Spades);
        assert_eq!(Suit::new_from_int(3), Suit::Clubs);
    }

    #[test]
    #[should_panic]
    fn test_new_from_int_panics() {
        // Test for an invalid input
        let suit = Suit::new_from_int(4);
    }

    #[test]
    fn test_to_string_internal() {
        assert_eq!(Suit::Hearts.to_string_internal(), "h");
        assert_eq!(Suit::Diamonds.to_string_internal(), "d");
        assert_eq!(Suit::Spades.to_string_internal(), "s");
        assert_eq!(Suit::Clubs.to_string_internal(), "c");
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;

    #[test]
    fn test_new() {
        let rank = Rank::Ace;
        let suit = Suit::Hearts;
        let card = Card::new(rank, suit);

        assert_eq!(card.rank(), rank);
        assert_eq!(card.suit(), suit);
    }

    #[test]
    fn test_new_with_int() {
        let card = Card::new_with_int(0);
        assert_eq!(card.rank(), Rank::Deuce);
        assert_eq!(card.suit(), Suit::Hearts);

        let card = Card::new_with_int(12);
        assert_eq!(card.rank(), Rank::Ace);
        assert_eq!(card.suit(), Suit::Hearts);

        let card = Card::new_with_int(13);
        assert_eq!(card.rank(), Rank::Deuce);
        assert_eq!(card.suit(), Suit::Diamonds);

        let card = Card::new_with_int(51);
        assert_eq!(card.rank(), Rank::Ace);
        assert_eq!(card.suit(), Suit::Clubs);
    }

    #[test]
    fn test_to_blackjack_score() {
        let card = Card::new(Rank::Ace, Suit::Hearts);
        assert_eq!(card.to_blackjack_score(), 1);

        let card = Card::new(Rank::Deuce, Suit::Hearts);
        assert_eq!(card.to_blackjack_score(), 2);

        let card = Card::new(Rank::Three, Suit::Hearts);
        assert_eq!(card.to_blackjack_score(), 3);

        let card = Card::new(Rank::Four, Suit::Hearts);
        assert_eq!(card.to_blackjack_score(), 4);

        let card = Card::new(Rank::Five, Suit::Hearts);
        assert_eq!(card.to_blackjack_score(), 5);

        let card = Card::new(Rank::Six, Suit::Hearts);
        assert_eq!(card.to_blackjack_score(), 6);

        let card = Card::new(Rank::Seven, Suit::Hearts);
        assert_eq!(card.to_blackjack_score(), 7);

        let card = Card::new(Rank::Eight, Suit::Hearts);
        assert_eq!(card.to_blackjack_score(), 8);

        let card = Card::new(Rank::Nine, Suit::Hearts);
        assert_eq!(card.to_blackjack_score(), 9);

        let card = Card::new(Rank::Ten, Suit::Hearts);
        assert_eq!(card.to_blackjack_score(), 10);

        let card = Card::new(Rank::Jack, Suit::Hearts);
        assert_eq!(card.to_blackjack_score(), 10);

        let card = Card::new(Rank::Queen, Suit::Hearts);
        assert_eq!(card.to_blackjack_score(), 10);

        let card = Card::new(Rank::King, Suit::Hearts);
        assert_eq!(card.to_blackjack_score(), 10);
    }
}

#[cfg(test)]
mod blackjack_rank_tests {
    use super::*;

    #[test]
    fn test_new() {
        let ranks: Vec<Rank> = (0..13).map(|i| Rank::new_from_int(i)).collect();
        for rank in ranks {
            let blackjack_rank = BlackjackRank::new(rank);
            assert_eq!(blackjack_rank.val, rank.to_blackjack_score());
        }
    }

    #[test]
    fn test_get_representative_card() {
        let blackjack_ranks = BlackjackRank::create_all();
        for blackjack_rank in blackjack_ranks {
            let card = blackjack_rank.get_representative_card();
            let expected_rank = match blackjack_rank.val {
                2 => Rank::Deuce,
                3 => Rank::Three,
                4 => Rank::Four,
                5 => Rank::Five,
                6 => Rank::Six,
                7 => Rank::Seven,
                8 => Rank::Eight,
                9 => Rank::Nine,
                10 => Rank::Ten,
                11 => Rank::Ace,
                _ => Rank::Deuce,
            };
            assert_eq!(card.rank(), expected_rank);
            assert_eq!(card.suit(), Suit::Spades);
        }
    }

    #[test]
    fn test_to_string_internal() {
        let blackjack_ranks = BlackjackRank::create_all();
        for blackjack_rank in blackjack_ranks {
            let expected_string = match blackjack_rank.val {
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "4",
                5 => "5",
                6 => "6",
                7 => "7",
                8 => "8",
                9 => "9",
                10 => "10",
                11 => "11",
                _ => "invalid",
            };
            assert_eq!(blackjack_rank.to_string_internal(), expected_string);
        }
    }

    #[test]
    fn test_create_all() {
        let all_blackjack_ranks = BlackjackRank::create_all();

        // Assert that the length of the created vector is as expected
        assert_eq!(all_blackjack_ranks.len(), 10);

        let expected_ranks = [
            Rank::Deuce,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Ace,
        ];

        for (index, blackjack_rank) in all_blackjack_ranks.iter().enumerate() {
            assert_eq!(blackjack_rank.val, expected_ranks[index].to_blackjack_score());
        }
    }
}


