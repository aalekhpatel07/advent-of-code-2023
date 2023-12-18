use aoc_2023::data_structures::{Direction, NeighborhoodShape, SparseGrid2D};
use rayon::prelude::*;

use pathfinding::prelude::astar;

pub fn main() {
    let data = include_str!("../../data/17.in");
    println!("part 1: {}", solve_part1(data));
    println!("part 2: {}", solve_part2(data));
}

pub fn solve_part1(data: &str) -> isize {
    let grid = data.parse::<SparseGrid2D<HeatLoss>>().unwrap();

    let start_node = NodeKey::new((0, 0), None, 0);
    let end_node = NodeKey::new(
        (grid.rows as isize - 1, grid.columns as isize - 1),
        Some(Direction::Right),
        0,
    );

    Direction::all_from_shape(NeighborhoodShape::Plus)
        .into_par_iter()
        .map(|direction| {
            astar(
                &start_node,
                |node_key| {
                    grid.max_consecutive_run_neighbors(*node_key)
                        .into_iter()
                        .map(|key| (key, grid.at(key.coord).map(|h| h.0).unwrap()))
                        .collect::<Vec<_>>()
                },
                |node_key| node_key.l1_distance(end_node),
                |node_key| {
                    node_key.coord == end_node.coord && node_key.direction == Some(direction)
                },
            )
            .map(|(_, dist)| dist as isize)
            .unwrap_or(isize::MAX)
        })
        .min()
        .unwrap()
}

pub fn solve_part2(data: &str) -> isize {
    let grid = data.parse::<SparseGrid2D<HeatLoss>>().unwrap();

    let start_node = NodeKey::new((0, 0), None, 0);
    let end_node = NodeKey::new(
        (grid.rows as isize - 1, grid.columns as isize - 1),
        Some(Direction::Right),
        0,
    );

    Direction::all_from_shape(NeighborhoodShape::Plus)
        .into_par_iter()
        .map(|direction| {
            astar(
                &start_node,
                |node_key| {
                    grid.min_max_consecutive_run_neighbors(*node_key)
                        .into_iter()
                        .map(|key| (key, grid.at(key.coord).map(|h| h.0).unwrap()))
                        .collect::<Vec<_>>()
                },
                |node_key| node_key.l1_distance(end_node),
                |node_key| {
                    node_key.coord == end_node.coord
                        && node_key.direction == Some(direction)
                        && node_key.run_length >= 4
                },
            )
            .map(|(_, dist)| dist as isize)
            .unwrap_or(isize::MAX)
        })
        .min()
        .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeKey {
    coord: (isize, isize),
    direction: Option<Direction>,
    run_length: usize,
}

impl NodeKey {
    pub fn new(coord: (isize, isize), direction: Option<Direction>, run_length: usize) -> NodeKey {
        NodeKey {
            coord,
            direction,
            run_length,
        }
    }
    pub fn l1_distance(&self, other: NodeKey) -> usize {
        self.coord.0.abs_diff(other.coord.0) + self.coord.1.abs_diff(other.coord.1)
    }
}

pub trait MaxConsecutiveRunNeighbors {
    fn max_consecutive_run_neighbors(&self, key: NodeKey) -> Vec<NodeKey>;
    fn min_max_consecutive_run_neighbors(&self, key: NodeKey) -> Vec<NodeKey>;
}

impl MaxConsecutiveRunNeighbors for SparseGrid2D<HeatLoss> {
    fn max_consecutive_run_neighbors(&self, key: NodeKey) -> Vec<NodeKey> {
        let NodeKey {
            coord,
            direction,
            run_length,
        } = key;

        let Some(direction) = direction else {
            return
                self
                .neighbors((coord.0 as usize, coord.1 as usize), NeighborhoodShape::Plus)
                .map(|(coord, direction, _)| {
                    NodeKey {
                        coord: (coord.0 as isize, coord.1 as isize),
                        direction: Some(direction),
                        run_length: 1
                    }
                })
                .collect()
        };

        let mut result = vec![];

        for (target_coord, target_direction, _) in self.neighbors(
            (coord.0 as usize, coord.1 as usize),
            NeighborhoodShape::Plus,
        ) {
            if target_direction == direction {
                if run_length >= 3 {
                    continue;
                } else {
                    result.push(NodeKey {
                        coord: (target_coord.0 as isize, target_coord.1 as isize),
                        direction: Some(target_direction),
                        run_length: run_length + 1,
                    });
                }
            } else {
                // can't go back.
                if target_direction == direction.opposite().unwrap() {
                    continue;
                }
                result.push(NodeKey {
                    coord: (target_coord.0 as isize, target_coord.1 as isize),
                    direction: Some(target_direction),
                    run_length: 1,
                });
            }
        }

        result
    }

    fn min_max_consecutive_run_neighbors(&self, key: NodeKey) -> Vec<NodeKey> {
        let NodeKey {
            coord,
            direction,
            run_length,
        } = key;

        let Some(direction) = direction else {
            return
                self
                .neighbors((coord.0 as usize, coord.1 as usize), NeighborhoodShape::Plus)
                .map(|(coord, direction, _)| {
                    NodeKey {
                        coord: (coord.0 as isize, coord.1 as isize),
                        direction: Some(direction),
                        run_length: 1
                    }
                })
                .collect()
        };

        let mut result = vec![];

        for (target_coord, target_direction, _) in self.neighbors(
            (coord.0 as usize, coord.1 as usize),
            NeighborhoodShape::Plus,
        ) {
            if target_direction == direction {
                // going in the same direction,
                // must keep going if less than 4 blocks travelled.
                if run_length < 4 {
                    result.push(NodeKey {
                        coord: (target_coord.0 as isize, target_coord.1 as isize),
                        direction: Some(target_direction),
                        run_length: run_length + 1,
                    });
                    continue;
                }
                // cannot go further if max blocks traversed.
                if run_length >= 10 {
                    continue;
                }

                result.push(NodeKey {
                    coord: (target_coord.0 as isize, target_coord.1 as isize),
                    direction: Some(target_direction),
                    run_length: run_length + 1,
                });
            } else {
                // can't go back or turn.
                if run_length < 4 || target_direction == direction.opposite().unwrap() {
                    continue;
                }
                result.push(NodeKey {
                    coord: (target_coord.0 as isize, target_coord.1 as isize),
                    direction: Some(target_direction),
                    run_length: 1,
                });
            }
        }

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeatLoss(usize);

impl TryFrom<char> for HeatLoss {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.is_ascii_digit() {
            true => Ok(Self(value.to_digit(10).unwrap() as usize)),
            false => Err("found non-digit".to_string()),
        }
    }
}

impl std::fmt::Display for HeatLoss {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ultra_2() {
        let data = r"111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(71, solve_part2(data));
        assert_eq!(59, solve_part1(data));
    }

    #[test]
    fn smol() {
        let data = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(102, solve_part1(data));
        assert_eq!(94, solve_part2(data));
    }
}
