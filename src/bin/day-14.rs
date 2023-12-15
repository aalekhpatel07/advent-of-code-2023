use aoc_2023::math::detect_cycle;

pub fn main() {
    let data = include_str!("../../data/14.in");

    println!("part 1: {}", solve_part1(data));
    println!("part 2: {}", solve_part2(data));
}

pub fn solve_part1(data: &str) -> usize {
    let grid = Grid2D::parse_str(data);
    grid.shift(Direction::North).weight()
}

pub fn solve_part2(data: &str) -> usize {
    let mut grid = Grid2D::parse_str(data);
    let steps = 1_000_000_000;

    let Some((steps_before_reaching_cycle, cycle_length)) = detect_cycle(
        &grid,
        |grid| grid.cycle(),
        Some(steps)
    ) else {
        panic!("No cycle found after checking {} steps!", steps)
    };

    let steps_in_the_last_cycle = (steps - steps_before_reaching_cycle) % cycle_length;

    for _ in 0..(steps_before_reaching_cycle + steps_in_the_last_cycle) {
        grid = grid.cycle();
    }
    grid.weight()
}

#[derive(Clone, Eq)]
pub struct Grid2D(Vec<Vec<Rock>>);

impl std::hash::Hash for Grid2D {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)    
    }
}

impl PartialEq for Grid2D {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}


pub type Coordinate = (usize, usize);

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    East,
    West,
    South
}


impl Grid2D {
    pub fn parse_str(data: &str) -> Self {
        Grid2D(
            data.lines()
                .map(|line| line.chars().map(|value| value.into()).collect())
                .collect(),
        )
    }

    pub fn to_string(&self) -> String {
        self.0
        .iter()
        .map(|row| {
            row
            .iter()
            .map(|rock| rock.to_string())
            .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
    }

    pub fn get_number_of_columns(&self) -> usize {
        match self.0.is_empty() {
            true => 0,
            false => self.0[0].len()
        }
    }

    pub fn state(&self) -> String {
        self.to_string()
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

    pub fn shift(&self, direction: Direction) -> Self {
        
        let num_columns = self.get_number_of_columns();
        let num_rows = self.0.len();

        // Copy over the cube rocks and spaces;
        let mut grid = Grid2D(vec![vec![Rock::Space; num_columns]; num_rows]);
        for (row, col) in self.get_positions_of_cube_rocks() {
            grid.0[row][col] = Rock::Cube;
        }

        let indices = match direction {
            Direction::North | Direction::South => {
                0..num_columns
            },
            Direction::East | Direction::West => {
                0..num_rows
            }
        };

        indices
        .for_each(|index| {
            for transformed in self.shift_single(index, direction) {
                match direction {
                    Direction::North | Direction::South => {
                        grid.0[transformed][index] = Rock::Rounded;
                    },
                    _ => {
                        grid.0[index][transformed] = Rock::Rounded;
                    }
                }
            }
        });

        grid
    }

    fn shift_single(&self, index: usize, direction: Direction) -> Vec<usize> {

        let mut next_rock_spot = match direction {
            Direction::North | Direction::West => 0,
            Direction::South => self.0.len() - 1,
            Direction::East => self.get_number_of_columns() - 1,
        };

        let rock_lanes_to_shift = match direction {
            Direction::North => {
                self
                .get_column(index)
                .enumerate()
                .collect::<Vec<_>>()
            },
            Direction::South => {
                self
                .get_column(index)
                .enumerate()
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
            },
            Direction::East => {
                self.0[index]
                .iter()
                .enumerate()
                .rev()
                .map(|(i, rock)| (i, *rock))
                .collect::<Vec<_>>()
            },
            Direction::West => {
                self.0[index]
                .iter()
                .enumerate()
                .map(|(i, rock)| (i, *rock))
                .collect::<Vec<_>>()
            }
        };

        let mut rocks = vec![];

        rock_lanes_to_shift
        .into_iter()
        .for_each(|(idx, rock)| {
            match rock {
                Rock::Space => {},
                Rock::Cube => {
                    match direction {
                        Direction::North | Direction::West => {
                            next_rock_spot = idx + 1;
                        },
                        Direction::South | Direction::East => {
                            if idx >= 1 {
                                next_rock_spot = idx - 1;
                            }
                        }
                    }
                },
                Rock::Rounded => {
                    rocks.push(next_rock_spot);
                    match direction {
                        Direction::North | Direction::West => {
                            next_rock_spot += 1;
                        },
                        Direction::South | Direction::East => {
                            if next_rock_spot >= 1 {
                                next_rock_spot -= 1;
                            }
                        }
                    }
                }
            }
        });

        rocks
    }

    pub fn get_positions_of_cube_rocks(&self) -> impl Iterator<Item=(usize, usize)> + '_ {
        self.0
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row
            .iter()
            .enumerate()
            .filter_map(move |(col_index, value)| {
                match *value == Rock::Cube {
                    true => Some((row_index, col_index)),
                    false => None
                }
            })
        })
    }

    pub fn cycle(&self) -> Self {
        self
        .shift(Direction::North)
        .shift(Direction::West)
        .shift(Direction::South)
        .shift(Direction::East)
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


#[cfg(test)]
mod tests {

    use super::{solve_part1, solve_part2, Grid2D, Direction};

    #[test]
    fn shift() {
        let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let grid = Grid2D::parse_str(input);

        let south = r".....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O";

        let west = r"O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#....";

        let east = r"....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#....";

        let north = r"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

        assert_eq!(&grid.shift(Direction::North).to_string(), north);
        assert_eq!(&grid.shift(Direction::East).to_string(), east);
        assert_eq!(&grid.shift(Direction::West).to_string(), west);
        assert_eq!(&grid.shift(Direction::South).to_string(), south);

    }

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

        assert_eq!(solve_part1(data), 136);
        assert_eq!(solve_part2(data), 64);
    }

    #[test]
    fn cycle() {
        let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";


        let expected_values = vec![
r".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
        ];

        let mut grid = Grid2D::parse_str(input);
        for expected in expected_values.into_iter() {
            assert_eq!(&grid.cycle().to_string(), expected);
            grid = grid.cycle();
        }

    }
}
