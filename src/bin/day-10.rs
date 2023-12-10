use colored::Colorize;
use std::collections::binary_heap::BinaryHeap;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data = include_str!("../../data/10.in");
    println!("part 1: {}", solve_part1(data));
    println!("part 2: {}", solve_part2(data, false));
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    #[test]
    fn test_smol_data() {
        let data = r".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(4, solve_part1(data));
        assert_eq!(1, solve_part2(data, true));
    }
    #[test]
    fn test_medium_data() {
        let data = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(8, solve_part1(data));
        assert_eq!(1, solve_part2(data, true));
    }

    #[test]
    fn test_unused_loop() {
        let data = r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

        assert_eq!(22, solve_part1(data));
        assert_eq!(4, solve_part2(data, true));
    }

    #[test]
    fn test_large() {
        let data = r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        assert_eq!(70, solve_part1(data));
        assert_eq!(8, solve_part2(data, true));
    }

    #[test]
    fn test_large2() {
        let data = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(80, solve_part1(data));
        assert_eq!(10, solve_part2(data, true));
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord(isize, isize);

#[derive(Debug, Clone)]
pub struct Graph(Vec<Vec<char>>);

impl std::ops::Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Graph {
    pub fn get(&self, coord: Coord) -> Option<char> {
        if !self.is_valid(coord) {
            return None;
        }
        Some(
            *self
                .0
                .get(coord.0 as usize)
                .unwrap()
                .get(coord.1 as usize)
                .unwrap(),
        )
    }

    pub fn find_start(&self) -> Option<Coord> {
        for (row_idx, row) in self.0.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                if *value == 'S' {
                    return Some(Coord(row_idx as isize, col_idx as isize));
                }
            }
        }
        None
    }

    fn find_and_rectify_start_shape(&mut self) {
        let shape = self
            .find_start()
            .map(|coord| {
                let neighbors = self.get_neighbors_for_start(coord).unwrap();
                let mut shifts = neighbors
                    .iter()
                    .map(|&neighbor| neighbor - coord)
                    .collect::<Vec<_>>();

                if shifts.contains(&Coord(-1, 0)) {
                    shifts.retain(|shift| shift != &Coord(-1, 0));
                    return match shifts.first().unwrap() {
                        Coord(1, 0) => '|',
                        Coord(0, 1) => 'L',
                        Coord(0, -1) => 'J',
                        _ => panic!("invalid."),
                    };
                }

                if shifts.contains(&Coord(1, 0)) {
                    shifts.retain(|shift| shift != &Coord(1, 0));
                    return match shifts.first().unwrap() {
                        Coord(0, -1) => '7',
                        Coord(0, 1) => 'F',
                        _ => panic!("invalid."),
                    };
                }
                '-'
            })
            .unwrap();

        let start = self.find_start().unwrap();
        *self
            .0
            .get_mut(start.0 as usize)
            .unwrap()
            .get_mut(start.1 as usize)
            .unwrap() = shape;
    }

    fn mark_non_loop_as_ground(&mut self, loop_indices: &HashMap<Coord, i32>) {
        for (row_idx, row) in self.0.iter_mut().enumerate() {
            for (col_idx, value) in row.iter_mut().enumerate() {
                if !loop_indices.contains_key(&Coord(row_idx as isize, col_idx as isize)) {
                    *value = '.';
                }
            }
        }
    }

    fn get_neighbors_for_start(&self, coord: Coord) -> Option<Vec<Coord>> {
        let mut adjacent = vec![];

        // Check if north has a pipe that opens to the south.
        if let Some('|' | '7' | 'F') = self.get(Coord(coord.0 - 1, coord.1)) {
            adjacent.push(Coord(coord.0 - 1, coord.1));
        }

        // Check if south has a pipe that opens to the north.
        if let Some('|' | 'L' | 'J') = self.get(Coord(coord.0 + 1, coord.1)) {
            adjacent.push(Coord(coord.0 + 1, coord.1));
        }

        // Check if west has a pipe that opens to the east.
        if let Some('-' | 'L' | 'F') = self.get(Coord(coord.0, coord.1 - 1)) {
            adjacent.push(Coord(coord.0, coord.1 - 1));
        }

        // Check if east has a pipe that opens to the west.
        if let Some('-' | 'J' | '7') = self.get(Coord(coord.0, coord.1 + 1)) {
            adjacent.push(Coord(coord.0, coord.1 + 1));
        }
        Some(adjacent)
    }

    fn neighbors(&self, coord: Coord) -> Option<impl Iterator<Item = Coord> + '_> {
        let Some(character) = self.get(coord) else {
            return None;
        };

        let shifts = match character {
            '|' => vec![(-1, 0), (1, 0)],
            '-' => vec![(0, -1), (0, 1)],
            'L' => vec![(-1, 0), (0, 1)],
            'J' => vec![(-1, 0), (0, -1)],
            '7' => vec![(1, 0), (0, -1)],
            'F' => vec![(1, 0), (0, 1)],
            '.' => vec![],
            _ => unreachable!("invalid character"),
        }
        .into_iter()
        .map(|(row, col)| Coord(row, col));

        Some(
            shifts
                .filter(move |shift| self.is_valid(coord + *shift))
                .map(move |shift| shift + coord)
                .filter(move |pos| self.get(*pos) != Some('.')),
        )
    }
    pub fn is_valid(&self, coord: Coord) -> bool {
        0 <= coord.0
            && coord.0 <= (self.0.len() as isize)
            && 0 <= coord.1
            && coord.1 <= (self.0.get(0).unwrap().len() as isize)
    }

    fn mark_and_show_cells(
        &self,
        inside: &[Coord],
        outside: &[Coord],
        loop_indices: &HashMap<Coord, i32>,
    ) {
        let mut graph: Vec<Vec<String>> = self
            .clone()
            .0
            .into_iter()
            .map(|row| row.into_iter().map(String::from).collect())
            .collect();
        for coord in inside {
            let v = graph
                .get_mut(coord.0 as usize)
                .unwrap()
                .get_mut(coord.1 as usize)
                .unwrap();
            *v = "I".green().to_string();
        }
        for coord in outside {
            let v = graph
                .get_mut(coord.0 as usize)
                .unwrap()
                .get_mut(coord.1 as usize)
                .unwrap();
            *v = "O".red().to_string();
        }
        for coord in loop_indices.keys() {
            let v = graph
                .get_mut(coord.0 as usize)
                .unwrap()
                .get_mut(coord.1 as usize)
                .unwrap();
            *v = v.blue().to_string();
        }

        for line in graph.iter() {
            let joined = line
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("");
            println!("{}", joined);
        }
        println!();
    }

    pub fn single_source_shortest_paths(&self, source: Coord) -> HashMap<Coord, i32> {
        let mut distances = HashMap::new();
        let mut queue = BinaryHeap::new();
        queue.push((0, source));

        while !queue.is_empty() {
            let (dist, current) = queue.pop().unwrap();
            distances.insert(current, -dist);

            if let Some(neighbors) = self.neighbors(current).map(|it| it.collect::<Vec<_>>()) {
                for neighbor in neighbors.iter() {
                    if !distances.contains_key(neighbor) {
                        queue.push((-(-dist + 1), *neighbor));
                    }
                }
            }
        }

        distances
    }
}

pub fn parse_graph(data: &str) -> Graph {
    Graph(data.lines().map(|line| line.chars().collect()).collect())
}

pub fn solve_part1(data: &str) -> i32 {
    let mut graph = parse_graph(data);
    let source = graph.find_start().unwrap();
    graph.find_and_rectify_start_shape();

    let shortest_paths = graph.single_source_shortest_paths(source);

    shortest_paths.values().copied().max().unwrap()
}

pub fn solve_part2(data: &str, debug: bool) -> usize {
    let mut graph = parse_graph(data);
    let source = graph.find_start().unwrap();
    graph.find_and_rectify_start_shape();

    let shortest_paths = graph.single_source_shortest_paths(source);
    graph.mark_non_loop_as_ground(&shortest_paths);

    let mut remaining_indices = HashSet::new();
    let num_columns = graph.0.get(0).unwrap().len();
    let num_rows = graph.0.len();

    for row_idx in 0..num_rows {
        for col_idx in 0..num_columns {
            if !shortest_paths.contains_key(&Coord(row_idx as isize, col_idx as isize)) {
                remaining_indices.insert(Coord(row_idx as isize, col_idx as isize));
            }
        }
    }

    let mut inside = vec![];
    let mut outside = vec![];

    // Check every non-loop index to see whether it lies inside the loop or outside.
    // Take the winding number approach by scanning a row from left to right
    // and flipping inside/outside status for a cell based on the parity of walls
    // encountered. Since we started outside the loop, an odd number of wall crossings
    // imply we're inside the loop, and similarly, an even number of wall crossings imply
    // we're outside the loop again.
    let sum = remaining_indices
        .iter()
        .map(|&coord| {
            if coord.1 == 0 {
                outside.push(coord);
                return 0;
            }
            // Count the north-facing wall crossings (i.e. '|' | 'J' | 'L') on the left of coord.
            let left_strip = graph
                .0
                .get(coord.0 as usize)
                .unwrap()
                .iter()
                .take(coord.1 as usize);
            let north_facing = left_strip.filter(|&c| matches!(c, '|' | 'J' | 'L')).count();
            if north_facing % 2 == 1 {
                inside.push(coord);
                1
            } else {
                outside.push(coord);
                0
            }
        })
        .sum();

    if debug {
        // arghh, print the inside/outside as color-coded for debug.
        graph.mark_and_show_cells(&inside, &outside, &shortest_paths);
    }

    sum
}
