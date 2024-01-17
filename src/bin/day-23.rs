use std::str::FromStr;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, HashMap, VecDeque};
use colored::{Colorize, ColoredString};
use petgraph::visit::EdgeRef;


fn main() {
    let data = include_str!("../../data/23.in");
    let trails = data.parse::<Trails>().unwrap();
    println!("part 1: {}", solve_part1(&trails));
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trail {
    Path,
    Forest,
    Slope(Direction)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    #[inline(always)]
    pub const fn coord(&self) -> Coord {
        match self {
            Self::Left => (0, -1),
            Self::Down => (1, 0),
            Self::Right => (0, 1),
            Self::Up => (-1, 0)
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Up => "^",
            Self::Down => "v",
            Self::Left => "<",
            Self::Right => ">"
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Trail {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_ref() {
            "#" => Ok(Self::Forest),
            "." => Ok(Self::Path),
            ">" => Ok(Self::Slope(Direction::Right)),
            "<" => Ok(Self::Slope(Direction::Left)),
            "^" => Ok(Self::Slope(Direction::Up)),
            "v" => Ok(Self::Slope(Direction::Down)),
            _ => Err("Unknown Trail".to_string())
        }
    }
}
impl std::fmt::Debug for Trail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Trail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Self::Forest => "#".to_string(),
            Self::Path => ".".to_string(),
            Self::Slope(d) => d.to_string()
        };
        write!(f, "{}", c)
    }
}

impl std::fmt::Display for Trails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            _ = writeln!(f, "{}", row.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(""));
        }
        Ok(())
    }
}

type Coord = (isize, isize);
type HeapEntry = (Reverse<isize>, Coord, Option<Coord>);

impl Trails {
    pub fn neighbors(&self, pos: Coord) -> HashSet<Coord> {
        let (row, col) = (pos.0, pos.1);
        
        let deltas = [
            (-1, 0), 
            (1, 0), 
            (0, -1), 
            (0, 1)
        ];
        let mut res = HashSet::<(isize, isize)>::new();

        if row < 0 || row >= self.0.len() as isize {
            return res;
        }
        if col < 0 || col >= self.0[0].len() as isize {
            return res;
        }
        match self.0[row as usize][col as usize] {
            Trail::Slope(dir) => {
                let delta = dir.coord();
                let (cx, cy) = (row + delta.0, col + delta.1);
                if cx < 0 || cx >= self.0.len() as isize {
                    return res;
                }
                if cy < 0 || cy >= self.0[0].len() as isize {
                    return res;
                }
                if self.0[cx as usize][cy as usize] == Trail::Path {
                    res.insert((cx, cy));
                }
            },
            Trail::Path => {
                for delta in deltas {
                    let (cx, cy) = (row + delta.0, col + delta.1);

                    if cx < 0 || cx >= self.0.len() as isize {
                        continue;
                    }
                    if cy < 0 || cy >= self.0[0].len() as isize {
                        continue;
                    }
                    // println!("{:?}", (cx, cy));
                    if matches!(self.0[cx as usize][cy as usize], Trail::Path | Trail::Slope(_)) {
                        res.insert((cx, cy));
                    }
                }
            },
            _ => {}
        }
        res
    }

    pub fn find_splitters(&self) -> Vec<(Coord, usize)> {

        let mut splitters = vec![];

        self.bfs((0, 1), |node, prev, dist| {
            let mut nbs = self.neighbors(node);
            if let Some(prev) = prev {
                nbs.remove(&prev);
            }
            if nbs.len() >= 2 {
                splitters.push((node, dist));
            }
        });

        splitters
    }

    pub fn debug_splitters(&self, splitters: &[(Coord, usize)]) {

        let splitters = splitters.into_iter().map(|s| (s.0.0, s.0.1)).collect::<HashSet<_>>();
        
        self.show_colored(|coord, _| {
            match splitters.contains(&coord) {
                true => {
                    Some("X".color(colored::Color::Red))
                },
                false => None
            }
        });
    }

    pub fn bfs<F>(&self, start: Coord, mut visit: F) 
    where
        F: FnMut(Coord, Option<Coord>, usize) -> ()
    {
        let mut queue: VecDeque<(Coord, Option<Coord>, usize)> = VecDeque::new();
        queue.push_back((start, None, 0));
        let mut visited : HashSet<Coord> = HashSet::new();

        while let Some((curr, prev, dist)) = queue.pop_front() {
            visit(curr, prev, dist);
            visited.insert(curr);

            for neighbor in self.neighbors(curr) {
                if !visited.contains(&neighbor) {
                    queue.push_back((neighbor, Some(curr), dist + 1));
                }
            }
        }
    }

    pub fn max_bfs<F, E>(&self, start: Coord, mut visit: F, reached_end: E) 
    where
        F: FnMut(Coord, Option<Coord>, usize) -> (),
        E: Fn(Coord) -> bool
    {
        let mut queue: BinaryHeap<(Reverse<isize>, Coord, Option<Coord>)> = BinaryHeap::new();
        queue.push((Reverse(0), start, None));
        let mut visited : HashSet<Coord> = HashSet::new();

        while let Some((dist, curr, prev)) = queue.pop() {
            visit(curr, prev, (-dist.0) as usize);
            visited.insert(curr);
            if reached_end(curr) {
                return;
            }

            for neighbor in self.neighbors(curr) {
                if !visited.contains(&neighbor) {
                    queue.push((Reverse(dist.0 - 1), neighbor, Some(curr)));
                }
            }
        }
    }

    pub fn longest_path(&self, start: Coord, end: Coord) -> Option<Vec<(usize, usize, usize)>> {

        let mut predecessors = HashMap::new();

        self.max_bfs(
            start, 
            |curr, prev, _dist| {
                predecessors.insert(curr, prev);
            }, 
            |e| e == end
        );
        Self::read_path_from_predecessors(
            &predecessors,
            start, 
            end
        )
    }

    fn read_path_from_predecessors(
        predecessors: &HashMap<Coord, Option<Coord>>, 
        start: Coord, 
        end: Coord
    ) -> Option<Vec<(usize, usize, usize)>> {
        if start == end {
            return Some(vec![(start.0 as usize, start.1 as usize, 0)]);
        }
        let Some(mut prev) = predecessors.get(&end) else {
            return None;
        };
        let mut res = vec![end];

        loop {
            if prev == &Some(start) {
                res.push(start);
                break;
            }
            let Some(prev_coord) = prev else {
                break;
            };

            res.push(*prev_coord);
            if predecessors.get(prev_coord).is_none() {
                return None;
            }
            prev = predecessors.get(prev_coord).unwrap();

        }

        res.reverse();
        Some(res.into_iter().enumerate().map(|(idx, v)| (v.0 as usize, v.1 as usize, idx)).collect())
    }

    pub fn show_colored<F>(&self, get_color: F)
    where
        F: Fn(Coord, &Trail) -> Option<ColoredString>
    {
        let mut res = vec![vec!["".to_string(); self.0[0].len()]; self.0.len()];

        for (row_idx, row) in self.0.iter().enumerate() {
            for (col_idx, entry) in row.iter().enumerate() {
                match get_color((row_idx as isize, col_idx as isize), entry) {
                    Some(colored_string) => {
                        res[row_idx][col_idx] = colored_string.to_string();
                    },
                    None => {
                        res[row_idx][col_idx] = entry.to_string();
                    }
                }
            }
            println!("{}", res[row_idx].iter().map(|s| s.to_string()).collect::<Vec<_>>().join(""));
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Trails(Vec<Vec<Trail>>);

impl AsRef<Vec<Vec<Trail>>> for Trails {
    fn as_ref(&self) -> &Vec<Vec<Trail>> {
        &self.0
    }
}

impl FromStr for Trails {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut island = vec![];
        for line in s.lines() {
            let mut row = vec![];
            for entry in line.chars() {
                row.push(entry.to_string().parse::<Trail>().unwrap());
            }
            island.push(row);
        }
        Ok(Self(island))
    }
}


pub fn solve_part1(trails: &Trails) -> usize {
    let start: Coord = (0, 1);
    let end: Coord = (trails.0.len() as isize - 1, trails.0[0].len() as isize - 2);
    let mut splitters = trails.find_splitters();

    splitters.insert(0, (start, 0));
    splitters.push((end, 0));


    let vertices = 
        splitters
        .iter()
        .map(|&(coord, _)| coord)
        .collect::<Vec<_>>();

    let mut distances: HashMap<Coord, HashMap<Coord, usize>> = HashMap::new();

    for &v in vertices.iter() {
        for &u in vertices.iter() {
            if u == v {
                continue;
            }

            let dist = {
                match trails.longest_path(v, u) {
                    Some(path) => path.len() - 1,
                    None => usize::MAX
                }
            };

            distances
            .entry(v)
            .and_modify(|d| {
                d.insert(u, dist);
            })
            .or_insert_with(|| {
                let mut h = HashMap::new();
                h.insert(u, dist);
                h
            });
        }
    }

    let mut edges = vec![];
    let mut nodes = HashSet::new();

    for (&source, target_distances) in distances.iter() {
        for (&target, &dist) in target_distances.iter() {
            nodes.insert(source);
            nodes.insert(target);
            if dist != usize::MAX {
                edges.push((source, target, dist));
            }
        }
    }

    let mut graph = petgraph::graph::Graph::<(isize, isize), usize>::new();

    let mut node_ids = HashMap::new();

    for node in nodes {
        let node_id = graph.add_node(node);
        node_ids.insert(node, node_id);
    }

    for (source, target, dist) in edges {
        graph.add_edge(*node_ids.get(&source).unwrap(), *node_ids.get(&target).unwrap(), dist);

    }

    let ordered = petgraph::algo::toposort(&graph, None).unwrap();

    let mut distances_final = HashMap::new();
    for &node_id in node_ids.values() {
        distances_final.insert(node_id, i64::MIN);
    }

    distances_final.insert(*node_ids.get(&(0, 1)).unwrap(), 0);

    for &node_id in ordered.iter() {
        for edge in graph.edges(node_id) {
            let best_so_far = *distances_final.get(&edge.target()).unwrap();
            let candidate = *distances_final.get(&node_id).unwrap() + *edge.weight() as i64;
            if candidate > best_so_far {
                *distances_final.get_mut(&edge.target()).unwrap() = candidate;
            }
        }
    }

    *distances_final.get(node_ids.get(&end).unwrap()).unwrap() as usize
}


pub fn solve_part2(data: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors() {
        let data = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

        let trails = data.parse::<Trails>().unwrap();
        // let path = trails.shortest_path(
        //     (0, 1), 
        //     (trails.0.len() as isize - 1, trails.0[0].len() as isize - 2)
        // );
        // trails.debug_path(&path);

        // let splitters = trails.find_splitters();
        // trails.debug_splitters(&splitters);
        assert_eq!(solve_part1(&trails), 94);

    }
}