use std::{collections::HashMap, str::FromStr};
use thiserror::Error;

pub type Point = (usize, usize);

#[derive(Debug, Clone, Error)]
pub enum Grid2DParseError {
    #[error("Failed to parse into a 2d grid")]
    BadData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Direction {
    pub fn all_from_shape(shape: NeighborhoodShape) -> Vec<Direction> {
        match shape {
            NeighborhoodShape::Box => vec![
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
                Direction::TopLeft,
                Direction::TopRight,
                Direction::BottomLeft,
                Direction::BottomRight,
            ],
            NeighborhoodShape::Plus => vec![
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
            ],
        }
    }

    pub fn opposite(&self) -> Option<Direction> {
        match self {
            Direction::Left => Some(Direction::Right),
            Direction::Right => Some(Direction::Left),
            Direction::Up => Some(Direction::Down),
            Direction::Down => Some(Direction::Up),
            Direction::TopLeft => Some(Direction::BottomRight),
            Direction::TopRight => Some(Direction::BottomLeft),
            Direction::BottomLeft => Some(Direction::TopRight),
            Direction::BottomRight => Some(Direction::TopLeft),
        }
    }
}

impl From<Direction> for (isize, isize) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => (-1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::TopLeft => (-1, -1),
            Direction::TopRight => (1, -1),
            Direction::BottomLeft => (-1, 1),
            Direction::BottomRight => (1, 1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SparseGrid2D<T> {
    inner: HashMap<Point, T>,
    pub rows: usize,
    pub columns: usize,
}

#[derive(Debug)]
pub enum Item<T> {
    Keep(T),
    Skip,
}

impl<T> From<char> for Item<T>
where
    T: TryFrom<char>,
{
    fn from(value: char) -> Self {
        match value.try_into() {
            Ok(value) => Item::Keep(value),
            Err(_) => Item::Skip,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum NeighborhoodShape {
    #[default]
    Plus,
    Box,
}

impl<T> std::fmt::Display for SparseGrid2D<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row_idx in 0..self.rows {
            for col_idx in 0..self.columns {
                if let Some(value) = self.inner.get(&(row_idx, col_idx)) {
                    _ = write!(f, "{}", value);
                } else {
                    _ = write!(f, ".")
                }
            }
            _ = writeln!(f);
        }
        Ok(())
    }
}

impl<T> FromStr for SparseGrid2D<T>
where
    Item<T>: TryFrom<char>,
{
    type Err = Grid2DParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num_rows = 0;
        let mut num_cols = None;
        let mut inner = std::collections::HashMap::<Point, T>::new();

        s.lines().enumerate().for_each(|(row_idx, row)| {
            num_rows += 1;
            let mut current_columns = 0;
            row.chars().enumerate().for_each(|(col_idx, val)| {
                current_columns += 1;

                match val.try_into() {
                    Ok(Item::Keep(value)) => {
                        inner.insert((row_idx, col_idx), value);
                    }
                    Ok(Item::Skip) => {}
                    _ => {}
                }
            });

            if num_cols.is_none() {
                num_cols = Some(current_columns);
            }
        });

        Ok(Self {
            inner,
            rows: num_rows,
            columns: num_cols.unwrap(),
        })
    }
}

impl<T> SparseGrid2D<T> {
    pub fn at(&self, coordinate: (isize, isize)) -> Option<&T> {
        if coordinate.0 < 0 || coordinate.0 >= self.rows as isize {
            return None;
        }
        if coordinate.1 < 0 || coordinate.1 >= self.columns as isize {
            return None;
        }
        self.inner
            .get(&(coordinate.0 as usize, coordinate.1 as usize))
    }

    pub fn neighbors(
        &self,
        coordinate: Point,
        shape: NeighborhoodShape,
    ) -> impl Iterator<Item = (Point, Direction, Option<&T>)> {
        let directions_to_check = Direction::all_from_shape(shape);

        directions_to_check
            .into_iter()
            .filter_map(move |direction| {
                let (delta_row, delta_column) = direction.into();
                let target_coordinate = (
                    coordinate.0 as isize + delta_row,
                    coordinate.1 as isize + delta_column,
                );
                if target_coordinate.0 < 0 || target_coordinate.0 >= self.rows as isize {
                    return None;
                }
                if target_coordinate.1 < 0 || target_coordinate.1 >= self.columns as isize {
                    return None;
                }
                Some((
                    (target_coordinate.0 as usize, target_coordinate.1 as usize),
                    direction,
                    self.inner
                        .get(&(target_coordinate.0 as usize, target_coordinate.1 as usize)),
                ))
            })
    }
}
