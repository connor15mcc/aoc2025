use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug)]
struct SafetyManual {
    // given p_a : { p_b, p_c }, p_a printed before p_b | p_c
    rules: HashMap<Page, HashSet<Page>>,
    updates: Vec<Vec<Page>>,
}

impl SafetyManual {
    fn ordered_update(&self, upd: &[Page]) -> bool {
        for (prev, next) in upd.iter().tuple_windows() {
            if !self.rules.get(prev).is_some_and(|hs| hs.contains(next)) {
                return false;
            }
        }

        true
    }

    fn middle_page_number(&self, pages: &[Page]) -> Page {
        pages[pages.len() / 2]
    }

    fn sort_pages(&self, pages: &[Page]) -> Vec<Page> {
        let mut pages = pages.to_owned();
        pages.sort_by(|a, b| match self.rules.get(a).map(|s| s.contains(b)) {
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
            None => Ordering::Equal,
        });

        pages
    }
}

#[derive(Debug)]
struct ParseSafetyManualErr;

impl FromStr for SafetyManual {
    type Err = ParseSafetyManualErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sec_rules, sec_updates) = s
            .split_once("\n\n")
            .expect("section delimited by blank line");

        let mut rules: HashMap<Page, HashSet<Page>> = HashMap::new();
        sec_rules.lines().for_each(|line| {
            let (l, r) = line.split_once('|').expect("delimited by |");
            let (l, r) = (
                l.parse().expect("should be int"),
                r.parse().expect("should be int"),
            );

            rules.entry(l).or_default().insert(r);
        });

        let updates = sec_updates
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse().expect("should be int"))
                    .collect()
            })
            .collect();

        Ok(SafetyManual { rules, updates })
    }
}

type Page = usize;

pub fn part_one(input: &str) -> Option<u32> {
    let sm = SafetyManual::from_str(input).expect("should be safety manual");

    let result = sm
        .updates
        .iter()
        .filter(|u| sm.ordered_update(u))
        .map(|u| sm.middle_page_number(u) as u32)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sm = SafetyManual::from_str(input).expect("should be safety manual");

    let result = sm
        .updates
        .iter()
        .filter(|u| !sm.ordered_update(u))
        .map(|u| sm.sort_pages(u))
        .map(|u| sm.middle_page_number(&u) as u32)
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
