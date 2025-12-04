advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| -> u32 {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));

            let fst = digits.next().unwrap();
            let snd = digits.next_back().unwrap_or(fst);

            fst * 10 + snd
        })
        .reduce(|acc, n| acc + n)
}

pub fn part_two(input: &str) -> Option<u32> {
    [
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]
    .iter()
    .fold(input.to_string(), |acc, (s, d)| {
        acc.replace(s, &format!("{s}{d}{s}"))
    })
    .lines()
    .map(|line| -> u32 {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));

        let fst = digits.next().unwrap();
        let snd = digits.next_back().unwrap_or(fst);

        fst * 10 + snd
    })
    .reduce(|acc, n| acc + n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
