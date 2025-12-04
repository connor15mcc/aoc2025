use lazy_static::lazy_static;
use std::{
    ops::{Mul, Neg},
    str::FromStr,
};

use itertools::iproduct;

advent_of_code::solution!(4);

#[derive(Debug)]
struct WordSearch {
    w: usize,
    h: usize,
    // TODO: can I just size an array at from_str time
    // TODO: want to hit with a CPU profiler
    grid: Vec<Vec<u8>>, // indexed by x (< h), y (< w)
}

impl WordSearch {
    fn points(&self) -> impl Iterator<Item = Point> {
        let (w, h) = (self.w, self.h);
        (0..h).flat_map(move |x| (0..w).map(move |y| Point { x, y }))
    }

    fn get(&self, p: Point) -> Option<u8> {
        self.grid.get(p.x).and_then(|v| v.get(p.y).copied())
    }

    fn num_xmas(&self, p: Point) -> u32 {
        let mut count = 0;
        if let Some(b'X') = self.get(p) {
            for dir in NEIGHBORS.iter() {
                let xmas = "XMAS".bytes().enumerate().all(|(i, c)| {
                    let neighbor = self.get(p.offset(*dir * i as i32));

                    neighbor.is_some_and(|n| n == c)
                });

                if xmas {
                    count += 1
                }
            }
        }
        count
    }

    fn describes_mas(&self, p: Point, dir: Offset) -> bool {
        let (m_a, s_a) = (self.get(p.offset(dir)), self.get(p.offset(-dir)));

        (m_a == Some(b'M') && s_a == Some(b'S')) || (m_a == Some(b'S') && s_a == Some(b'M'))
    }

    fn is_x_mas_a(&self, p: Point) -> bool {
        if let Some(b'A') = self.get(p) {
            for chunk in POSSIBLE_CROSSES.chunks(2) {
                if self.describes_mas(p, chunk[0]) && self.describes_mas(p, chunk[1]) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug)]
struct ParseWordSearchErr;

impl FromStr for WordSearch {
    type Err = ParseWordSearchErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<_>> = s.lines().map(|l| l.bytes().collect()).collect();

        let h = grid.len();
        let w = if let Some(v) = grid.first() {
            v.len()
        } else {
            0
        };

        Ok(WordSearch { w, h, grid })
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn offset(&self, o: Offset) -> Point {
        let x = (self.x as i32 + o.dx) as usize;
        let y = (self.y as i32 + o.dy) as usize;

        Point { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Offset {
    dx: i32,
    dy: i32,
}

lazy_static! {
    static ref NEIGHBORS: Vec<Offset> = {
        iproduct!([-1i32, 0, 1], [-1i32, 0, 1])
            .map(|(dx, dy)| Offset { dx, dy })
            .collect()
    };
    static ref POSSIBLE_CROSSES: Vec<Offset> = {
        iproduct!([-1i32, 1], [-1i32, 1])
            .map(|(dx, dy)| Offset { dx, dy })
            .collect()
    };
}

impl Mul<i32> for Offset {
    type Output = Offset;

    fn mul(self, rhs: i32) -> Self::Output {
        let (dx, dy) = (self.dx * rhs, self.dy * rhs);
        Offset { dx, dy }
    }
}

impl Neg for Offset {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Offset {
            dx: -self.dx,
            dy: -self.dy,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let ws = WordSearch::from_str(input).unwrap();

    Some(ws.points().map(|p| ws.num_xmas(p)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let ws = WordSearch::from_str(input).unwrap();

    Some(ws.points().filter(|&p| ws.is_x_mas_a(p)).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
