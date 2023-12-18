use aoc_2023::data_structures::{SparseGrid2D, NeighborhoodShape};
use std::{collections::{BinaryHeap, HashMap, HashSet}, cmp::Ordering};


pub fn main() {
    let data = include_str!("../../data/17.in");
    println!("part 1: {}", solve_part1(data));
    // println!("part 2: {}", solve_part2(data));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down
}

impl From<Direction> for (isize, isize) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => (-1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
        }
    }
}

impl From<(isize, isize)> for Direction { 
    fn from(value: (isize, isize)) -> Self {
        match value {
            (0, -1) => Direction::Left,
            (0, 1) => Direction::Right,
            (1, 0) => Direction::Down,
            _ => panic!("invalid direction")
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord)]
pub struct Cost(isize);

impl Cost {
    pub fn value(&self) -> usize {
        -self.0 as usize
    }
}

impl PartialOrd<Cost> for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (-self.0).partial_cmp(&(-other.0))
    }
}

impl std::ops::Add for Cost {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}


// #[derive(Debug, Clone)]
// pub struct Neighbor<'grid> {
//     grid: &'grid SparseGrid2D<HeatLoss>,
//     towards: Option<Direction>,
//     consecutive_steps: usize,
//     last_direction_index: usize,
//     source: (isize, isize)
// }


// #[derive(Debug)]
// pub struct NeighborIter<'nbr> {
//     neighbor: &'nbr mut Neighbor<'nbr>,
//     source: (isize, isize),
//     last_direction_index: usize
// }


// impl<'grid> Neighbor<'grid> {
//     pub fn neighbors(&mut self, source: (isize, isize)) -> NeighborIter<'_> {
//         NeighborIter {
//             neighbor: self,
//             source,
//             last_direction_index: 0
//         }
//     }
// }

// impl<'grid> Iterator for Neighbor<'grid> {
//     type Item = (isize, isize);
//     fn next(&mut self) -> Option<Self::Item> {

//         match self.last_direction_index {
//             0 => {
//                 // return the left neighbor if we can go that way.
//                 // can we go left?
//                 let direction_delta: (isize, isize) = Direction::Left.into();
//                 let target = (direction_delta.0 + self.source.0, direction_delta.1 + self.source.1);
//                 // Neighbor exists.
//                 if let Some(val) = self.grid.at(target) {
//                     // can we actually go left given the max limit of consecutive steps?
//                     if let Some(already_going_in_direction) = self.towards {
//                         if self.consecutive_steps < 4 {
//                             self.consecutive_steps += 1;
//                         }
//                     }
//                 }
//                 self.last_direction_index += 1;
//             },
//             1 => {
//                 // return the bottom neighbor if we can go that way.
//                 self.last_direction_index += 1;
//             },
//             2 => {
//                 // return the right neighbor if we can go that way.
//                 self.last_direction_index += 1;
//             },
//             _ => {
//                 return None
//             }
//         }


//         None
//         // [Direction::Left, Direction::Down, Direction::Right]
//         // .into_iter()
//         // .map(|direction| {
//         //     let (delta_row, delta_col) = direction.into();
//         //     (direction, (delta_row + self.source.0, delta_col + self.source.1))
//         // })
//         // .filter_map(|(direction, (target_row, target_col))| {
//         //     if let Some(current_dir) = self.towards {
//         //         if current_dir == *direction {
//         //             if self.consecutive_steps >= 4 {
//         //                 return None;
//         //             } else {

//         //             }
//         //         }
//         //     }
//         //     // self.grid.at()
//         // })
//     }
// }


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeKey {
    coord: (isize, isize),
    direction: Option<Direction>,
    run_length: usize
}

impl std::hash::Hash for NodeKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coord.hash(state);
        self.direction.hash(state);
    }
}


impl NodeKey {
    pub fn new(coord: (isize, isize), direction: Option<Direction>, run_length: usize) -> NodeKey {
        NodeKey {
            coord,
            direction,
            run_length
        }
    }
}


pub fn djikstra(grid: &SparseGrid2D<HeatLoss>, source: (isize, isize), end: (isize, isize)) -> isize {
    let mut distances: HashMap<(isize, isize), isize> = HashMap::new();
    let mut visited = HashSet::<NodeKey>::new();

    let mut queue = std::collections::BinaryHeap::new();
    queue.push((0, NodeKey::new(source, None, 0)));

    while let Some((distance, nodekey)) = queue.pop() {
        visited.insert(nodekey);
        let d = distances.entry(nodekey.coord).or_insert(isize::MAX);
        *d = (*d).min(-distance);

        if nodekey.coord == end {
            break;
        }

        for ((neighbor_row, neighbor_col), _) in 
            grid.neighbors(
                (nodekey.coord.0 as usize, nodekey.coord.1 as usize), 
                NeighborhoodShape::Plus
            )
        {
            // if above neighbor:
            if (neighbor_row as isize + 1, neighbor_col as isize) == (nodekey.coord.0, nodekey.coord.1) {
                // Could we have gone up?
                let (could_go, run_length) = match nodekey.direction {
                    Some(current_direction) => {
                        if current_direction == Direction::Up {
                            (nodekey.run_length < 3, nodekey.run_length + 1)
                        } else {
                            (true, 1)
                        }
                    },
                    _ => (true, 1)
                };
                if could_go {
                    let neighbor_nodekey = NodeKey::new((neighbor_row as isize, neighbor_col as isize), Some(Direction::Up), run_length);

                    let cost = *grid.at((neighbor_row as isize, neighbor_col as isize)).unwrap();
                    let alternate_cost = (cost.0 as isize).saturating_add(-distance);
                    if !visited.contains(&neighbor_nodekey) && alternate_cost < *distances.entry((neighbor_row as isize, neighbor_col as isize)).or_insert(isize::MAX) {
                        queue.push((-alternate_cost, neighbor_nodekey));
                    }
                }
            }

            // if bottom neighbor:
            if (neighbor_row as isize, neighbor_col as isize) == (nodekey.coord.0 + 1, nodekey.coord.1) {
                // Could we have gone bottom?
                let (could_go, run_length) = match nodekey.direction {
                    Some(current_direction) => {
                        if current_direction == Direction::Down {
                            (nodekey.run_length < 3, nodekey.run_length + 1)
                        } else {
                            (true, 1)
                        }
                    },
                    _ => (true, 1)
                };
                if could_go {
                    let neighbor_nodekey = NodeKey::new((neighbor_row as isize, neighbor_col as isize), Some(Direction::Down), run_length);

                    let cost = *grid.at((neighbor_row as isize, neighbor_col as isize)).unwrap();
                    let alternate_cost = (cost.0 as isize).saturating_add(-distance);
                    if !visited.contains(&neighbor_nodekey) && alternate_cost < *distances.entry((neighbor_row as isize, neighbor_col as isize)).or_insert(isize::MAX) {
                        queue.push((-alternate_cost, neighbor_nodekey));
                    }
                }
            }

            // if right neighbor:
            if (neighbor_row as isize, neighbor_col as isize) == (nodekey.coord.0, nodekey.coord.1 + 1) {
                // Could we have gone right?
                let (could_go, run_length) = match nodekey.direction {
                    Some(current_direction) => {
                        if current_direction == Direction::Right {
                            (nodekey.run_length < 3, nodekey.run_length + 1)
                        } else {
                            (true, 1)
                        }
                    },
                    _ => (true, 1)
                };
                if could_go {
                    let neighbor_nodekey = NodeKey::new((neighbor_row as isize, neighbor_col as isize), Some(Direction::Right), run_length);

                    let cost = *grid.at((neighbor_row as isize, neighbor_col as isize)).unwrap();
                    let alternate_cost = (cost.0 as isize).saturating_add(-distance);
                    if !visited.contains(&neighbor_nodekey) && alternate_cost < *distances.entry((neighbor_row as isize, neighbor_col as isize)).or_insert(isize::MAX) {
                        queue.push((-alternate_cost, neighbor_nodekey));
                    }
                }
            }


            // if left neighbor:
            if (neighbor_row as isize, neighbor_col as isize + 1) == (nodekey.coord.0, nodekey.coord.1) {
                // Could we have gone right?
                let (could_go, run_length) = match nodekey.direction {
                    Some(current_direction) => {
                        if current_direction == Direction::Left {
                            (nodekey.run_length < 3, nodekey.run_length + 1)
                        } else {
                            (true, 1)
                        }
                    },
                    _ => (true, 1)
                };
                if could_go {
                    let neighbor_nodekey = NodeKey::new((neighbor_row as isize, neighbor_col as isize), Some(Direction::Left), run_length);

                    let cost = *grid.at((neighbor_row as isize, neighbor_col as isize)).unwrap();
                    let alternate_cost = (cost.0 as isize).saturating_add(-distance);
                    if !visited.contains(&neighbor_nodekey) && alternate_cost < *distances.entry((neighbor_row as isize, neighbor_col as isize)).or_insert(isize::MAX) {
                        queue.push((-alternate_cost, neighbor_nodekey));
                    }
                }
            }

        }

    }   

    *distances.get(&(grid.rows as isize - 1, grid.columns as isize - 1)).unwrap()
}


pub fn solve_part1(data: &str) -> isize {
    let grid = data.parse::<SparseGrid2D<HeatLoss>>().unwrap();
    djikstra(&grid, (0, 0), (grid.rows as isize - 1, grid.columns as isize - 1))
}


pub fn solve_part2(data: &str) -> usize {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeatLoss(usize);

impl TryFrom<char> for HeatLoss {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.is_ascii_digit() {
            true => Ok(Self(value.to_digit(10).unwrap() as usize)),
            false => Err(format!("found non-digit"))
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

        // let grid = data.parse::<SparseGrid2D<HeatLoss>>().unwrap();
        // let foo: Vec<_> = grid.neighbors((1, 1), NeighborhoodShape::Other(vec![(0, -1), (0, 1), (1, 0)])).collect();
        // println!("{}", grid);
        // println!("{:?}", foo);
        assert_eq!(102, solve_part1(data));
    }
}