use std::{
    collections::{HashMap, HashSet},
    iter,
};

use itertools::iproduct;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let (w, h, grid) = {
        let mut grid = Vec::new();
        let mut it = input.lines().map(|l| l.chars());

        let first = it.next().unwrap();
        grid.extend(first);
        let w = grid.len();

        for item in it {
            grid.extend(item);
        }
        (w, grid.len() / w, grid)
    };

    let mut parts = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        let mut it = line.chars().enumerate();
        while let Some((x, char)) = it.next() {
            if !char.is_ascii_digit() {
                continue;
            }

            let mut digits = String::new();
            let mut is_part = false;
            let digit_chars = iter::once((x, char))
                .chain(it.by_ref().take_while(|(_x_pos, c)| c.is_ascii_digit()));
            for (x, char) in digit_chars {
                digits.push(char);
                for (x_off, y_off) in iproduct!([-1i32, 0, 1], [-1i32, 0, 1]) {
                    let (x, y) = (x as i32 + x_off, y as i32 + y_off);
                    // bounds check
                    if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
                        continue;
                    };

                    let neighbor = grid[y as usize * w + x as usize];
                    if !neighbor.is_ascii_digit() && neighbor != '.' {
                        is_part = true;
                    }
                }
            }

            if is_part {
                parts.insert((x, y), digits.parse().unwrap());
            }
        }
    }

    Some(parts.values().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (w, h, grid) = {
        let mut grid = Vec::new();
        let mut it = input.lines().map(|l| l.chars());

        let first = it.next().unwrap();
        grid.extend(first);
        let w = grid.len();

        for item in it {
            grid.extend(item);
        }
        (w, grid.len() / w, grid)
    };

    let mut all_parts = HashMap::new();
    let mut gears = HashMap::<_, HashSet<_>>::new();

    for (line_y, line) in input.lines().enumerate() {
        let mut it = line.chars().enumerate();
        while let Some((first_x, char)) = it.next() {
            if !char.is_ascii_digit() {
                continue;
            }

            let mut digits = String::new();
            let mut is_part = false;
            let digit_chars = iter::once((first_x, char))
                .chain(it.by_ref().take_while(|(_x_pos, c)| c.is_ascii_digit()));
            for (x, char) in digit_chars {
                digits.push(char);
                for (x_off, y_off) in iproduct!([-1i32, 0, 1], [-1i32, 0, 1]) {
                    let (x, y) = (x as i32 + x_off, line_y as i32 + y_off);
                    // bounds check
                    if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
                        continue;
                    };

                    let neighbor = grid[y as usize * w + x as usize];
                    if !neighbor.is_ascii_digit() && neighbor != '.' {
                        is_part = true;
                    }

                    if neighbor == '*' {
                        gears.entry((x, y)).or_default().insert((first_x, line_y));
                    }
                }
            }

            if is_part {
                all_parts.insert((first_x, line_y), digits.parse().unwrap());
            }
        }
    }

    Some(
        gears
            .iter()
            .filter(|(_k, parts)| parts.len() == 2)
            .map(|(_gear_pos, parts)| {
                parts
                    .iter()
                    .map(|(part_x, part_y)| {
                        all_parts
                            .get(&(part_x.to_owned(), part_y.to_owned()))
                            .unwrap()
                    })
                    .product::<u32>()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
