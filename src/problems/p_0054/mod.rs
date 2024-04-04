//! **Problem 54** - *Poker Hands*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(54, "Poker Hands", solve)
}

use once_cell::sync::Lazy;
use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;

fn solve() -> String {
    let input = include_str!("0054_poker.txt");
    let parsed_input = parse_input(input);

    parsed_input.into_iter().filter(|&game| game_result(game)).count().to_string()
}

#[derive(Copy, Clone, Debug)]
enum Hand {
    HighCard([u8; 5]),
    OnePair((u8, [u8; 3])),
    TwoPairs((u8, u8, u8)),
    ThreeOfAKind((u8, [u8; 2])),
    Straight([u8; 5]),
    Flush([u8; 5]),
    FullHouse((u8, u8)),
    FourOfAKind((u8, u8)),
    StraightFlush([u8; 5]),
    RoyalFlush,
}

fn hand_score(hand: Hand) -> u8 {
    match hand {
        Hand::HighCard(_) => 0,
        Hand::OnePair(_) => 1,
        Hand::TwoPairs(_) => 2,
        Hand::ThreeOfAKind(_) => 3,
        Hand::Straight(_) => 4,
        Hand::Flush(_) => 5,
        Hand::FullHouse(_) => 6,
        Hand::FourOfAKind(_) => 7,
        Hand::StraightFlush(_) => 8,
        Hand::RoyalFlush => 9,
    }
}

fn game_result(game: Game) -> bool {
    //! Returns true if player 1 wins, false if player 2 wins.

    let (player1, player2) = game;
    let player1 = hand(player1);
    let player1_score = hand_score(player1);
    let player2 = hand(player2);
    let player2_score = hand_score(player2);

    match player1_score.cmp(&player2_score) {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => match player1 {
            Hand::HighCard(play1_val) => {
                if let Hand::HighCard(play2_val) = player2 {
                    play1_val[0] > play2_val[0]
                } else {
                    unreachable!("Player 2 must have a high card here.")
                }
            }
            Hand::OnePair(play1_val) => {
                if let Hand::OnePair(play2_val) = player2 {
                    match play1_val.0.cmp(&play2_val.0) {
                        Ordering::Greater => true,
                        Ordering::Less => false,
                        Ordering::Equal => play1_val.1 > play2_val.1,
                    }
                } else {
                    unreachable!("Player 2 must have a pair here.")
                }
            }
            Hand::TwoPairs(play1_val) => {
                if let Hand::TwoPairs(play2_val) = player2 {
                    match play1_val.0.cmp(&play2_val.0) {
                        Ordering::Greater => true,
                        Ordering::Less => false,
                        Ordering::Equal => match play1_val.1.cmp(&play2_val.1) {
                            Ordering::Greater => true,
                            Ordering::Less => false,
                            Ordering::Equal => play1_val.2 > play2_val.2,
                        },
                    }
                } else {
                    unreachable!("Player 2 must have two pairs here.")
                }
            }
            Hand::ThreeOfAKind(play1_val) => {
                if let Hand::ThreeOfAKind(play2_val) = player2 {
                    match play1_val.0.cmp(&play2_val.0) {
                        Ordering::Greater => true,
                        Ordering::Less => false,
                        Ordering::Equal => play1_val.1 > play2_val.1,
                    }
                } else {
                    unreachable!("Player 2 must have a three of a kind here.")
                }
            }
            Hand::Straight(play1_val) => {
                if let Hand::Straight(play2_val) = player2 {
                    play1_val[0] > play2_val[0]
                } else {
                    unreachable!("Player 2 must have a straight here.")
                }
            }
            Hand::Flush(play1_val) => {
                if let Hand::Flush(play2_val) = player2 {
                    play1_val[0] > play2_val[0]
                } else {
                    unreachable!("Player 2 must have a flush here.")
                }
            }
            Hand::FullHouse(play1_val) => {
                if let Hand::FullHouse(play2_val) = player2 {
                    match play1_val.0.cmp(&play2_val.0) {
                        Ordering::Greater => true,
                        Ordering::Less => false,
                        Ordering::Equal => play1_val.1 > play2_val.1,
                    }
                } else {
                    unreachable!("Player 2 must have a full house here.")
                }
            }
            Hand::FourOfAKind(play1_val) => {
                if let Hand::FourOfAKind(play2_val) = player2 {
                    match play1_val.0.cmp(&play2_val.0) {
                        Ordering::Greater => true,
                        Ordering::Less => false,
                        Ordering::Equal => play1_val.1 > play2_val.1,
                    }
                } else {
                    unreachable!("Player 2 must have a four of a kind here.")
                }
            }
            Hand::StraightFlush(play1_val) => {
                if let Hand::StraightFlush(play2_val) = player2 {
                    play1_val[0] > play2_val[0]
                } else {
                    unreachable!("Player 2 must have a straight flush here.")
                }
            }
            Hand::RoyalFlush => panic!("There should not be ties in the input."),
        },
    }
}

fn hand(player: [(u8, u8); 5]) -> Hand {
    // high card
    let mut sorted_cards = player.iter().map(|card| card.0).collect::<Vec<u8>>();
    sorted_cards.sort_by_key(|&rank| Reverse(rank));
    let mut result: Hand = Hand::HighCard(sorted_cards.try_into().unwrap());

    // three of a kind, four of a kind, pairs
    let mut kind_counts: HashMap<u8, u8> = HashMap::new();
    for card in player {
        let count = kind_counts.entry(card.0).or_insert(0);
        *count += 1;
    }
    let mut pairs: Vec<u8> = Vec::new();
    for (rank, count) in kind_counts {
        match count {
            2 => {
                pairs.push(rank);
            }
            3 => {
                let mut other_cards: Vec<u8> = Vec::with_capacity(2);
                for card in player {
                    if card.0 != rank {
                        other_cards.push(card.0);
                    }
                }

                other_cards.sort_by_key(|&rank| Reverse(rank));

                result = Hand::ThreeOfAKind((rank, other_cards.try_into().unwrap()));
            }
            4 => {
                let mut other_card: u8 = 0;
                for card in player {
                    if card.0 != rank {
                        other_card = card.0;
                    }
                }

                result = Hand::FourOfAKind((rank, other_card));
            }
            _ => {}
        }
    }

    if let Hand::HighCard(_) = result {
        match pairs.len() {
            1 => {
                let mut other_cards = player.into_iter().map(|card| card.0).filter(|&rank| rank != pairs[0]).collect::<Vec<u8>>();
                other_cards.sort_by_key(|&rank| Reverse(rank));
                result = Hand::OnePair((pairs[0], other_cards.try_into().unwrap()));
            }
            2 => {
                let mut left_card: Option<u8> = None;
                for card in player {
                    if card.0 != pairs[0] && card.0 != pairs[1] {
                        left_card = Some(card.0);
                        break;
                    }
                }
                assert!(left_card.is_some());

                result = Hand::TwoPairs((pairs[0], pairs[1], left_card.unwrap()));
            }
            _ => {}
        }
    } else if let Hand::ThreeOfAKind(value) = result {
        for pair in &pairs {
            if *pair != value.0 {
                result = Hand::FullHouse((value.0, *pair));
                break;
            }
        }
    }

    if let Hand::HighCard(_) = result {
        // straight
        let mut straight_cards = player.iter().map(|card| card.0).collect::<Vec<u8>>();
        straight_cards.sort();
        let straight = (straight_cards[0]..(straight_cards[0] + 5)).collect::<Vec<u8>>() == straight_cards;
        if straight {
            let mut hand = player.into_iter().map(|card| card.0).collect::<Vec<u8>>();
            hand.sort_by_key(|&rank| Reverse(rank));
            result = Hand::Straight(hand.try_into().unwrap());
        }

        // flush
        let mut flush = true;
        for card in player {
            if card.1 != player[0].1 {
                flush = false;
                break;
            }
        }
        if flush {
            let mut hand = player.into_iter().map(|card| card.0).collect::<Vec<u8>>();
            hand.sort_by_key(|&rank| Reverse(rank));
            result = Hand::Flush(hand.try_into().unwrap());
        }

        // straight flush, royal flush
        let straight_flush = flush && straight;
        if straight_flush {
            let mut hand = player.into_iter().map(|card| card.0).collect::<Vec<u8>>();
            hand.sort_by_key(|&rank| Reverse(rank));
            result = if hand[0] != 12 {
                Hand::StraightFlush(hand.try_into().unwrap())
            } else {
                Hand::RoyalFlush
            };
        }
    }

    result
}

// (rank, suit)
type Game = ([(u8, u8); 5], [(u8, u8); 5]);

fn parse_input(input: &str) -> Vec<Game> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut line_elements = line.split_whitespace();

            let player1: [(u8, u8); 5] = line_elements.by_ref().take(5).map(parse_card).collect::<Vec<(u8, u8)>>().try_into().unwrap();
            let player2: [(u8, u8); 5] = line_elements.map(parse_card).collect::<Vec<(u8, u8)>>().try_into().unwrap();

            (player1, player2)
        })
        .collect()
}

fn parse_card(card: &str) -> (u8, u8) {
    let mut chars = card.chars();

    let rank = *RANKS.get(&chars.next().unwrap()).unwrap();
    let suit = *SUITS.get(&chars.next().unwrap()).unwrap();

    (rank, suit)
}

static SUITS: Lazy<HashMap<char, u8>> = Lazy::new(|| {
    let mut new_hashmap = HashMap::new();

    new_hashmap.insert('C', 0);
    new_hashmap.insert('D', 1);
    new_hashmap.insert('H', 2);
    new_hashmap.insert('S', 3);

    new_hashmap
});

static RANKS: Lazy<HashMap<char, u8>> = Lazy::new(|| {
    let mut new_hashmap = HashMap::new();

    new_hashmap.insert('2', 0);
    new_hashmap.insert('3', 1);
    new_hashmap.insert('4', 2);
    new_hashmap.insert('5', 3);
    new_hashmap.insert('6', 4);
    new_hashmap.insert('7', 5);
    new_hashmap.insert('8', 6);
    new_hashmap.insert('9', 7);
    new_hashmap.insert('T', 8);
    new_hashmap.insert('J', 9);
    new_hashmap.insert('Q', 10);
    new_hashmap.insert('K', 11);
    new_hashmap.insert('A', 12);

    new_hashmap
});
