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

    fn to_string(&self) -> String {
        match self {
            Rank::Deuce => "Deuce",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
            Rank::Ace => "Ace",
        }.to_string()
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
            Rank::Ace => return 1, // case eleven is handled outside
        }
    }

    fn to_int(&self) -> i32{
        match self {
            Rank::Ace => return 1,
            Rank::Deuce => return 2,
            Rank::Three => return 3,
            Rank::Four => return 4,
            Rank::Five => return 5,
            Rank::Six => return 6,
            Rank::Seven => return 7,
            Rank::Eight => return 8,
            Rank::Nine => return 9,
            Rank::Ten => return 10,
            Rank::Jack => return 11,
            Rank::Queen => return 12,
            Rank::King => return 13,
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

    fn to_string(&self) -> String {
        match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Spades => "Spades",
            Suit::Clubs => "Clubs",
        }.to_string()
    }

    fn to_int(&self) -> i32{
        match self {
            Suit::Hearts => return 0,
            Suit::Diamonds => return 1,
            Suit::Spades => return 2,
            Suit::Clubs => return 3,
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

    fn from_string(str: &str) -> Card {
        // implementation
        return Card::default();
    }

    fn from_nb(nb: i32) -> Card {
        // implementation
        return Card::default();
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    fn suit(&self) -> Suit {
        self.suit
    }

    fn nb(&self) -> i32 {
        // implementation
        return 0;
    }

    fn to_string(&self) -> String {
        // implementation
        return String::default();
    }

    pub fn to_blackjack_score(&self) -> i32 {
        // implementation
        return  0;
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

fn to_cards(str: &str) -> Vec<Card> {
    // implementation
    Vec::<Card>::default()
}

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
        // implementation
        String::default()
    }

    pub fn get_representative_card(&self) -> Card {
        // implementation
        Card::default()
    }
}

impl Allable for BlackjackRank{
    fn create_all() -> Vec<BlackjackRank> {
        // implementation
        Vec::<BlackjackRank>::default()
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

