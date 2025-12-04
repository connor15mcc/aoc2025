use std::cmp::max;
use std::ops::Add;
use std::str::FromStr;

advent_of_code::solution!(2);

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn impossible(&self, r: u32, g: u32, b: u32) -> bool {
        self.sets
            .iter()
            .any(|s| s.red > r || s.green > g || s.blue > b)
    }

    fn power(&self) -> u32 {
        let (mut max_r, mut max_g, mut max_b) = (0, 0, 0);
        for set in &self.sets {
            max_r = max(max_r, set.red);
            max_g = max(max_g, set.green);
            max_b = max(max_b, set.blue);
        }

        max_r * max_g * max_b
    }
}

#[derive(Debug)]
struct ParseGameErr;

impl FromStr for Game {
    type Err = ParseGameErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, sets) = s
            .strip_prefix("Game ")
            .and_then(|s| s.split_once(": "))
            .ok_or(ParseGameErr)?;

        let id_fromstr = id.parse().map_err(|_| ParseGameErr)?;
        let sets_fromstr = sets
            .split("; ")
            .map(|s| s.parse::<Set>().or(Err(ParseGameErr)))
            .collect::<Result<Vec<Set>, ParseGameErr>>()?;

        Ok(Game {
            id: id_fromstr,
            sets: sets_fromstr,
        })
    }
}

#[derive(Default, Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

struct ParseSetErr;

impl FromStr for Set {
    type Err = ParseSetErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Set::default();

        for s in s.split(", ") {
            let (num, color) = s.split_once(' ').ok_or(ParseSetErr)?;
            let num_fromstr = num.parse().or(Err(ParseSetErr))?;

            match color {
                "red" => set.red = num_fromstr,
                "green" => set.green = num_fromstr,
                "blue" => set.blue = num_fromstr,
                _ => (),
            };
        }

        Ok(set)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|l| Game::from_str(l).unwrap())
        .filter_map(|g| {
            if g.impossible(12, 13, 14) {
                return None;
            }
            Some(g.id)
        })
        .reduce(u32::add);

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|l| Game::from_str(l).unwrap())
        .map(|g| g.power())
        .reduce(u32::add);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
