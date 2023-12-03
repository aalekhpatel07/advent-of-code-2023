use std::collections::HashSet;
use regex::Regex;


fn main() {
    let data = include_str!("../../data/03.in");

    let res1 = part1::solve_part1(&data);
    let res2 = part2::solve_part2(&data);

    println!("part 1: {res1}");
    println!("part 2: {res2}");
}

#[derive(Debug, Clone)]
pub struct Grid(Vec<String>);


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub struct Position(isize, isize);


impl Grid {
    pub fn new(data: &[&str]) -> Self {
        let mut result = vec![];
        for line in data {
            result.push(line.chars().collect::<String>());
        }
        Self(result)
    }

    /// Extract the number blocks from the grid.
    pub fn scan_numbers(&self) -> Vec<NumberIsland> {
        let mut results = vec![];
        let digits = Regex::new(r"(\d+)").unwrap();

        for (row_idx, row) in self.0.iter().enumerate() {
            for num in digits.find_iter(row) {
                let col_start = num.start() as isize;
                let col_end = num.end() as isize;
                let value = num.as_str().parse::<usize>().unwrap();

                results.push(
                    NumberIsland(
                        value, 
                        Position(row_idx as isize, col_start), 
                        Position(row_idx as isize, col_end)
                    )
                );
            }
        }

        results
    }

    /// Get the positions of all the non-digit and non-period symbols on the grid.
    pub fn get_symbols(&self) -> Vec<Position> {
        let mut results = vec![];
        let symbols = Regex::new(r"[^\d.]").unwrap();

        for (row_idx, row) in self.0.iter().enumerate() {
            for found in symbols.find_iter(row) {
                results.push(
                    Position(row_idx as isize, found.start() as isize)
                );
            }
        }
        results
    }
}

/// A horizontal block on the grid that contains digits that forms
/// a number. It contains the value of the number, the starting position, 
/// and the ending position of the run.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct NumberIsland(usize, Position, Position);


impl NumberIsland {

    /// Return whether the given position is valid and exists in a grid of (num_rows, num_cols).
    pub fn is_valid(&self, position: Position, num_rows: usize, num_cols: usize) -> bool {
        let (row, col) = (position.0, position.1);
        0 <= row && row < num_rows as isize && 0 <= col && col < num_cols as isize
    }

    /// Given a run of digits on the grid, 
    /// get the indices on the border of the enclosing box.
    /// 
    /// |--------|
    /// |12345678|
    /// |--------|
    pub fn neighboring_indices(&self, num_rows: usize, num_cols: usize) -> HashSet<Position> {

        let mut results = HashSet::new();

        let current_row = self.1.0;
        let (col_min, col_max) = (self.1.1, self.2.1 - 1);

        for col_idx in col_min..=col_max {
            if self.is_valid(Position(current_row - 1, col_idx), num_rows, num_cols) {
                results.insert(Position(current_row - 1, col_idx));
            }
            if self.is_valid(Position(current_row + 1, col_idx), num_rows, num_cols) {
                results.insert(Position(current_row + 1, col_idx));
            }
        }

        let left_strip = vec![
            Position(current_row - 1, self.1.1 - 1),
            Position(current_row, self.1.1 - 1),
            Position(current_row + 1, self.1.1 - 1),
        ];
        let right_strip = vec![
            Position(current_row - 1, self.2.1),
            Position(current_row, self.2.1),
            Position(current_row + 1, self.2.1),
        ];
        for position in left_strip {
            if self.is_valid(position, num_rows, num_cols) {
                results.insert(position);
            }
        }
        for position in right_strip {
            if self.is_valid(position, num_rows, num_cols) {
                results.insert(position);
            }
        }

        results
    }

    /// Return whether a given position 
    /// lies on the enclosing box of the 
    /// number block.
    pub fn is_adjacent_to(&self, position: Position, num_rows: usize, num_cols: usize) -> bool {
        let (pos_row, pos_col) = (position.0, position.1);
        self.neighboring_indices(num_rows, num_cols).contains(&Position(pos_row, pos_col))
    }
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_smol_data() {
        let data = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(part1::solve_part1(&data), 4361);
        assert_eq!(part2::solve_part2(&data), 467835);
    }
}


mod part1 {
    use super::Grid;

    pub fn solve_part1(data: &str) -> usize {
        let grid = Grid::new(&data.lines().collect::<Vec<_>>());
        let numbers = grid.scan_numbers();

        let num_rows = grid.0.len();
        let num_cols = grid.0[0].len();

        numbers
        .iter()
        .filter_map(|island| {
            match grid
                .get_symbols()
                .iter()
                .any(|symbol| {
                    island.is_adjacent_to(*symbol, num_rows, num_cols)
                }) 
            {
                true => Some(island.0),
                false => None
            }
        })
        .sum()
    }
}

mod part2 {
    use std::collections::HashSet;
    use super::Grid;

    pub fn solve_part2(data: &str) -> usize {
        let grid = Grid::new(&data.lines().collect::<Vec<_>>());
        let numbers = grid.scan_numbers();

        let num_rows = grid.0.len();
        let num_cols = grid.0[0].len();
        let mut sum = 0;

        for symbol_pos in grid.get_symbols() {
            let mut adjacent_islands = HashSet::new();

            for island in numbers.iter() {
                if island.is_adjacent_to(symbol_pos, num_rows, num_cols) {
                    adjacent_islands.insert(island);
                }
                if adjacent_islands.len() > 2 {
                    break;
                }
            }

            if adjacent_islands.len() == 2 {
                sum += adjacent_islands.iter().map(|island| island.0).product::<usize>();
            }
        }
        sum
    }
}
