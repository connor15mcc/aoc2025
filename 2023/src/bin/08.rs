use std::{
    collections::{HashMap, VecDeque},
    ops::{Div, Mul, Rem},
    str::FromStr,
};

advent_of_code::solution!(8);

#[derive(Debug)]
struct Document {
    instructions: VecDeque<char>,
    nodes: HashMap<String, Node>,
}

impl Document {
    fn steps_to_end(&self, start: &str) -> u64 {
        let mut steps = 0;
        let mut current = start;
        let mut instructions = self.instructions.iter().cycle();

        while !current.ends_with('Z') {
            let direction = instructions.next().unwrap();
            let Node { left, right, .. } = self.nodes.get(current).unwrap();
            current = match direction {
                'L' => left,
                'R' => right,
                _ => panic!("inscrutable direction"),
            };
            steps += 1;
        }

        steps
    }
}

#[derive(Debug)]
struct ParseDocumentErr;

impl FromStr for Document {
    type Err = ParseDocumentErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, rest) = s.split_once("\n\n").unwrap();
        let instructions = first.to_string().chars().collect();
        let nodes = rest
            .lines()
            .map(|l| {
                let n = Node::from_str(l).unwrap();
                (n.id.clone(), n)
            })
            .collect();

        Ok(Document {
            instructions,
            nodes,
        })
    }
}

#[derive(parse_display::FromStr, Debug, Clone)]
#[display("{id} = ({left}, {right})")]
struct Node {
    id: String,
    left: String,
    right: String,
}

// snippet attributable to "Rust Snippets for Competitive Programming":
// https://bamgoesn.github.io/rust-ps-md/math/gcd.html
fn gcd<T>(x: T, y: T) -> T
where
    T: Copy + PartialEq + PartialOrd + Rem<Output = T> + From<u8>,
{
    if y == 0.into() {
        x
    } else {
        let v = x % y;
        gcd(y, v)
    }
}

fn lcm<T>(x: T, y: T) -> T
where
    T: Copy
        + PartialEq
        + PartialOrd
        + Rem<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + From<u8>,
{
    x / gcd(x, y) * y
}

pub fn part_one(input: &str) -> Option<u32> {
    let doc = Document::from_str(input).unwrap();

    let mut instructions = doc.instructions.to_owned();
    let mut node_id = "AAA";
    let mut steps = 0;

    while let Some(dir) = instructions.pop_front() {
        let node = doc.nodes.get(node_id).unwrap();
        steps += 1;
        let next_id = if dir == 'L' { &node.left } else { &node.right };
        if next_id == "ZZZ" {
            return Some(steps);
        }
        node_id = next_id;

        // repeat if necessary
        if instructions.is_empty() {
            instructions = doc.instructions.to_owned();
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let doc = Document::from_str(input).unwrap();

    let mut result = 1; // this is a good first lower bound

    for starting_node in doc.nodes.keys().filter(|id| id.ends_with('A')) {
        let steps = doc.steps_to_end(starting_node);
        result = lcm(result, steps);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
