
pub fn main() {
    let data = include_str!("../../data/02.in");
    part1::solve_part1(data);
    part2::solve_part2(data);
}

pub type Set = std::collections::HashMap<String, u32>;

/// Parse the descriptions of the sets drawn in each game into concrete structures.
pub fn parse_cubes(s: &str) -> (u32, Vec<Set>) {
    let (game_with_index, rest) = s.split_once(':').unwrap();
    let index: u32 = game_with_index["Game ".len()..].parse().unwrap();
    let mut all_sets = vec![];

    for set in rest.split(';') {
        let set = set.trim();
        let items = set.split(',');
        let mut current = Set::default();
        for item in items {
            let item = item.trim();
            let (num, color) = item.split_once(' ').unwrap();
            let num: u32 = num.parse().unwrap();
            current.insert(color.to_string(), num);
        }
        all_sets.push(current);
    }

    (index, all_sets)
}

pub mod part1 {
    pub fn solve_part1(data: &str) {

        let mut allowed_cubes = std::collections::HashMap::new();
        allowed_cubes.insert("red", 12);
        allowed_cubes.insert("green", 13);
        allowed_cubes.insert("blue", 14);

        let total: u32 = 
            data
            .lines()
            .filter_map(|line| {
                let (idx, sets) = crate::parse_cubes(line);
                for set in sets.iter() {
                    // Can this set be extracted given the current cubes we have?
                    for (color, frequency) in set {
                        if !allowed_cubes.contains_key(color.as_str()) || allowed_cubes[color.as_str()] < *frequency {
                            return None;
                        }
                    }
                }
                Some(idx)
            })
            .sum();

        println!("part 1: {total}");
    }
}

pub mod part2 {

    pub fn solve_part2(data: &str) {
        let acc: u32 = 
            data
            .lines()
            .map(|line| {
                let (_, sets) = crate::parse_cubes(line);
                let mut result_map = std::collections::HashMap::<String, _>::new();
                for set in sets.iter() {
                    for (color, frequency) in set {
                        result_map
                        .entry(color.into())
                        .and_modify(|freq| { *freq = (*frequency).max(*freq) })
                        .or_insert(*frequency);
                    }
                }
                result_map.values().product::<u32>()  
            })
            .sum();

        println!("part 2: {acc}");
    }
}