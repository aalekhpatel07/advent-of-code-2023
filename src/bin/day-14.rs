#[derive(Debug, Clone)]
pub struct Grid2D(Vec<Vec<Rock>>);

impl Grid2D {
    pub fn parse_str(data: &str) -> Self {
        Grid2D(
            data.lines()
                .map(|line| line.chars().map(|value| value.into()).collect())
                .collect(),
        )
    }

    pub fn weight(&self) -> usize {
        self.0
            .iter()
            .rev()
            .enumerate()
            .map(|(row_index, row)| {
                row.iter().filter(|&value| *value == Rock::Rounded).count() * (row_index + 1)
            })
            .sum()
    }

    fn get_column(&self, index: usize) -> impl Iterator<Item = Rock> + '_ {
        (0..self.0.len()).map(move |row_index| self.0[row_index][index])
    }

    fn shift_column_upward(&mut self, col_index: usize) {
        self.get_column(col_index);
        let _column = vec![Rock::Space; self.0.len()];

        let _next_free_spot = 0;
    }
}

impl std::fmt::Display for Grid2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self
            .0
            .iter()
            .map(|row| row.iter().map(|&c| c.to_string()).collect::<String>())
        {
            _ = writeln!(f, "{row}");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rock {
    Rounded,
    Cube,
    Space,
}

impl From<char> for Rock {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::Rounded,
            '#' => Self::Cube,
            '.' => Self::Space,
            _ => panic!("invalid char"),
        }
    }
}

impl std::fmt::Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Rounded => write!(f, "O"),
            Self::Cube => write!(f, "#"),
            Self::Space => write!(f, "."),
        }
    }
}

pub fn main() {
    let data = include_str!("../../data/13.in");

    println!("part 1: {}", solve_part1(data));
    println!("part 2: {}", solve_part2(data));
}

pub fn solve_part1(_data: &str) -> usize {
    // data.split("\n\n")
    //     .par_bridge()
    //     .map(|block| {
    //         Grid2D(
    //             block
    //                 .lines()
    //                 .map(|row| row.chars().map(|c| (c as u8)).collect())
    //                 .collect(),
    //         )
    //     })
    //     .map(|grid| grid.find_lines_of_reflection().next())
    //     .map(|reflection| reflection.map(|r| r.score()).unwrap_or_default())
    //     .sum()
    0
}

pub fn solve_part2(_data: &str) -> usize {
    0
    //     data.split("\n\n")
    //         .par_bridge()
    //         .map(|block| {
    //             let grid = Grid2D(
    //                 block
    //                     .lines()
    //                     .map(|row| row.chars().map(|c| (c as u8)).collect())
    //                     .collect(),
    //             );
    //             grid
    //         })
    //         .map(|grid| grid.find_new_line_of_reflection().unwrap())
    //         .map(|reflection| reflection.score())
    //         .sum()
}

#[cfg(test)]
mod tests {

    use super::solve_part1;

    #[test]
    fn smol() {
        let data = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        solve_part1(data);
    }
}
