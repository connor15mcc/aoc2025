use itertools::Itertools;
use std::{cmp::Ordering, str::FromStr};

advent_of_code::solution!(2);

#[derive(Debug)]
struct Report {
    levels: Vec<Level>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut ord = Ordering::Equal;
        for (prev, next) in self.levels.iter().tuple_windows() {
            let next_ord = prev.cmp(next);
            // changes in direction are unsafe
            if (ord == Ordering::Greater && next_ord == Ordering::Less)
                || (ord == Ordering::Less && next_ord == Ordering::Greater)
            {
                return false;
            }
            ord = next_ord;

            if !(1..=3).contains(&next.abs_diff(*prev)) {
                return false;
            }
        }

        true
    }

    fn dampened_is_safe(&self) -> bool {
        let mut ord = Ordering::Equal;
        for ((i, prev), (j, next)) in self.levels.iter().enumerate().tuple_windows() {
            let next_ord = prev.cmp(next);
            // changes in direction are unsafe
            if (ord == Ordering::Greater && next_ord == Ordering::Less)
                || (ord == Ordering::Less && next_ord == Ordering::Greater)
            {
                return self.possible_parts_safety(i, j);
            }
            ord = next_ord;

            if !(1..=3).contains(&next.abs_diff(*prev)) {
                return self.possible_parts_safety(i, j);
            }
        }

        true
    }

    // to handle dampening, consider the Report with that level removed
    fn possible_parts_safety(&self, i: usize, j: usize) -> bool {
        [i, j].iter().any(|&pivot| {
            let mut levels = self.levels.clone();
            levels.remove(pivot);
            Report { levels }.is_safe()
        })
    }
}

#[derive(Debug)]
struct ParseReportErr;

impl FromStr for Report {
    type Err = ParseReportErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s.split_whitespace().map(|i| i.parse().unwrap()).collect();
        Ok(Report { levels })
    }
}

type Level = i32;

pub fn part_one(input: &str) -> Option<usize> {
    let num_safe_reports = input
        .lines()
        .map(|l| Report::from_str(l).unwrap())
        .filter(|r| r.is_safe())
        .count();

    Some(num_safe_reports)
}

pub fn part_two(input: &str) -> Option<usize> {
    let num_safe_reports = input
        .lines()
        .map(|l| Report::from_str(l).unwrap())
        .filter(|r| r.dampened_is_safe())
        .count();

    Some(num_safe_reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
