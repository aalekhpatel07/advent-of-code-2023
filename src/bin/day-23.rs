use std::cmp::Reverse;
use std::str::FromStr;
use std::collections::{HashSet, VecDeque, BinaryHeap};
use colored::ColoredString;
use std::collections::HashMap;


fn main() {
    let data = include_str!("../../data/23.in");
    let trails = data.parse::<Trails>().unwrap();
    println!("part 1: {}", solve_part1(&trails));
}

type Coord = (isize, isize);

pub trait VisitBfs<N> {

    fn on_node_discovered(&mut self, node: N, parent: Option<N>, breadth: usize);
    fn is_goal(&self, node: N) -> bool { false }
}


pub trait VisitDfs<N> {

    fn on_node_started(&mut self, node: N, parent: Option<N>, depth: usize) {}

    /// The entire subtree rooted at this node has been visited at this point.
    fn on_node_finished(&mut self, node: N, depth: usize) {}
    fn on_goal_reached(&mut self, node: N, depth: usize) {

    }
    fn is_goal(&self, node: N) -> bool;
}

#[derive(Debug, Clone)]
pub struct LongestPathIterator<'trails> {
    trails: &'trails Trails,
    start: Coord,
    end: Coord,
    longest_path_length: usize,
}

impl<'trails> LongestPathIterator<'trails> {
    pub fn new(start: Coord, end: Coord, trails: &'trails Trails) -> Self {
        Self {
            trails,
            start,
            end,
            longest_path_length: 0,
        }
    }
}

impl VisitDfs<Coord> for LongestPathIterator<'_> {
    fn is_goal(&self, node: Coord) -> bool {
        node == self.end
    }

    fn on_node_finished(&mut self, node: Coord, depth: usize) {
        
    }
    fn on_goal_reached(&mut self, node: Coord, depth: usize) {
        self.longest_path_length = self.longest_path_length.max(depth);
    }
}


#[derive(Debug, Clone)]
pub struct JunctionMapBuilder<'trails> {
    trails: &'trails Trails,
    start: Coord,
    end: Coord,
    parent_map: HashMap<Coord, Option<Coord>>,
    junctions: HashSet<Coord>
}

pub type CompressedMap = HashMap<Coord, HashMap<Coord, usize>>;

impl<'a> JunctionMapBuilder<'a> {
    pub fn new(start: Coord, end: Coord, trails: &'a Trails) -> Self {
        Self {
            trails,
            start,
            end,
            parent_map: Default::default(),
            junctions: Default::default()
        }
    }

    fn is_junction(&self, pos: Coord) -> bool {
        self.trails.neighbors_part2(pos).len() >= 3
    }

    pub fn compress(&mut self) -> CompressedMap {
        let mut compressed = CompressedMap::new();

        // TODO:
        // Starting at junctions and walking backwards towards the start node,
        // find all the intermediate junctions and track their distances 
        // using the parent_map.
        for &junction in self.junctions.iter() {
            let distances_from_junction = compressed.entry(junction).or_insert(Default::default());

            let mut current = junction;
            let mut dist = 0;
            while current != self.start {
                current = self.parent_map.get(&current).unwrap().unwrap();
                dist += 1;
                if self.is_junction(current) || current == self.start {
                    distances_from_junction.insert(current, dist);
                }
            }
        }

        for (key, val) in compressed.iter() {
            println!("{:?}: {:?}", key, val);
        }
        // Now invert this map to get distances from source to target instead of "to source from target".
        let mut compressed_inv = HashMap::new();

        for (target, value) in compressed.into_iter() {
            for (source, dist) in value.into_iter() {
                compressed_inv
                .entry(source)
                .and_modify(|mapping: &mut HashMap<Coord, usize>| {
                    mapping.entry(target).and_modify(|d| *d = dist).or_insert(dist);
                })
                .or_insert_with(|| {
                    let mut _m = HashMap::new();
                    _m.insert(target, dist);
                    _m
                });
            }
        }
        println!("");
        
        compressed_inv
    }
}


impl VisitBfs<Coord> for JunctionMapBuilder<'_> {

    fn on_node_discovered(&mut self, node: Coord, parent: Option<Coord>, _breadth: usize) {
        self.parent_map.insert(node, parent);

        if self.is_junction(node) || node == self.start || node == self.end {
            self.junctions.insert(node);
        }
    }
}

impl Trails {

    pub fn neighbors_part1(&self, pos: Coord) -> HashSet<Coord> {
        let (row, col) = (pos.0, pos.1);
        let deltas = 
            match self.0[pos.0 as usize][pos.1 as usize] {
                b'>' => {
                    vec![(0, 1)]
                },
                b'<' => {
                    vec![(0, -1)]
                },
                b'^' => {
                    vec![(-1, 0)]
                },
                b'v' => {
                    vec![(1, 0)]
                },
                _ => {
                    vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
                }
            };

        let mut res = HashSet::<(isize, isize)>::new();
        for (dx, dy) in deltas {
            let (cx, cy) = (row + dx, col + dy);
            if 0 <= cx && cx < self.0.len() as isize && 0 <= cy && cy < self.0[0].len() as isize {
                if self.0[cx as usize][cy as usize] != b'#' {
                    res.insert((cx, cy));
                }
            }
        }
        res
    }

    pub fn neighbors_part2(&self, pos: Coord) -> HashSet<Coord> {
        let (row, col) = (pos.0, pos.1);
        let deltas = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut res = HashSet::<(isize, isize)>::new();
        for (dx, dy) in deltas {
            let (cx, cy) = (row + dx, col + dy);
            if 0 <= cx && cx < self.0.len() as isize && 0 <= cy && cy < self.0[0].len() as isize {
                if self.0[cx as usize][cy as usize] != b'#' {
                    res.insert((cx, cy));
                }
            }
        }
        res
    }
}


pub fn bfs<V, N, NeighborFn, I>(
    start: N,
    neighbors: NeighborFn,
    visitor: &mut V,
)
where   
    V: VisitBfs<N>,
    NeighborFn: Fn(N) -> I,
    N: std::hash::Hash + Eq + Copy,
    I: IntoIterator<Item=N>
{
    let mut deque = VecDeque::new();
    deque.push_back((start, None, 0usize));
    let mut seen = HashSet::new();

    while let Some((node, parent, breadth)) = deque.pop_front() {
        seen.insert(node);
        visitor.on_node_discovered(node, parent, breadth);
        if visitor.is_goal(node) {
            break;
        }
        for neighbor in neighbors(node) {
            if !seen.contains(&neighbor) {
                deque.push_back((neighbor, Some(node), breadth + 1));
            }
        }
    }
}

fn _dfs<V, NeighborFn, I>(
    start: Coord, 
    depth: usize,
    neighbors: &NeighborFn,
    visitor: &mut V,
    seen: &mut HashSet<Coord>
) 
where
    V: VisitDfs<Coord>,
    NeighborFn: Fn(Coord) -> I,
    I: IntoIterator<Item=Coord>,
{
    if visitor.is_goal(start) {
        visitor.on_goal_reached(start, depth);
    }

    for neighbor in neighbors(start) {
        if !seen.contains(&neighbor) {
            seen.insert(neighbor);
            _dfs(neighbor, depth + 1, neighbors, visitor, seen);
            seen.remove(&neighbor);
            visitor.on_node_finished(start, depth)
        }
    }
}


pub fn dfs<V, NeighborFn, I>(
    start: Coord, 
    neighbors: NeighborFn,
    visitor: &mut V,
)
where
    V: VisitDfs<Coord>,
    NeighborFn: Fn(Coord) -> I,
    I: IntoIterator<Item=Coord>
{
    let mut seen = HashSet::new();
    _dfs(start, 0, &neighbors, visitor, &mut seen)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Trails(Vec<Vec<u8>>);

impl FromStr for Trails {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut island = vec![];
        for line in s.lines() {
            let mut row = vec![];
            for entry in line.chars() {
                row.push(entry as u8);
            }
            island.push(row);
        }
        Ok(Self(island))
    }
}


pub fn solve_part1(trails: &Trails) -> usize {
    let start: Coord = (0, 1);
    let end: Coord = (trails.0.len() as isize - 1, trails.0[0].len() as isize - 2);
    let mut longest_path_finder = LongestPathIterator::new(start, end, trails);

    dfs(
        start, 
        |node| trails.neighbors_part1(node), 
        &mut longest_path_finder,
    );

    longest_path_finder.longest_path_length
}


pub fn solve_part2(trails: &Trails) -> usize {
    let start: Coord = (0, 1);
    let end: Coord = (trails.0.len() as isize - 1, trails.0[0].len() as isize - 2);

    let mut builder = JunctionMapBuilder::new(start, end, trails);

    bfs(
        start, 
        |node| trails.neighbors_part2(node), 
        &mut builder
    );

    let compressed = builder.compress();

    // println!("{:?}", compressed);

    for (key, value) in compressed.iter() {
        println!("{:?}: {:?}", key, value);
    }


    // let start: Coord = (0, 1);
    // let end: Coord = (trails.0.len() as isize - 1, trails.0[0].len() as isize - 2);

    // let mut junction_finder = JunctionMapBuilder::new(start, end, trails);
    
    // bfs(start, |node| trails.neighbors_part2(node), &mut junction_finder);

    // let compressed = junction_finder.compress();

    // 0
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
        assert_eq!(solve_part1(&trails), 94);
        assert_eq!(solve_part2(&trails), 0);

    }
}