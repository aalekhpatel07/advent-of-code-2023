pub fn main() {
    let data = include_str!("../../data/11.in");

    println!("part 1: {}", solve_part1(data));
    println!("part 2: {}", solve_part2(data));
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grid(Vec<Vec<char>>);
pub type Coord = (isize, isize);

impl Grid {
    pub fn new(data: &str) -> Self {
        Self(data.lines().map(|line| line.chars().collect()).collect())
    }

    fn empty_rows(&self) -> std::collections::HashSet<usize> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(row_idx, row)| {
                if row.iter().all(|&c| c == '.') {
                    Some(row_idx)
                } else {
                    None
                }
            })
            .collect()
    }

    fn empty_columns(&self) -> std::collections::HashSet<usize> {
        let mut hset = std::collections::HashSet::new();

        let num_rows = self.0.len();
        let num_cols = self.0.get(0).unwrap().len();

        for col_idx in 0..num_cols {
            let mut is_empty = true;
            for row_idx in 0..num_rows {
                if *self.0.get(row_idx).unwrap().get(col_idx).unwrap() != '.' {
                    is_empty = false;
                    break;
                }
            }
            if is_empty {
                hset.insert(col_idx);
            }
        }
        hset
    }

    fn coordinates_of_galaxies(&self) -> Vec<Coord> {
        let mut coords = vec![];

        self.0.iter().enumerate().for_each(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &char)| char == '#')
                .for_each(|(col_idx, _)| {
                    coords.push((row_idx as isize, col_idx as isize));
                })
        });

        coords
    }

    fn pairwise_distances(&self, scale: usize) -> Vec<usize> {
        let coordinates = self.coordinates_of_galaxies();

        let empty_columns = self.empty_columns();
        let empty_rows = self.empty_rows();

        let mut distances = vec![];

        for i in 0..coordinates.len() {
            for j in i + 1..coordinates.len() {
                let start = coordinates.get(i).unwrap();
                let end = coordinates.get(j).unwrap();

                let start_row = start.0.min(end.0) as usize;
                let end_row = start.0.max(end.0) as usize;

                let start_col = start.1.min(end.1) as usize;
                let end_col = start.1.max(end.1) as usize;

                let empty_rows_crossed = empty_rows
                    .iter()
                    .filter(|&row_idx| start_row <= *row_idx && *row_idx <= end_row)
                    .count();

                let empty_columns_crossed = empty_columns
                    .iter()
                    .filter(|&col_idx| start_col <= *col_idx && *col_idx <= end_col)
                    .count();

                let mut distance = start.0.abs_diff(end.0) + start.1.abs_diff(end.1);

                // Number of extra empty columns traversed.
                // + Number of extra empty rows traversed.
                // The original copy is already accounted for
                // so count only the copies.
                distance += (scale - 1) * (empty_rows_crossed + empty_columns_crossed);

                distances.push(distance);
            }
        }
        distances
    }
}

pub fn solve_part1(data: &str) -> usize {
    let grid = Grid::new(data);
    grid.pairwise_distances(2).into_iter().sum()
}

pub fn solve_part2(data: &str) -> usize {
    let grid = Grid::new(data);
    grid.pairwise_distances(1_000_000).into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    #[test]
    fn expand() {
        let data = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(solve_part1(data), 374);
        assert_eq!(solve_part2(data), 82000210);
    }
}
