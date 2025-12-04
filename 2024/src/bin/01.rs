use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(1);

type LocationID = u32;

#[derive(Debug)]
struct ListPair {
    left: Vec<LocationID>,
    right: Vec<LocationID>,
    right_appearances: HashMap<LocationID, u32>,
}

impl ListPair {
    fn total_distance(&self) -> u32 {
        let mut l = self.left.clone();
        let mut r = self.right.clone();
        l.sort();
        r.sort();

        l.iter().zip(r.iter()).map(|(l, r)| l.abs_diff(*r)).sum()
    }

    fn similarity_score(&self) -> u32 {
        self.left
            .iter()
            .map(|&v| v * self.right_appearances.get(&v).unwrap_or(&0))
            .sum()
    }
}

#[derive(Debug)]
struct ParseListPairErr;

impl FromStr for ListPair {
    type Err = ParseListPairErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right): (Vec<_>, Vec<_>) = s
            .lines()
            .map(|l| {
                let mut parts = l.split_whitespace();
                let (l, r) = (
                    parts.next().expect("input lines composed of 2 parts"),
                    parts.next().expect("input lines composed of 2 parts"),
                );

                (
                    l.parse::<LocationID>().expect("IDs are LocationIDs"),
                    r.parse::<LocationID>().expect("IDs are LocationIDs"),
                )
            })
            .unzip();

        let right_appearances =
            right
                .iter()
                .fold(HashMap::<LocationID, u32>::new(), |mut m, &k| {
                    *m.entry(k).or_default() += 1;
                    m
                });

        Ok(ListPair {
            left,
            right,
            right_appearances,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(ListPair::from_str(input).ok()?.total_distance())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(ListPair::from_str(input).ok()?.similarity_score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
