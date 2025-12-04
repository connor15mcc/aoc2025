use std::{cmp, collections::HashSet, str::FromStr};

advent_of_code::solution!(4);

#[derive(parse_display::FromStr, Debug, Clone)]
#[display("Card {id}: {winning} | {yours}")]
struct Scratchcard {
    #[from_str(regex = r"\s*(?<>\w+)")]
    id: u32,
    yours: Numbers,
    winning: Numbers,
}

impl Scratchcard {
    fn matches(&self) -> std::collections::hash_set::Intersection<'_, i32, std::hash::RandomState> {
        self.yours.0.intersection(&self.winning.0)
    }

    fn total_copies(&self, deck: &Vec<Scratchcard>) -> u32 {
        1 + self
            .matches()
            .enumerate()
            .map(|(match_idx, _match)| {
                deck.get(self.id as usize + match_idx)
                    .expect("problem states it won't go beyond deck")
                    .total_copies(deck)
            })
            .sum::<u32>()
    }
}

#[derive(Debug, Clone)]
struct Numbers(HashSet<i32>);

struct ParseNumbersErr;

impl FromStr for Numbers {
    type Err = ParseNumbersErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .split_whitespace()
            .map(|i| i.parse::<i32>().or(Err(ParseNumbersErr)))
            .collect::<Result<HashSet<i32>, ParseNumbersErr>>()?;

        Ok(Numbers(inner))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|l| {
            Scratchcard::from_str(l)
                .expect("parser should succeed")
                .matches()
                .count()
        })
        .map(|n| {
            if n == 0 {
                return 0;
            }
            2u32.pow(n as u32 - 1)
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = &input
        .lines()
        .map(|l| Scratchcard::from_str(l).unwrap())
        .collect::<Vec<Scratchcard>>();

    let result = cards.iter().map(|card| card.total_copies(cards)).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
