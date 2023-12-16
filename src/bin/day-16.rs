use std::collections::{HashMap, HashSet, VecDeque};
use rayon::prelude::*;

pub type Point = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug, Clone)]
pub struct Mirrors {
    inner: HashMap<Point, u8>,
    rows: usize,
    columns: usize,
}


impl std::fmt::Display for Mirrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row_idx in 0..self.rows {
            for col_idx in 0..self.columns {
                _ = write!(f, "{}", *self.inner.get(&(row_idx as isize, col_idx as isize)).unwrap_or(&b'.') as char);
            }
            _ = writeln!(f);
        }
        Ok(())
    }
}

impl Mirrors {

    #[inline(always)]
    pub fn is_mirror(c: u8) -> bool {
        c == b'-' || c == b'|' || c == b'/' || c == b'\\'
    }
    pub fn new(data: &str) -> Self {
        let inner = HashMap::from_iter(
            data
            .lines()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row
                .chars()
                .enumerate()
                .filter(|(_, value)| Self::is_mirror(*value as u8))
                .map(move |(col_idx, value)| ((row_idx as isize, col_idx as isize), value as u8))
            })
        );

        let grid = data.lines().collect::<Vec<_>>();

        Self {
            inner,
            rows: grid.len(),
            columns: grid[0].len()
        }

    }

    pub fn next_coordinate(&self, source: Point, direction: Direction) -> Option<Point> {
        let (row, col) = (source.0, source.1);
        match direction {
            Direction::East => {
                // we can go east.
                if col < self.columns as isize - 1 {
                    Some((row, col + 1))
                } else {
                    None
                }
            },
            Direction::North => {
                // we can go north.
                if row > 0 {
                    Some((row - 1, col))
                } else {
                    None
                }
            },
            Direction::West => {
                // we can go west.
                if col > 0 {
                    Some((row, col - 1))
                } else {
                    None
                }
            },
            Direction::South => {
                // we can go south.
                if row < self.rows as isize - 1 {
                    Some((row + 1, col))
                } else {
                    None
                }
            }
        }
    }

    pub fn step(&self, outgoing: Direction, coordinate: Point) -> Vec<(Point, Direction)> {
        let value = *self.inner.get(&(coordinate.0, coordinate.1)).unwrap_or(&b'.');
        let mut starting_points = vec![];

        match (value, outgoing) {
            // Reflect down.
            (b'\\', Direction::East) | (b'/', Direction::West) => {
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::South) {
                    starting_points.push((neighbor, Direction::South));
                }
            },
            // Reflect left.
            (b'\\', Direction::North) | (b'/', Direction::South) => {
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::West) {
                    starting_points.push((neighbor, Direction::West));
                }
            },
            // Reflect up.
            (b'\\', Direction::West) | (b'/', Direction::East) => {
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::North) {
                    starting_points.push((neighbor, Direction::North));
                }
            },
            // Reflect right.
            (b'\\', Direction::South) | (b'/', Direction::North) => {
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::East) {
                    starting_points.push((neighbor, Direction::East));
                }
            },
            // Split up and down.
            (b'|', Direction::East | Direction::West) => {
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::North) {
                    starting_points.push((neighbor, Direction::North));
                }
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::South) {
                    starting_points.push((neighbor, Direction::South));
                }
            },

            // Split left and right.
            (b'-', Direction::North | Direction::South) => {
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::East) {
                    starting_points.push((neighbor, Direction::East));
                }
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::West) {
                    starting_points.push((neighbor, Direction::West));
                }
            },
            // (b'.', _) 
            // | (b'|', Direction::North | Direction::South) 
            // | (b'-', Direction::East | Direction::West)
            _ => {
                if let Some(neighbor) = self.next_coordinate(coordinate, outgoing) {
                    starting_points.push((neighbor, outgoing));
                }
            },
        }

        starting_points
    }

    pub fn trace_rays(&self, source: Point, direction: Direction) -> HashSet<Point> {
        let mut seen = HashSet::new();
        let mut tiles_seen = HashSet::new();
        let mut queue = vec![(source, direction)].into_iter().collect::<VecDeque<_>>();

        while let Some((source, direction)) = queue.pop_front() {
            tiles_seen.insert(source);
            seen.insert((source, direction));
            let steps = self.step(direction, source);
            queue.extend(
                steps
                .into_iter()
                .filter(|(src, dir)| !seen.contains(&(*src, *dir)))
            )
        }
        tiles_seen
    }
}

pub fn solve_part1(data: &str) -> usize {
    let mirrors = Mirrors::new(data);
    mirrors.trace_rays((0, 0), Direction::East).len()
}

pub fn solve_part2(data: &str) -> usize {
    let mirrors = Mirrors::new(data);

    let mut sources_and_directions = vec![];

    // Check rays going down from the top edge.
    sources_and_directions.extend(
        (0..mirrors.columns).map(|col_idx| ((0, col_idx as isize), Direction::South))
    );
    // Check rays going up from the bottom edge.
    sources_and_directions.extend(
        (0..mirrors.columns).map(|col_idx| ((mirrors.rows as isize - 1, col_idx as isize), Direction::North))
    );
    // Check rays going right from the left edge.
    sources_and_directions.extend(
        (0..mirrors.rows).map(|row_idx| ((row_idx as isize, 0), Direction::East))
    );
    // Check rays going right from the right edge.
    sources_and_directions.extend(
        (0..mirrors.rows).map(|row_idx| ((row_idx as isize, mirrors.columns as isize - 1), Direction::West))
    );

    sources_and_directions
    .par_iter()
    .map(|(source, direction)| mirrors.trace_rays(*source, *direction).len())
    .max()
    .unwrap()
}

fn main() {
    let data = include_str!("../../data/16.in");
    println!("part 1: {}", solve_part1(data));
    println!("part 2: {}", solve_part2(data));
}

#[cfg(test)]
mod tests {
    use crate::{solve_part1, solve_part2};


    #[test]
    fn smol() {
        let data = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(solve_part1(data), 46);
        assert_eq!(solve_part2(data), 51);
    }
}