use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let data = include_str!("../../data/16.in");
    println!("part 1: {}", solve_part1(data));
    println!("part 2: {}", solve_part2(data));
}

pub fn solve_part1(data: &str) -> usize {
    let mirrors = Mirrors::new(data);
    mirrors.trace_rays((0, 0), Direction::East).len()
}

pub fn solve_part2(data: &str) -> usize {
    let mirrors = Mirrors::new(data);

    let mut sources_and_directions = vec![];

    // Check rays going down from the top edge.
    sources_and_directions
        .extend((0..mirrors.columns).map(|col_idx| ((0, col_idx as isize), Direction::South)));
    // Check rays going up from the bottom edge.
    sources_and_directions.extend((0..mirrors.columns).map(|col_idx| {
        (
            (mirrors.rows as isize - 1, col_idx as isize),
            Direction::North,
        )
    }));
    // Check rays going right from the left edge.
    sources_and_directions
        .extend((0..mirrors.rows).map(|row_idx| ((row_idx as isize, 0), Direction::East)));
    // Check rays going right from the right edge.
    sources_and_directions.extend((0..mirrors.rows).map(|row_idx| {
        (
            (row_idx as isize, mirrors.columns as isize - 1),
            Direction::West,
        )
    }));

    sources_and_directions
        .par_iter()
        .map(|(source, direction)| mirrors.trace_rays(*source, *direction).len())
        .max()
        .unwrap()
}

pub type Point = (isize, isize);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Mirrors {
    inner: HashMap<Point, u8>,
    rows: usize,
    columns: usize,
}

impl Mirrors {
    #[inline(always)]
    pub fn is_mirror(c: u8) -> bool {
        c == b'-' || c == b'|' || c == b'/' || c == b'\\'
    }
    pub fn new(data: &str) -> Self {
        let inner = HashMap::from_iter(data.lines().enumerate().flat_map(|(row_idx, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, value)| Self::is_mirror(*value as u8))
                .map(move |(col_idx, value)| ((row_idx as isize, col_idx as isize), value as u8))
        }));

        let grid = data.lines().collect::<Vec<_>>();

        Self {
            inner,
            rows: grid.len(),
            columns: grid[0].len(),
        }
    }

    #[inline(always)]
    pub fn next_coordinate(&self, source: Point, direction: Direction) -> Option<Point> {
        let (row, col) = (source.0, source.1);
        match direction {
            Direction::East if col < self.columns as isize - 1 => Some((row, col + 1)),
            Direction::North if row > 0 => Some((row - 1, col)),
            Direction::West if col > 0 => Some((row, col - 1)),
            Direction::South if row < self.rows as isize - 1 => Some((row + 1, col)),
            _ => None,
        }
    }

    pub fn step(
        &self,
        outgoing: Direction,
        coordinate: Point,
    ) -> impl Iterator<Item = (Point, Direction)> {
        let char = *self
            .inner
            .get(&(coordinate.0, coordinate.1))
            .unwrap_or(&b'.');

        let mut starting_points = vec![];

        match (char, outgoing) {
            // Reflect south.
            (b'\\', Direction::East) | (b'/', Direction::West) => {
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::South) {
                    starting_points.push((neighbor, Direction::South));
                }
            }
            // Reflect west.
            (b'\\', Direction::North) | (b'/', Direction::South) => {
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::West) {
                    starting_points.push((neighbor, Direction::West));
                }
            }
            // Reflect north.
            (b'\\', Direction::West) | (b'/', Direction::East) => {
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::North) {
                    starting_points.push((neighbor, Direction::North));
                }
            }
            // Reflect east.
            (b'\\', Direction::South) | (b'/', Direction::North) => {
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::East) {
                    starting_points.push((neighbor, Direction::East));
                }
            }
            // Split north and south.
            (b'|', Direction::East | Direction::West) => {
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::North) {
                    starting_points.push((neighbor, Direction::North));
                }
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::South) {
                    starting_points.push((neighbor, Direction::South));
                }
            }
            // Split east and west.
            (b'-', Direction::North | Direction::South) => {
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::East) {
                    starting_points.push((neighbor, Direction::East));
                }
                if let Some(neighbor) = self.next_coordinate(coordinate, Direction::West) {
                    starting_points.push((neighbor, Direction::West));
                }
            }
            // keep going same direction
            _ => {
                if let Some(neighbor) = self.next_coordinate(coordinate, outgoing) {
                    starting_points.push((neighbor, outgoing));
                }
            }
        }

        starting_points.into_iter()
    }

    pub fn trace_rays(&self, source: Point, direction: Direction) -> HashSet<Point> {
        let mut seen = HashSet::new();
        let mut tiles_seen = HashSet::new();
        let mut queue = vec![(source, direction)]
            .into_iter()
            .collect::<VecDeque<_>>();

        while let Some((source, direction)) = queue.pop_front() {
            tiles_seen.insert(source); // track the distinct tiles that were covered.
            seen.insert((source, direction)); // track which direction the ray was facing when it got here.
            queue.extend(
                self.step(direction, source)
                    .filter(|(src, dir)| !seen.contains(&(*src, *dir))),
            )
        }
        tiles_seen
    }
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
