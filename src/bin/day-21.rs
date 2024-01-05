use aoc_2023::data_structures;
use std::collections::{HashMap, HashSet, VecDeque};
use colorgrad::magma;
use colored::{Colorize, CustomColor};


pub fn main() {
    let data = include_str!("../../data/21.in");
    let grid = parse_grid(data);
    // println!("part 1: {}", solve_part1(data, 64));
    println!("part 1: {}", solve_part1(&grid, 64));
    println!("part 2: {}", solve_part2(&grid, 26501365));
}

pub type Grid2D = Vec<Vec<u8>>;

pub trait GetDistances {
    fn get_distances(&self, max_steps: usize) -> Vec<Vec<isize>>;
}

impl GetDistances for Grid2D {
    fn get_distances(&self, max_steps: usize) -> Vec<Vec<isize>> {
        let mut distances = vec![vec![-1_isize; self.len()]; self.len()];
        let mut seen = std::collections::HashSet::new();

        let (cx, cy) = (self.len() / 2, self.len() / 2);
        let mut queue = VecDeque::new();

        queue.push_back((cx as isize, cy as isize, 0isize));
        distances[cx][cy] = 0;

        while let Some((x, y, dist)) = queue.pop_front() {
            if seen.contains(&(x, y)) {
                continue;
            }
            distances[x as usize][y as usize] = dist;
            seen.insert((x, y));
            // check if neighbors already visited once, skip if so.
            vec![(x, y + 1), (x + 1, y), (x - 1, y), (x, y - 1)]
            .into_iter()
            .filter(|&(a, b)| 0 <= a && a < self.len() as isize && 0 <= b && b < self.len() as isize)
            .filter(|&(a, b)| self[a as usize][b as usize] != b'#')
            .for_each(|(a, b)| {
                if !seen.contains(&(a, b)) && dist <= max_steps as isize - 1 {
                    queue.push_back((a, b, 1 + dist));
                }
            })
        }

        distances
    }
}

pub fn parse_grid(data: &str) -> Grid2D {
    data.lines().map(|s| s.as_bytes().to_vec()).collect::<Vec<_>>()
}


fn debug_grid_distances(grid: &Vec<Vec<isize>>) {
    let grad = magma();
    let max_distance = *grid.iter().map(|row| row.iter().max().unwrap()).max().unwrap();
    
    for row in grid.iter() {
        for (_col, &value) in row.iter().enumerate() {
            let color = {
                if value == -1 {
                    colored::CustomColor { r: 0, g: 0, b: 0 }
                }
                else {
                    let scale = (value + 1) as f64 / (max_distance + 1) as f64;
                    let colorgrad_color = grad.at(scale).to_rgba8();
                    colored::CustomColor { r: colorgrad_color[0], g: colorgrad_color[1], b: colorgrad_color[2] }
                }
            };
            if value == -1 {
                print!("{:02}", "#".custom_color(color));
            } else {
                print!("{:02}", value.to_string().custom_color(color));
            }
        }
        println!();
    }

}


/// Observation: If we reach a tile in an odd (even) number of steps, all routes to that tile take odd (even) number of steps.
/// Any tile reachable at a smaller odd (even) number of steps is also reachable at a larger odd (even) number of steps.
/// Thus, after X steps, only those tiles are reachable that are at taxicab distance at most X and of the same parity as X.
pub fn solve_part1(grid: &Grid2D, steps: usize) -> usize {

    let distances = grid.get_distances(steps);
    debug_grid_distances(&distances);

    distances
    .into_iter()
    .map(
        |row| 
        row
        .iter()
        .filter(|&&v| v >= 0)
        .filter(|&&value| value <= steps as isize && (value as usize % 2 == steps as usize % 2))
        .count()
    )
    .sum()
}


pub fn solve_part2(grid: &Grid2D, steps: usize) -> usize {
    let distances = grid.get_distances(grid.len() / 2);
    debug_grid_distances(&distances);
    0
}


#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2, parse_grid};

    #[test]
    fn part1() {
        let data = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let grid = parse_grid(data);
        assert_eq!(solve_part1(&grid, 6), 16);
        // assert_eq!(solve_part2(data, 6), 16);
        // assert_eq!(solve_part2(data, 10), 50);
        // assert_eq!(solve_part2(data, 50), 1594);
        // assert_eq!(solve_part2(data, 100), 6536);
        // assert_eq!(solve_part2(data, 500), 167004);
        // assert_eq!(solve_part2(data, 1000), 668697);
        // assert_eq!(solve_part2(data, 5000), 16733044);
    }
}