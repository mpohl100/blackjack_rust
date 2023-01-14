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
}

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl Suit {
    fn to_string(&self) -> String {
        match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Spades => "Spades",
            Suit::Clubs => "Clubs",
        }.to_string()
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    fn from_string(str: &str) -> Card {
        // implementation
        return Card::default();
    }

    fn from_nb(nb: i32) -> Card {
        // implementation
        return Card::default();
    }

    fn rank(&self) -> Rank {
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

    fn to_blackjack_score(&self) -> i32 {
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

    fn to_string(&self) -> String {
        // implementation
        String::default()
    }

    fn get_representative_card(&self) -> Card {
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

