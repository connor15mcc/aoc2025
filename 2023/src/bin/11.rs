use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

advent_of_code::solution!(11);

type Point = (usize, usize);

struct Cosmos {
    galaxies: Vec<Point>,
    empty_cols: HashSet<usize>,
    empty_rows: HashSet<usize>,
}

impl Cosmos {
    fn manhattan_distance_with_gravity(&self, age: usize) -> usize {
        let mut result = 0;
        for ((x_a, y_a), (x_b, y_b)) in self.galaxies.iter().tuple_combinations() {
            // this is easier to avoid underflow (rather than tmp casting to i32)
            let (col_min, col_max) = (x_a.min(x_b), x_a.max(x_b));
            let (row_min, row_max) = (y_a.min(y_b), y_a.max(y_b));

            let (col_dist, row_dist) = (col_max - col_min, row_max - row_min);

            let col_gravity = self
                .empty_cols
                .iter()
                .filter(|&col| col > col_min && col < col_max)
                .count();
            let row_gravity = self
                .empty_rows
                .iter()
                .filter(|&row| row > row_min && row < row_max)
                .count();

            result += (col_dist + col_gravity * (age - 1)) + (row_dist + row_gravity * (age - 1))
        }
        result
    }
}

#[derive(Debug)]
struct CosmosParseErr;

impl FromStr for Cosmos {
    type Err = CosmosParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies = Vec::new();
        let mut empty_cols = HashSet::new();
        let mut empty_rows = HashSet::new();
        let height = s.lines().count();
        let width = s.lines().next().unwrap().chars().count();

        s.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, char)| {
                if char == '#' {
                    galaxies.push((col, row));
                }
            })
        });

        (0..width).for_each(|col| {
            if !galaxies.iter().any(|&(galaxy_col, _)| galaxy_col == col) {
                empty_cols.insert(col);
            }
        });
        (0..height).for_each(|row| {
            if !galaxies.iter().any(|&(_, galaxy_row)| galaxy_row == row) {
                empty_rows.insert(row);
            }
        });

        Ok(Cosmos {
            galaxies,
            empty_cols,
            empty_rows,
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        Cosmos::from_str(input)
            .unwrap()
            .manhattan_distance_with_gravity(2),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        Cosmos::from_str(input)
            .unwrap()
            .manhattan_distance_with_gravity(1000000),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::template::read_file("examples", DAY);

        assert_eq!(
            Cosmos::from_str(input)
                .unwrap()
                .manhattan_distance_with_gravity(10),
            1030
        );
        assert_eq!(
            Cosmos::from_str(input)
                .unwrap()
                .manhattan_distance_with_gravity(100),
            8410
        );
    }
}
