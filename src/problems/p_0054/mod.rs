//! **Problem 54** - *Poker Hands*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(54, "Poker Hands", solve)
}

use std::cmp::Ordering;
use std::collections::HashMap;

fn solve() -> String {
    let input = include_str!("0054_poker.txt");
    input.lines().map(|line| Game::new(line).winner()).filter(|&winner| winner == 1).count().to_string()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}
impl Suit {
    fn new(suit: char) -> Self {
        match suit {
            'S' => Self::Spades,
            'C' => Self::Clubs,
            'H' => Self::Hearts,
            'D' => Self::Diamonds,
            _ => panic!("Invalid suit."),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Rank {
    Two,
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
    fn new(rank: char) -> Self {
        match rank {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Invalid rank."),
        }
    }
    fn value(&self) -> u8 {
        match self {
            Self::Two => 0,
            Self::Three => 1,
            Self::Four => 2,
            Self::Five => 3,
            Self::Six => 4,
            Self::Seven => 5,
            Self::Eight => 6,
            Self::Nine => 7,
            Self::Ten => 8,
            Self::Jack => 9,
            Self::Queen => 10,
            Self::King => 11,
            Self::Ace => 12,
        }
    }
}
impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Card {
    suit: Suit,
    rank: Rank,
}
impl Card {
    fn new(card: &str) -> Self {
        let mut chars = card.chars();

        let rank = Rank::new(chars.next().unwrap());
        let suit = Suit::new(chars.next().unwrap());

        Self { suit, rank }
    }
    fn value(&self) -> u8 {
        self.rank.value()
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}
impl Hand {
    fn new(hand: &str) -> Self {
        let cards = hand.split_whitespace().map(Card::new).collect::<Vec<Card>>().try_into().unwrap();
        Self { cards }
    }
    fn score(&self) -> u32 {
        let mut score = 0;

        let mut cards = self.cards;

        // high card
        cards.sort();
        score = score.max(self.generate_hand_code(HandType::HighCard, cards));

        // straight
        let mut straight = true;
        for (i, card) in cards.iter().enumerate().skip(1) {
            if card.value() != cards[i - 1].value() + 1 {
                straight = false;
                break;
            }
        }
        if straight {
            score = score.max(self.generate_hand_code(HandType::Straight, cards));
        }

        // flush
        let mut flush = false;
        if cards.iter().all(|card| card.suit == cards[0].suit) {
            score = score.max(self.generate_hand_code(HandType::Flush, cards));
            flush = true;
        }

        // straight flush, royal flush (just a subset of straight flush, not checked directly)
        if straight && flush {
            score = score.max(self.generate_hand_code(HandType::StraightFlush, cards));
        }

        // count the number of cards of each rank
        let mut rank_counts = HashMap::new();
        for card in &cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
        }

        // four of a kind
        if let Some((&rank, _)) = rank_counts.iter().find(|&(_, value)| *value == 4) {
            // put these 4 cards to the back of the hand
            let other_index = cards.iter().position(|card| card.rank != rank).unwrap();
            cards.swap(0, other_index);
            score = score.max(self.generate_hand_code(HandType::FourOfAKind, cards));
        }

        // three of a kind, full house
        if let Some((&rank3, _)) = rank_counts.iter().find(|&(_, value)| *value == 3) {
            // put these 3 cards to the back of the hand
            let mut other_indices = cards.iter().enumerate().filter(|(_, card)| card.rank != rank3).map(|(i, _)| i);
            let other_index_1 = other_indices.next().unwrap();
            let other_index_2 = other_indices.next().unwrap();
            cards.swap(0, other_index_1);
            cards.swap(1, other_index_2);

            score = score.max(self.generate_hand_code(HandType::ThreeOfAKind, cards));

            // full house
            if rank_counts.iter().any(|(_, value)| *value == 2) {
                // these 2 cards are already at the front of the hand (because other 3 were put to the back)
                score = score.max(self.generate_hand_code(HandType::FullHouse, cards));
            }
        }

        // one pair, two pairs
        if let Some((&rank2, _)) = rank_counts.iter().find(|&(_, value)| *value == 2) {
            // put these 2 cards to the back of the hand
            let mut other_indices = cards.iter().enumerate().filter(|(_, card)| card.rank != rank2).map(|(i, _)| i);
            let other_index_1 = other_indices.next().unwrap();
            let other_index_2 = other_indices.next().unwrap();
            let other_index_3 = other_indices.next().unwrap();
            cards.swap(0, other_index_1);
            cards.swap(1, other_index_2);
            cards.swap(2, other_index_3);

            score = score.max(self.generate_hand_code(HandType::OnePair, cards));

            // two pairs
            if let Some((&rank2_2, _)) = rank_counts.iter().filter(|&(_, value)| *value == 2).nth(1) {
                // put the only card left to the front of the hand
                let other_index = cards.iter().position(|card| card.rank != rank2 && card.rank != rank2_2).unwrap();
                cards.swap(0, other_index);
                if cards[1] > cards[3] {
                    cards.swap(1, 3);
                    cards.swap(2, 4);
                }
                score = score.max(self.generate_hand_code(HandType::TwoPairs, cards));
            }
        }

        score
    }
    fn generate_hand_code(&self, hand_type: HandType, cards: [Card; 5]) -> u32 {
        let mut code = 0;
        let mut factor = 1;

        // cards are sorted in the ascending order of their weights
        for card in cards {
            code += factor * card.value() as u32;
            factor *= 13;
        }

        // add the biggest value representing the hand type
        code += factor * hand_type.value() as u32;

        // return the generated code
        code
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score().cmp(&other.score())
    }
}

struct Game {
    player1: Hand,
    player2: Hand,
}
impl Game {
    fn new(game: &str) -> Self {
        let (player1, player2) = game.trim().split_at(game.len() / 2);
        let player1 = Hand::new(player1.trim());
        let player2 = Hand::new(player2.trim());
        Self { player1, player2 }
    }
    fn winner(&mut self) -> u8 {
        match self.player1.cmp(&self.player2) {
            Ordering::Greater => 1,
            Ordering::Less => 2,
            Ordering::Equal => 0,
        }
    }
}

enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}
impl HandType {
    fn value(&self) -> u8 {
        match self {
            Self::HighCard => 0,
            Self::OnePair => 1,
            Self::TwoPairs => 2,
            Self::ThreeOfAKind => 3,
            Self::Straight => 4,
            Self::Flush => 5,
            Self::FullHouse => 6,
            Self::FourOfAKind => 7,
            Self::StraightFlush => 8,
        }
    }
}
