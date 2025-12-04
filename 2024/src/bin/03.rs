use regex::Regex;
use std::str::FromStr;

advent_of_code::solution!(3);

const MUL_RE: &str = r"mul\((\d+),(\d+)\)";
const DO_RE: &str = r"do\(\)";
const DONT_RE: &str = r"don't\(\)";

#[derive(Debug)]
struct Memory(Vec<MulOp>);

#[derive(Debug)]
struct ParseMemoryErr;

impl FromStr for Memory {
    type Err = ParseMemoryErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(&format!("{MUL_RE}|{DO_RE}|{DONT_RE}")).unwrap();
        let mut enabled = true;

        let muls = re
            .captures_iter(s)
            .filter_map(|c| {
                if let (Some(l), Some(r)) = (c.get(1), c.get(2)) {
                    let (left, right) = (
                        l.as_str().parse().expect("regex only matches ints"),
                        r.as_str().parse().expect("regex only matches ints"),
                    );
                    Some(MulOp {
                        enabled,
                        left,
                        right,
                    })
                } else {
                    match &c[0] {
                        "do()" => enabled = true,
                        "don't()" => enabled = false,
                        _ => panic!("unexpected match"),
                    }
                    None
                }
            })
            .collect();

        Ok(Memory(muls))
    }
}

#[derive(Debug)]
struct MulOp {
    enabled: bool,
    left: i32,
    right: i32,
}

pub fn part_one(input: &str) -> Option<i32> {
    let result = Memory::from_str(input)
        .unwrap()
        .0
        .iter()
        .map(|mul| mul.left * mul.right)
        .sum::<i32>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let result = Memory::from_str(input)
        .unwrap()
        .0
        .iter()
        .filter_map(|mul| {
            if mul.enabled {
                Some(mul.left * mul.right)
            } else {
                None
            }
        })
        .sum::<i32>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
