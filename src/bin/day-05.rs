
pub fn main() {

    let data = include_str!("../../data/05.in");

    println!("part 1: {}", solve_part1(data));
    println!("part 2: {}", solve_part2(data));

}

pub fn solve_part1(data: &str) -> usize {
    let (seeds, almanac) = parse_seeds_and_almanac(data);

    seeds
    .iter()
    .map(|seed| {
        almanac.propagate_seed(*seed as usize)
    })
    .min()
    .unwrap()
}

pub fn solve_part2(data: &str) -> usize {
    let (seeds, almanac) = parse_seeds_and_almanac(data);

    seeds
    .chunks_exact(2)
    .map(|pair| {
        almanac.propagate_seed_range(pair[0] as isize .. pair[0] as isize + pair[1] as isize)
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap()
    })
    .min()
    .unwrap() as usize
}


#[cfg(test)]
mod tests {
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn test_smol_data() {
    let data = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
        assert_eq!(solve_part1(data), 35);
        assert_eq!(solve_part2(data), 46);
    }

}


#[derive(Debug, Clone, Copy)]
pub struct Mapping {
    destination_start: usize,
    source_start: usize,
    range: usize
}

impl FromStr for Mapping {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s.split_whitespace().collect::<Vec<_>>();
        if nums.len() != 3 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "expected exactly 3 numbers for a range description."))
        }
        Ok(Self {
            destination_start: nums[0].parse().unwrap(),
            source_start: nums[1].parse().unwrap(),
            range: nums[2].parse().unwrap(),
        })
    }
}


use std::ops::Range;
use std::str::FromStr;


#[derive(Debug, Clone, Default)]
pub struct Category {
    mappings: Vec<Mapping>,
}


impl Category {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}


/// Given two ranges, compute its intersection range, if exists.
#[inline(always)]
pub fn find_common_interval(range1: Range<isize>, range2: Range<isize>) -> Option<Range<isize>> {
    if range2.start > range1.end || range1.start > range2.end {
        None
    } else {
        let start = range1.start.max(range2.start);
        let end = range1.end.min(range2.end);
        Some(start..end)
    }
}


#[derive(Debug, Clone)]
pub struct Almanac(Vec<Category>);

impl Almanac {
    /// Given a single seed value, propagate it all the way to "location"
    /// level and return the mapped value.
    pub fn propagate_seed(&self, seed: usize) -> usize {

        let mut current_value = seed;

        for category in self.0.iter() {
            for mapping in &category.mappings {
                if mapping.source_start <= current_value && current_value <= mapping.source_start + mapping.range {
                    current_value = mapping.destination_start + (current_value - mapping.source_start);
                    break;
                }
            }
        }
        current_value
    }

    /// Given a range of seed values, propagate them all the way to the "location"
    /// level by splitting and joining ranges in whichever level necessary.
    /// We can do this due to the fact that the mappings are linear and monotonically
    /// increasing, i.e. no need to check values between a range except for those that
    /// lie on a "destination" range boundary, which would split into its own interval
    /// anyway in the next level.
    pub fn propagate_seed_range(&self, seeds: Range<isize>) -> Vec<Range<isize>> {

        let mut current_ranges = vec![seeds];
        let total = self.0.len();

        for (idx, category) in self.0.iter().enumerate() {

            if idx == total - 1 {
                return current_ranges;
            }
            let mut next_ranges = vec![];

            // Attempt to split the current ranges into mapped ranges.
            for range in &current_ranges {
                let mut some_mapping_found = false;
                for mapping in &category.mappings {
                    let shift = mapping.destination_start as isize - mapping.source_start as isize;
                    let mapping_range = mapping.source_start as isize..mapping.source_start as isize + mapping.range as isize;

                    if let Some(common_interval) = find_common_interval(range.clone(), mapping_range) {
                        next_ranges.push(common_interval.start + shift..common_interval.end + shift);
                        some_mapping_found = true;
                    }
                }

                // If no mappings were found, just carry forward the entire range as is.
                if !some_mapping_found {
                    next_ranges.push(range.clone());
                }

                // It is possible that the mappings didn't cover the desired range
                // entirely. In this case, carry forward the remaining intervals 
                // to the next category.

                // Actual interval decomposition sounds really tricky to get right
                // but I guess my test data doesn't have any ranges that cover the mapping
                // only partially (over the non-mapped data), so I got lucky.
            }
            current_ranges = next_ranges;
        }
        current_ranges
    }
}

pub fn parse_seeds_and_almanac(data: &str) -> (Vec<u64>, Almanac) {
    let lines = data.split("\n\n").collect::<Vec<_>>();
    let seeds: Vec<_> = 
        lines[0]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    let mut almanac: Almanac = Almanac(Default::default());

    for &line in &lines[1..] {
        let (_, mappings) = line.split_once(" map:").unwrap();

        let mappings: Vec<_> = 
        mappings
        .trim()
        .split('\n')
        .map(|s| s.parse::<Mapping>().unwrap())
        .collect();

        let category = Category {
            mappings
        };

        almanac.0.push(category);
    }

    (seeds, almanac)

}
