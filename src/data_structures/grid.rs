use std::{collections::HashMap, str::FromStr};
use thiserror::Error;

pub type Point = (usize, usize);

#[derive(Debug, Clone, Error)]
pub enum Grid2DParseError {
    #[error("Failed to parse into a 2d grid")]
    BadData
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
    Skip
}

impl<T> From<char> for Item<T> 
where
    T: TryFrom<char>
{
    fn from(value: char) -> Self {
        match value.try_into() {
            Ok(value) => {
                Item::Keep(value)
            },
            Err(_) => {
                Item::Skip
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum NeighborhoodShape {
    #[default]
    Plus,
    Box,
    Other(Vec<(isize, isize)>)
}

impl<T> std::fmt::Display for SparseGrid2D<T> 
where
    T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for row_idx in 0..self.rows {
            for col_idx in 0..self.columns {
                if let Some(value) = self.inner.get(&(row_idx, col_idx)) {
                    _ = write!(f, "{}", value);
                } else {
                    _ = write!(f, "{}", ".")
                }
            }
            _ = writeln!(f, "");
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

        s
        .lines()
        .enumerate()
        .for_each(|(row_idx, row)| {
            num_rows += 1;
            let mut current_columns = 0;
            row
            .chars()
            .enumerate()
            .for_each(|(col_idx, val)| {
                current_columns += 1;

                match val.try_into() {
                    Ok(Item::Keep(value)) => {
                        inner.insert((row_idx, col_idx), value);
                    },
                    Ok(Item::Skip) => {},
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

impl<T> SparseGrid2D<T>
{
    pub fn at(&self, coordinate: (isize, isize)) -> Option<&T> {

        if coordinate.0 < 0 || coordinate.0 >= self.rows as isize {
            return None;
        }
        if coordinate.1 < 0 || coordinate.1 >= self.columns as isize {
            return None;
        }
        self.inner.get(&(coordinate.0 as usize, coordinate.1 as usize))
    }

    pub fn neighbors(&self, coordinate: Point, shape: NeighborhoodShape) -> impl Iterator<Item=(Point, Option<&T>)> {
        let directions_to_check = match shape {
            NeighborhoodShape::Box => {
                vec![
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1)
                ]
            },
            NeighborhoodShape::Plus => {
                vec![
                    (-1, 0),
                    (0, -1),
                    (0, 1),
                    (1, 0),
                ]
            },
            NeighborhoodShape::Other(other) => other
        };

        directions_to_check
        .into_iter()
        .filter_map(move |(delta_row, delta_column)| {
            let target_coordinate = (coordinate.0 as isize + delta_row, coordinate.1 as isize + delta_column);
            if target_coordinate.0 < 0 || target_coordinate.0 >= self.rows as isize {
                return None;
            }
            if target_coordinate.1 < 0 || target_coordinate.1 >= self.columns as isize {
                return None;
            }
            Some((
                (target_coordinate.0 as usize, target_coordinate.1 as usize),
                self.inner.get(&(target_coordinate.0 as usize, target_coordinate.1 as usize))
            ))
        })
    }
}