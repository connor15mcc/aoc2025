use std::str::FromStr;

advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    duration_ms: u64,
    distance_mm: u64,
}

impl Race {
    fn winning_strategies(&self) -> Vec<u64> {
        (0..=self.duration_ms)
            .filter(|t| t * (self.duration_ms - t) > self.distance_mm)
            .collect()
    }
}

fn input_to_races(input: &str) -> Vec<Race> {
    let (time, distance) = input.split_once("\n").unwrap();

    // skip label col
    let times = time.split_whitespace().skip(1);
    let distances = distance.split_whitespace().skip(1);

    times
        .zip(distances)
        .map(|(t, d)| Race {
            duration_ms: t.parse().unwrap(),
            distance_mm: d.parse().unwrap(),
        })
        .collect()
}

fn input_to_race(input: &str) -> Race {
    let (time, distance) = input.split_once("\n").unwrap();

    // skip label col
    let time = time.split_whitespace().skip(1).collect::<String>();
    let distance = distance.split_whitespace().skip(1).collect::<String>();

    Race {
        duration_ms: time.parse().unwrap(),
        distance_mm: distance.parse().unwrap(),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = input_to_races(input);

    let product = races
        .iter()
        .map(|r| r.winning_strategies().len() as u64)
        .product::<u64>();

    Some(product)
}

pub fn part_two(input: &str) -> Option<u64> {
    let race = input_to_race(input);
    Some(race.winning_strategies().len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
