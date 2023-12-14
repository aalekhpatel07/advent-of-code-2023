use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Grid2D(Vec<Vec<u8>>);

impl std::fmt::Display for Grid2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter().map(|row| {
            row.iter()
                .map(|&c| (c as char).to_string())
                .collect::<String>()
        }) {
            _ = writeln!(f, "{row}");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

impl std::fmt::Display for Reflection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Reflection::Horizontal(h) => write!(f, "Horizontal({})", h),
            Reflection::Vertical(v) => write!(f, "Vertical({})", v),
        }
    }
}

impl Reflection {
    pub fn score(&self) -> usize {
        match *self {
            Self::Horizontal(h) => h * 100,
            Self::Vertical(v) => v,
        }
    }
}

impl Grid2D {
    fn is_reflection_line_horizontal(&self, index: isize) -> bool {
        let upwards = (0..index).rev();
        let downwards = index..(self.0.len() as isize);
        upwards
            .zip(downwards)
            .all(|(up, down)| self.0[up as usize] == self.0[down as usize])
    }

    fn is_reflection_line_vertical(&self, index: isize) -> bool {
        let leftwards = (0..index).rev();
        let rightwards = index..(self.0.get(0).unwrap().len() as isize);
        leftwards
            .zip(rightwards)
            .all(|(left, right)| self.get_col(left as usize) == self.get_col(right as usize))
    }

    pub fn find_horizontal_reflection(&self) -> impl Iterator<Item=Reflection> + '_ {
        (1..(self.0.len() as isize))
            .filter(|&index| self.is_reflection_line_horizontal(index))
            .map(|v| Reflection::Horizontal(v as usize))
    }

    fn get_col(&self, index: usize) -> Vec<u8> {
        (0..self.0.len())
            .map(|row_index| self.0[row_index][index])
            .collect()
    }

    pub fn find_vertical_reflection(&self) -> impl Iterator<Item=Reflection> + '_ {
        (1..(self.0.get(0).unwrap().len() as isize))
            .filter(|&index| self.is_reflection_line_vertical(index))
            .map(|v| Reflection::Vertical(v as usize))
    }

    fn correct_smudge(&self, row_index: usize, col_index: usize) -> Self {
        let mut res = Self(self.0.clone());
        let value = res
            .0
            .get_mut(row_index)
            .unwrap()
            .get_mut(col_index)
            .unwrap();
        if *value == b'.' {
            *value = b'#';
        } else {
            *value = b'.';
        }

        res
    }

    pub fn find_lines_of_reflection(&self) -> impl Iterator<Item = Reflection> + '_ {
        self.find_horizontal_reflection()
        .chain(self.find_vertical_reflection())
    }

    pub fn find_new_line_of_reflection(&self) -> Option<Reflection> {
        let old_reflections: std::collections::HashSet<_> =
            self.find_lines_of_reflection().collect();

        let rows = self.0.len();
        let cols = self.0[0].len();

        for row_index in 0..rows {
            for col_index in 0..cols {
                let maybe_corrected = self.correct_smudge(row_index, col_index);
                let new_reflections: std::collections::HashSet<_> =
                    maybe_corrected.find_lines_of_reflection().collect();
                
                if let Some(new_reflection) = new_reflections.difference(&old_reflections).next() {
                    // If there's exactly one smudge that when fixed produces at least
                    // one new line of reflection, then we just found it!
                    return Some(*new_reflection);
                }
            }
        }
        panic!("No smudges found!: {}", self);
    }
}

pub fn main() {
    let data = include_str!("../../data/13.in");

    println!("part 1: {}", solve_part1(data));
    println!("part 2: {}", solve_part2(data));
}

pub fn solve_part1(data: &str) -> usize {
    data.split("\n\n")
        .par_bridge()
        .map(|block| {
            Grid2D(
                block
                    .lines()
                    .map(|row| row.chars().map(|c| (c as u8)).collect())
                    .collect(),
            )
        })
        .map(|grid| grid.find_lines_of_reflection().next())
        .map(|reflection| reflection.map(|r| r.score()).unwrap_or_default())
        .sum()
}

pub fn solve_part2(data: &str) -> usize {
    data.split("\n\n")
        // .par_bridge()
        .map(|block| {
            let grid = Grid2D(
                block
                    .lines()
                    .map(|row| row.chars().map(|c| (c as u8)).collect())
                    .collect(),
            );
            // println!("{}", grid);
            grid
        })
        .map(|grid| grid.find_new_line_of_reflection().unwrap())
        .map(|reflection| reflection.score())
        .sum()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{solve_part1, solve_part2};

    #[test]
    fn expand() {
        let data = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(solve_part1(data), 405);
        assert_eq!(solve_part2(data), 400);
    }

    #[test]

    fn tricky_no_smudges() {
        let data = r"...#..#
...#..#
##..##.
.#.##.#
..#..##
#.#.##.
#.#.###
##.##..
##.##..
#.#.###
..#.##.
..#..##
.#.##.#
##..##.
...#..#";

        let grid = super::Grid2D(
            data
                .lines()
                .map(|row| row.chars().map(|c| (c as u8)).collect())
                .collect()
        );

        assert_eq!(Some(crate::Reflection::Horizontal(8)), grid.find_new_line_of_reflection());

    }

    #[test]
    fn tricky_line_of_reflection() {
        let data = r"...#..#
...#..#
##..##.
.#.##.#
..#..##
#.#.##.
#.#.###
##.##..
##.##..
#.#.###
#.#.##.
..#..##
.#.##.#
##..##.
...#..#";
        
        let grid = super::Grid2D(
            data
                .lines()
                .map(|row| row.chars().map(|c| (c as u8)).collect())
                .collect()
        );

        assert_eq!(vec![crate::Reflection::Horizontal(8), crate::Reflection::Horizontal(1)].into_iter().collect::<HashSet<_>>(), grid.find_lines_of_reflection().collect::<HashSet<_>>());
    }
}
