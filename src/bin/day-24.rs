use std::fmt::Display;

use colored::Colorize;

fn main() {
    let data = include_str!("../../data/24.in");
    let res = solve_part1(data, 200000000000000, 400000000000000);
    println!("part 1: {}", res);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hailstone {
    x0: isize,
    y0: isize,
    z0: isize,
    vx: isize,
    vy: isize,
    vz: isize,
}

impl Display for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {} @ {}, {}, {}", self.x0, self.y0, self.z0, self.vx, self.vy, self.vz)
    }
}


impl Hailstone {
    pub fn slope_2d(&self) -> Option<f64> {
        if self.vx == 0 {
            None
        } else {
            Some(self.vy as f64 / self.vx as f64)
        }
    }
    pub fn pos(&self) -> (isize, isize, isize) {
        (self.x0, self.y0, self.z0)
    }
    pub fn pos_2d(&self) -> (isize, isize) {
        (self.x0, self.y0)
    }

    /// Determine if two linear trajectories in R^2 intersect in a given box.
    pub fn crosses_2d(&self, other: &Hailstone, min_pos: isize, max_pos: isize, debug: bool) -> bool {

        if debug {
            println!("Hailstone A: {}", self);
            println!("Hailstone B: {}", other);
        }

        // Are trajectories parallel? If so, then they don't cross each other unless they start at the same point.
        if self.slope_2d() == other.slope_2d() {
            if self.pos_2d() != other.pos_2d() {
                if debug {
                    println!("Hailstones' paths are parallel; they never intersect.");
                }
            }
            return self.pos_2d() == other.pos_2d();
        }
        // Form a system of equations:
        // x0 + t0 * v0 = x1 + t1 * v1 
        // where x0, x1 are positions vectors, 
        // v0, v1 are velocity vectors, 
        // and t0, t1 are scalar timestamps where
        // these trajectories intersect.

        // Find scalars t0 and t1 that satisfy:
        // [[x00] + t0 * [[v00] = [[x10] + t1 * [[v10]
        // [x01]]        [v01]] = [x11]]        [v11]]

        let x00 = self.x0 as f64;
        let x01 = self.y0 as f64;
        let x10 = other.x0 as f64;
        let x11 = other.y0 as f64;
        let v00 = self.vx as f64;
        let v01 = self.vy as f64;
        let v10 = other.vx as f64;
        let v11 = other.vy as f64;

        let z0 = x00 - x10;
        let z1 = x01 - x11;

        // Get inverse of coefficient matrix:
        // [[-v00, v10]
        // [-v01, v11]]

        let determinant = (-v00 * v11) - (-v01 * v10);
        if determinant == 0.0 {
            unreachable!("Determinant is 0!");
        }

        // Adjoint is:
        // [[v11, -v10]
        // [v01, -v00]]
        // Take product with:
        // [[z0]
        // [z1]]
        // and divide by determinant.
        let t0 = (v11 * z0 - v10 * z1) / determinant;
        let t1 = (v01 * z0 - v00 * z1) / determinant;

        if t0 < 0.0 {
            if t1 < 0.0 {
                if debug {
                    println!("Hailstones' paths crossed in the past for both hailstones.");
                }
                return false;
            }
            if debug {
                println!("Hailstones' paths crossed in the past for hailstone A.");
            }
            return false;
        } else {
            if t1 < 0.0 {
                if debug {
                    println!("Hailstones' paths crossed in the past for hailstone B.");
                }
                return false;
            }
        }

        let pos = (self.x0 as f64 + t0 * self.vx as f64, self.y0 as f64 + t0 * self.vy as f64);
        let time_bound_valid = (pos.0.min(pos.1) >= min_pos as f64) && (pos.0.max(pos.1) <= max_pos as f64);

        if time_bound_valid {
            if debug {
                println!("Hailstones' paths will cross {} the test area (at x={:.3}, y={:.3}).", "inside".bold().green(), pos.0, pos.1);
            }
        } 
        else {
            if debug {
                println!("Hailstones' paths will cross {} the test area (at x={:.3}, y={:.3}).", "outside".red(), pos.0, pos.1);
            }
        }
        
        time_bound_valid
    }
}

pub fn parse_hailstone(s: &str) -> Hailstone {
    let (left, right) = s.split_once("@").unwrap();
    let left = left.trim();
    let right = right.trim();

    let position = left.split(", ").map(str::trim).map(str::parse::<isize>).collect::<Result<Vec<isize>, _>>().unwrap();
    let velocity = right.split(", ").map(str::trim).map(str::parse::<isize>).collect::<Result<Vec<isize>, _>>().unwrap();

    Hailstone {
        x0: position[0],
        y0: position[1],
        z0: position[2],
        vx: velocity[0],
        vy: velocity[1],
        vz: velocity[2]
    }
}


pub fn solve_part1(data: &str, min_pos: isize, max_pos: isize) -> usize {
    let hailstones = data.lines().map(parse_hailstone).collect::<Vec<_>>();
    let mut counter = 0;
    for i in 0..hailstones.len() {
        for j in i+1..hailstones.len() {
            if hailstones[i].crosses_2d(&hailstones[j], min_pos, max_pos, false) {
                counter += 1;
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smol() {
        let data = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!(solve_part1(data, 7, 27), 2);
    }

}