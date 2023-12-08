use aoc_2023::math::lcm;


pub fn main() {
    let data = include_str!("../../data/08.in");
    println!("part 1: {}", solve_part1(data));
    println!("part 2: {}", solve_part2(data));
}


/// Compute the minimum number of steps needed to traverse from "AAA" to "ZZZ"
/// given we could only follow a never-ending cycle of instructions.
pub fn solve_part1(data: &str) -> usize {
    let (seq, mapping) = parse_sequence_and_mappings(data);
    count_steps(&seq, &mapping, "AAA", |node| node == "ZZZ")
}


/// Compute the least common multiple of the minimum number of steps needed for each starting node to reach an ending node
/// following left/right from a never-ending cycle of instructions.
pub fn solve_part2(data: &str) -> usize {
    let (seq, mapping) = parse_sequence_and_mappings(data);

        mapping
        .keys()
        .filter(|&node| node.ends_with('A'))
        .map(|node| {
            count_steps(&seq, &mapping, node, |node| node.ends_with('Z'))
        })
        .reduce(lcm)
        .expect("starting nodes to be non-empty.")
}

pub type Mapping = std::collections::HashMap<String, [String; 2]>;


/// Traverse the mapping following the left/right instructions from the sequence
/// and stop when the start node satisfies the ending condition.
pub fn count_steps<F>(seq: &[usize], mapping: &Mapping, start_node: &str, end_cond: F) -> usize 
where
    F: Fn(&str) -> bool
{
    let mut start_node = start_node;
    let mut counter = 0;

    for &selection in seq.iter().cycle() {
        if end_cond(start_node) {
            break;
        }
        start_node = mapping.get(start_node).expect("start_node to exist in the mapping")[selection].as_str();
        counter += 1;
    }

    counter
}


#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    #[test]
    fn test_smol_data() {
        let data = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(solve_part1(data), 2);
        assert_eq!(solve_part2(data), 2);
    }

    #[test]
    fn test_part1_data2() {
        let data = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve_part1(data), 6);
        assert_eq!(solve_part2(data), 6);
    }

    #[test]
    fn test_part2_data1() {
        let data = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(solve_part2(data), 6);
    }
}


pub fn parse_sequence_and_mappings(data: &str) -> (Vec<usize>, Mapping) {
    let blocks: Vec<String> = data.split("\n\n").map(String::from).collect();

    let sequence = 
        blocks
        .first()
        .unwrap()
        .chars()
        .map(|c| {
            match c {
                'R' => 1usize,
                'L' => 0,
                _ => unreachable!("only R or L expected")
            }
        })
        .collect::<Vec<_>>();

    let mappings: Mapping = 
        blocks
        .last()
        .unwrap()
        .lines()
        .map(|line| {
            let (start, rest) = line.split_once(" = ").unwrap();
            let left: String = rest.chars().skip(1).take(3).collect();
            let right: String = rest.chars().skip(6).take(3).collect();
            (start.to_string(), [left, right])
        })
        .collect();

    (sequence, mappings)
}

