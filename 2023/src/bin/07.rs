use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_t: HandType = self.into();
        let other_t: HandType = other.into();
        (self_t, self.cards.clone()).cmp(&(other_t, other.cards.clone()))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct ParseHandErr;

impl FromStr for Hand {
    type Err = ParseHandErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = s.chars().map(Card::from).collect::<Vec<Card>>();
        dbg!(&cards);

        Ok(Hand { cards })
    }
}

// derived PartialOrd uses order of declaration (first declared is least)
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Card {
    Joker,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '1' => Card::One,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Joker,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("non-existent card"),
        }
    }
}

// derived PartialOrd uses order of declaration (first declared is least)
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&Hand> for HandType {
    fn from(value: &Hand) -> Self {
        let most_freq = value
            .cards
            .iter()
            .filter(|c| !matches!(c, Card::Joker))
            .cloned()
            .fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            })
            .iter()
            .max_by(|a, b| {
                // first look to frequency, then to card type
                if a.1 == b.1 {
                    a.0.cmp(b.0)
                } else {
                    a.1.cmp(b.1)
                }
            })
            .unwrap_or((&Card::Ace, &0)) // if no-non-Jokers, Ace is the best card
            .0
            .clone();

        let cards: Vec<Card> = value
            .cards
            .iter()
            .map(|c| {
                if matches!(c, Card::Joker) {
                    most_freq.clone()
                } else {
                    c.to_owned()
                }
            })
            .collect();

        let freqs = cards.iter().cloned().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
        match (freqs.len(), freqs.values().max().unwrap()) {
            (5, _) => HandType::HighCard,     // all unique
            (4, _) => HandType::OnePair,      // one duplicate
            (3, 3) => HandType::ThreeOfAKind, // two duplicates of same type
            (3, 2) => HandType::TwoPair,      // two duplicates of different types
            (2, 4) => HandType::FourOfAKind,  // four duplicates of same type
            (2, 3) => HandType::FullHouse,    // four duplicates of two types
            (1, _) => HandType::FiveOfAKind,  // five of a kind
            _ => panic!("impossible length"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_whitespace().take(2).next_tuple().unwrap();

            (hand.parse::<Hand>().unwrap(), bid.parse::<u32>().unwrap())
        })
        .collect();

    hands.sort_by_key(|t| t.clone().0);

    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|(index, (_hand, bid))| {
            let rank = index as u32 + 1;
            rank * bid
        })
        .sum::<u32>();

    Some(total_winnings)
}

// part two reuses the same implementation as above, but mutates the definition of 'J'
pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_whitespace().take(2).next_tuple().unwrap();

            (hand.parse::<Hand>().unwrap(), bid.parse::<u32>().unwrap())
        })
        .collect();

    hands.sort_by_key(|t| t.clone().0);

    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|(index, (_hand, bid))| {
            dbg!(index, _hand, bid);
            let rank = index as u32 + 1;
            rank * bid
        })
        .sum::<u32>();

    Some(total_winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_misc() {
        assert!(Hand::from_str("JJJJJ").unwrap() > Hand::from_str("AAAA2").unwrap());
        assert!(Hand::from_str("AKQAJ").unwrap() > Hand::from_str("AKQAK").unwrap());
        assert!(Hand::from_str("JKKK2").unwrap() < Hand::from_str("QQQQ2").unwrap());
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
