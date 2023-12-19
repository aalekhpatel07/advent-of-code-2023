use aoc_2023::data_structures::LateralPolygon;
use regex::Regex;

pub fn main() {
    let data = include_str!("../../data/18.in");
    println!("part 1: {}", solve(data, false));
    println!("part 2: {}", solve(data, true));
}

/// TIL: [Pick's Theorem] states that `A = i + b/2 - 1`
/// where `i` denotes the number of lattice points interior to the polygon,
/// `b` denotes the number of lattice points on the boundary of the polygon,
/// and `A` denotes the area of the polygon.
///
/// The Shoelace formula gives us the area of the polygon, `A`.
/// The perimeter `b` is simply the number of distinct lattice points on the horizontal/vertical edges of the polygon.
///
/// For this problem we want to calculate `i + b`, so using Pick's theorem, we have
/// the desired answer is `A + b / 2 + 1`.
///
/// [Pick's Theorem]: https://en.wikipedia.org/wiki/Pick%27s_theorem
pub fn solve(data: &str, is_part2: bool) -> usize {
    let polygon = parse_polygon(data, is_part2);
    polygon.shoelace_area() + polygon.perimeter() / 2 + 1
}

pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn delta(&self, scale: isize) -> (isize, isize) {
        match *self {
            Direction::Left => (0, -scale),
            Direction::Up => (-scale, 0),
            Direction::Right => (0, scale),
            Direction::Down => (scale, 0),
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
            _ => panic!("invalid direction"),
        }
    }
}

pub fn parse_polygon(data: &str, is_part2: bool) -> LateralPolygon {
    let mut current_coord: (isize, isize) = (0, 0);
    let mut res = vec![current_coord];

    let dig_plan_re = Regex::new(r"([DLRU]) (\d+) \((.*)\)").unwrap();

    data.lines().for_each(|line| {
        let captures = dig_plan_re.captures(line).unwrap();
        let (dir, steps) = match is_part2 {
            true => {
                let color = captures.get(3).unwrap().as_str();
                let hex_encoded_dist = usize::from_str_radix(&color[1..6], 16).unwrap();
                let dir = match color[6..7].parse::<usize>().unwrap() {
                    0 => Direction::Right,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    3 => Direction::Up,
                    _ => {
                        panic!("expected one of 0|1|2|3");
                    }
                };
                (dir, hex_encoded_dist as isize)
            }
            false => {
                let dir: Direction = captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
                    .unwrap()
                    .into();
                let steps: isize = captures.get(2).unwrap().as_str().parse().unwrap();
                (dir, steps)
            }
        };

        let delta = dir.delta(steps);
        current_coord = (current_coord.0 + delta.0, current_coord.1 + delta.1);
        res.push(current_coord);
    });

    LateralPolygon::new(res.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smol() {
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
        assert_eq!(solve(data, false), 62);
        assert_eq!(solve(data, true), 952408144115);
    }
}
