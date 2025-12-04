advent_of_code::solution!(9);

fn gen_differences(hist: Vec<i32>) -> Vec<Vec<i32>> {
    let mut differences = Vec::new();
    differences.push(hist);
    let mut depth = 0;

    loop {
        let mut curr_differences = Vec::new();
        for i in 0..differences[depth].len() - 1 {
            let diff = differences[depth][i + 1] - differences[depth][i];
            curr_differences.push(diff);
        }

        differences.push(curr_differences);
        depth += 1;

        if differences[depth].iter().all(|&i| i == 0) {
            break;
        }
    }
    differences
}

fn extrapolate_next(differences: Vec<Vec<i32>>) -> i32 {
    let mut differences = differences.to_owned();

    for i in (0..differences.len()).rev() {
        if i == differences.len() - 1 {
            differences[i].push(0);
        } else {
            let prev = differences[i].last().unwrap().to_owned();
            let diff = differences[i + 1].last().unwrap().to_owned();
            differences[i].push(prev + diff);
        }
    }
    differences[0].last().unwrap().to_owned()
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut result = 0;
    for line in input.lines() {
        let values = line
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let diff = gen_differences(values);

        result += extrapolate_next(diff);
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut result = 0;
    for line in input.lines() {
        let values_rev = line
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .rev()
            .collect::<Vec<i32>>();

        let diff = gen_differences(values_rev);

        result += extrapolate_next(diff);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
