use phf::phf_map;

advent_of_code::solution!(10);

type Grid = [Vec<char>];

type Point = (usize, usize);

type Offset = (i32, i32);

fn input_to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[derive(Debug)]
struct GetFromGridErr;

fn get_val(grid: &Grid, (y, x): Point) -> Result<char, GetFromGridErr> {
    let c = grid
        .get(y)
        .ok_or(GetFromGridErr)?
        .get(x)
        .ok_or(GetFromGridErr)?;

    Ok(c.to_owned())
}

fn starting_pos(grid: &Grid) -> Point {
    let mut starts = grid
        .iter()
        .enumerate()
        .filter_map(|(y, line)| line.iter().position(|&c| c == 'S').map(|x| (y, x)));

    let (y, x) = starts
        .next()
        .expect("exactly one starting pos should exist");

    (y, x)
}

static NEIGHBOR_OFFSETS: [Offset; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn neighbors(pt: Point) -> Vec<Point> {
    let mut neighbors = vec![];
    for off in NEIGHBOR_OFFSETS {
        neighbors.push(add_offset(pt, off));
    }

    neighbors
}

// we can't start at start, since it doesn't explicitly connect to pipes
fn start_neighbors(grid: &Grid, pt: Point) -> (Point, Point) {
    let mut result = vec![];

    for neighbor in neighbors(pt) {
        if let Ok(nval) = get_val(grid, neighbor) {
            if nval == '.' {
                continue;
            }

            let pmoves = possible_moves(grid, neighbor);
            if pt == pmoves.0 || pt == pmoves.1 {
                result.push(neighbor);
            }
        }
    }

    (result[0], result[1])
}

static PIPE_OFFSETS: phf::Map<char, (Offset, Offset)> = phf_map! {
    '|' => ((1,0), (-1, 0)),
    '-' => ((0,1), (0, -1)),
    'L' => ((-1,0), (0, 1)),
    'J' => ((-1,0), (0, -1)),
    '7' => ((0,-1), (1, 0)),
    'F' => ((0,1), (1, 0)),
};

fn next_move_offsets(c: char) -> (Offset, Offset) {
    PIPE_OFFSETS.get(&c).unwrap().to_owned()
}

fn add_offset((y, x): Point, (y_off, x_off): Offset) -> Point {
    ((y as i32 + y_off) as usize, (x as i32 + x_off) as usize)
}

fn possible_moves(grid: &Grid, pt: Point) -> (Point, Point) {
    let offsets = next_move_offsets(get_val(grid, pt).unwrap());

    (add_offset(pt, offsets.0), add_offset(pt, offsets.1))
}

fn shoelace(vertices: &[Point]) -> f32 {
    let area = vertices
        .windows(2)
        .map(|w| (w[0].0 as i32 + w[1].0 as i32) * (w[0].1 as i32 - w[1].1 as i32))
        .sum::<i32>()
        / 2;
    (area as f32).abs()
}

fn picks(area: usize, boundary_len: usize) -> usize {
    area - (boundary_len / 2) + 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = &input_to_grid(input);

    let starting_pos = starting_pos(grid);
    let mut positions = vec![starting_pos];
    let mut current = start_neighbors(grid, starting_pos).0; // chose 0/1 arbitrarily

    loop {
        let prev = positions.last().unwrap().to_owned();
        positions.push(current);

        let (a, b) = possible_moves(grid, current);
        if (a == starting_pos || b == starting_pos) && prev != starting_pos {
            return Some(positions.len().div_ceil(2) as u32);
        }

        current = if b == prev { a } else { b }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = &input_to_grid(input);

    let starting_pos = starting_pos(grid);
    let mut positions = vec![starting_pos];
    let mut current = start_neighbors(grid, starting_pos).0; // chose 0/1 arbitrarily

    loop {
        let prev = positions.last().unwrap().to_owned();
        positions.push(current);

        let (a, b) = possible_moves(grid, current);
        if (a == starting_pos || b == starting_pos) && prev != starting_pos {
            break;
        }

        current = if b == prev { a } else { b }
    }
    dbg!(&positions);
    let area = shoelace(&positions) as usize;

    let boundary_len = positions.len();
    let interior_points = picks(area, boundary_len);
    Some(interior_points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
