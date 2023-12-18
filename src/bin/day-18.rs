use aoc_2023::data_structures::Polygon;
use regex::Regex;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down
}

impl Direction {
    pub fn delta(&self, scale: isize) -> (isize, isize) {
        match *self {
            Direction::Left => (0, -scale),
            Direction::Up => (-scale, 0),
            Direction::Right => (0, scale),
            Direction::Down => (scale, 0)
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!("invalid direction")
        }
    }
}

pub fn solve_part1(data: &str) -> usize {

    let mut current_coord: (isize, isize) = (0, 0);
    let mut res = vec![current_coord];

    let dig_plan_re = Regex::new(r"([DLRU]) (\d+) \(.*\)").unwrap();

    data
    .lines()
    .for_each(|line| {
        let captures = dig_plan_re.captures(line).unwrap();

        let dir: Direction = captures.get(1).unwrap().as_str().chars().next().unwrap().into();
        let steps: isize = captures.get(2).unwrap().as_str().parse().unwrap();

        let delta = dir.delta(steps);
        current_coord = (current_coord.0 + delta.0, current_coord.1 + delta.1);
        res.push(current_coord);
    });

    res.remove(res.len() - 1);

    let polygon = Polygon::new(res.into_iter());

    polygon.area()
}


pub fn main() {
    let data = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    assert_eq!(solve_part1(data), 62);
    // assert_eq!(solve_part2(data), 0);
}