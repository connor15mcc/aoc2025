use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

advent_of_code::solution!(5);

#[derive(Debug)]
struct ParseAlmanacErr;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<RangeInclusive<u64>>,
    maps: HashMap<String, Map>,
}

impl Almanac {
    fn min_soil_number(&self, seeds: Vec<RangeInclusive<u64>>) -> u64 {
        let mut maps = vec![];
        let mut map = self.maps.get("seed");
        while let Some(m) = map {
            let next = self.maps.get(&m.dst_category);
            maps.push(m);
            map = next;
        }

        let values = maps.iter().fold(seeds, |values, range_map| {
            values
                .iter()
                .flat_map(|r| range_map.correspond(r.clone()))
                .collect()
        });

        values.iter().map(|id| id.start()).min().unwrap().to_owned()
    }

    fn with_ranges(s: &str) -> Result<Self, ParseAlmanacErr> {
        let parts = s.split_once("\n").unwrap();

        let seeds = parts
            .0
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .chunks(2)
            .map(|chunk| {
                let (start, len) = (
                    chunk[0].parse::<u64>().unwrap(),
                    chunk[1].parse::<u64>().unwrap(),
                );
                let end = start + len;
                start..=end - 1
            })
            .collect::<Vec<RangeInclusive<u64>>>();

        let maps = parts
            .1
            .split("\n\n")
            .map(|s| {
                let map: Map = s.parse().unwrap();
                Ok((map.src_category.to_owned(), map))
            })
            .collect::<Result<HashMap<String, Map>, ParseAlmanacErr>>()?;

        Ok(Almanac { seeds, maps })
    }
}

impl FromStr for Almanac {
    type Err = ParseAlmanacErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once("\n").unwrap();

        let seeds = parts
            .0
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|s| {
                let id = s.parse::<u64>().unwrap();
                id..=id
            })
            .collect::<Vec<RangeInclusive<u64>>>();

        let maps = parts
            .1
            .split("\n\n")
            .map(|s| {
                let map: Map = s.parse().unwrap();
                Ok((map.src_category.to_owned(), map))
            })
            .collect::<Result<HashMap<String, Map>, ParseAlmanacErr>>()?;

        Ok(Almanac { seeds, maps })
    }
}

#[derive(Debug, Clone)]
struct Map {
    src_category: String,
    dst_category: String,
    ranges: Vec<ConversionRange>,
}

impl Map {
    fn correspond(&self, input: RangeInclusive<u64>) -> Vec<RangeInclusive<u64>> {
        let mut ranges = vec![input.clone()];
        let mut result = vec![];
        for range_map in self.ranges.iter() {
            let mut unchanged = vec![];

            // mapped ranges are ready to return, unmapped are forwarded along
            for range in ranges.iter() {
                let (below, overlap, above) = range_map.convert(range.clone());
                if let Some(below) = below {
                    unchanged.push(below);
                }
                if let Some(overlap) = overlap {
                    result.push(overlap);
                }
                if let Some(above) = above {
                    unchanged.push(above);
                }
            }

            ranges.clear();
            ranges.append(&mut unchanged);
        }

        // unaltered ranges are completed via the identity function
        result.append(&mut ranges);

        result
    }
}

impl FromStr for Map {
    type Err = ParseAlmanacErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim_start().split_once("\n").unwrap();

        let (src_category, dst_category) = {
            let name: Vec<&str> = parts.0.strip_suffix(" map:").unwrap().split("-").collect();
            assert!(name.len() == 3);
            (name[0].to_owned(), name[2].to_owned())
        };

        let ranges = parts
            .1
            .lines()
            .map(|s| s.parse().or(Err(ParseAlmanacErr)))
            .collect::<Result<Vec<ConversionRange>, ParseAlmanacErr>>()?;

        Ok(Map {
            src_category,
            dst_category,
            ranges,
        })
    }
}

#[derive(Debug, Clone)]
struct ConversionRange {
    src: u64,
    dst: u64,
    len: u64,
}

impl ConversionRange {
    // returns (below, overlapping & mapped, above)
    fn convert(
        &self,
        inp: RangeInclusive<u64>,
    ) -> (
        Option<RangeInclusive<u64>>,
        Option<RangeInclusive<u64>>,
        Option<RangeInclusive<u64>>,
    ) {
        let (input_start, input_end) = inp.clone().into_inner();
        let src_end = self.src + self.len - 1;

        let below = if input_start < self.src {
            Some(input_start..=self.src.saturating_sub(1).min(input_end))
        } else {
            None
        };

        let overlap = if input_end >= self.src && input_start <= src_end {
            let overlap_start = input_start.max(self.src);
            let overlap_end = input_end.min(src_end);
            Some((self.dst + overlap_start - self.src)..=(self.dst + overlap_end - self.src))
        } else {
            None
        };

        let above = if input_end > src_end {
            Some(src_end.saturating_add(1).max(input_start)..=input_end)
        } else {
            None
        };

        (below, overlap, above)
    }
}

impl FromStr for ConversionRange {
    type Err = ParseAlmanacErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        assert!(parts.len() == 3);

        let (dst, src, len): (u64, u64, u64) = (
            parts[0].parse().unwrap(),
            parts[1].parse().unwrap(),
            parts[2].parse().unwrap(),
        );

        Ok(ConversionRange { src, dst, len })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac = Almanac::from_str(input).unwrap();

    Some(almanac.min_soil_number(almanac.seeds.to_owned()))
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac = Almanac::with_ranges(input).unwrap();

    dbg!(&almanac);

    Some(almanac.min_soil_number(almanac.seeds.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
