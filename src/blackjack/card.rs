use crate::blackjack::traits::Allable;

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
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

#[derive(Eq, Ord, PartialEq, PartialOrd)]
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

impl Default for Card {
    fn default() -> Card {
        Card {
            rank: Rank::Deuce,
            suit: Suit::Hearts,
        }
    }
}

impl Clone for Card {
    fn clone(&self) -> Card {
        Card {
            rank: self.rank,
            suit: self.suit,
        }
    }
}

impl Copy for Card {}

pub struct BlackjackRank {
    val: i32,
}

impl BlackjackRank {
    pub fn new(rank: Rank) -> BlackjackRank {
        BlackjackRank {
            val: rank.to_blackjack_score(),
        }
    }

    pub fn to_string(&self) -> String {
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

impl Default for BlackjackRank {
    fn default() -> BlackjackRank {
        BlackjackRank { val: -1 }
    }
}

impl Clone for BlackjackRank {
    fn clone(&self) -> BlackjackRank {
        BlackjackRank { val: self.val }
    }
}

impl Copy for BlackjackRank {}

impl PartialEq for BlackjackRank {
    fn eq(&self, other: &BlackjackRank) -> bool {
        self.val == other.val
    }
}

impl Eq for BlackjackRank {}

impl PartialOrd for BlackjackRank {
    fn partial_cmp(&self, other: &BlackjackRank) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BlackjackRank {
    fn cmp(&self, other: &BlackjackRank) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

