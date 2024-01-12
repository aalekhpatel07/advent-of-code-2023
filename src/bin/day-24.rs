use std::fmt::Display;
use aoc_2023::data_structures::{Vec, Matrix};
use colored::Colorize;

fn main() {
    let data = include_str!("../../data/24.in");
    let res = solve_part1(data, 200000000000000, 400000000000000, false);
    println!("part 1: {}", res);
    println!("part 2: {}", solve_part2(data));
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
        write!(
            f,
            "{}, {}, {} @ {}, {}, {}",
            self.x0, self.y0, self.z0, self.vx, self.vy, self.vz
        )
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
    pub fn pos_2d(&self) -> (isize, isize) {
        (self.x0, self.y0)
    }

    /// Determine if two linear trajectories in R^2 intersect in a given box.
    pub fn crosses_2d(
        &self,
        other: &Hailstone,
        min_pos: isize,
        max_pos: isize,
        debug: bool,
    ) -> bool {
        if debug {
            println!("Hailstone A: {}", self);
            println!("Hailstone B: {}", other);
        }

        // Are trajectories parallel? If so, then they don't cross each other unless they start at the same point.
        if self.slope_2d() == other.slope_2d() {
            if self.pos_2d() != other.pos_2d() && debug {
                println!("Hailstones' paths are parallel; they never intersect.");
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
        } else if t1 < 0.0 {
            if debug {
                println!("Hailstones' paths crossed in the past for hailstone B.");
            }
            return false;
        }

        let pos = (
            self.x0 as f64 + t0 * self.vx as f64,
            self.y0 as f64 + t0 * self.vy as f64,
        );
        let time_bound_valid =
            (pos.0.min(pos.1) >= min_pos as f64) && (pos.0.max(pos.1) <= max_pos as f64);

        if time_bound_valid {
            if debug {
                println!(
                    "Hailstones' paths will cross {} the test area (at x={:.3}, y={:.3}).",
                    "inside".bold().green(),
                    pos.0,
                    pos.1
                );
            }
        } else if debug {
            println!(
                "Hailstones' paths will cross {} the test area (at x={:.3}, y={:.3}).",
                "outside".red(),
                pos.0,
                pos.1
            );
        }

        time_bound_valid
    }

    pub fn position_and_velocity(&self) -> (Vec<f64, 3>, Vec<f64, 3>) 
    {
        let pos = Vec::new(self.x0 as f64, self.y0 as f64, self.z0 as f64);
        let vel = Vec::new(self.vx as f64, self.vy as f64, self.vz as f64);
        (pos, vel)
    }
}


pub fn parse_hailstone(s: &str) -> Hailstone {
    let (left, right) = s.split_once('@').unwrap();
    let left = left.trim();
    let right = right.trim();

    let position = left
        .split(", ")
        .map(str::trim)
        .map(str::parse::<isize>)
        .collect::<Result<std::vec::Vec<isize>, _>>()
        .unwrap();
    let velocity = right
        .split(", ")
        .map(str::trim)
        .map(str::parse::<isize>)
        .collect::<Result<std::vec::Vec<isize>, _>>()
        .unwrap();

    Hailstone {
        x0: position[0],
        y0: position[1],
        z0: position[2],
        vx: velocity[0],
        vy: velocity[1],
        vz: velocity[2],
    }
}

pub fn solve_part1(data: &str, min_pos: isize, max_pos: isize, debug: bool) -> usize {
    let hailstones = data.lines().map(parse_hailstone).collect::<std::vec::Vec<_>>();
    let mut counter = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            if hailstones[i].crosses_2d(&hailstones[j], min_pos, max_pos, debug) {
                counter += 1;
            }
            if debug { println!(); }
        }
    }
    counter
}


pub fn cross_matrix(v: &Vec<f64, 3>) -> Matrix<f64, 3, 3> {
    let v = v.as_ref();
    Matrix::try_from_slice(&[
        0.,
        -v[2],
        v[1],
        v[2],
        0.,
        -v[0],
        -v[1],
        v[0],
        0.
    ]).unwrap()
}


pub fn solve_part2(data: &str) -> f64 {
    let hailstones = data.lines().map(parse_hailstone).collect::<std::vec::Vec<_>>();

    let (i, j, k) = (0, 1, 2);

    let (h0, h1, h2) = (hailstones[i], hailstones[j], hailstones[k]);

    let (p0, v0) = h0.position_and_velocity();
    let (p1, v1) = h1.position_and_velocity();
    let (p2, v2) = h2.position_and_velocity();

    let row_vector_upper = p1.cross(&v1) - p0.cross(&v0);
    let row_vector_lower = p2.cross(&v2) - p0.cross(&v0);


    println!("{}, {}", row_vector_upper, row_vector_lower);
    // let row_data = vec![
    //     row_vector_upper.x,
    //     row_vector_upper.y,
    //     row_vector_upper.z,
    //     row_vector_lower.x,
    //     row_vector_lower.y,
    //     row_vector_lower.z
    // ];

    // // let matrix: Matrix<f64, 6, 6> = Matrix::try_from_slice(&matrix_data).unwrap();
    // let row: Vec<f64, 6> = Vec::try_from_slice(&row_data).unwrap();

    // println!("old:\n{}", matrix);
    // println!("old:\n{}", row);

    let top_left = cross_matrix(&v0) - cross_matrix(&v1);
    let bottom_left = cross_matrix(&v0) + cross_matrix(&v2);
    let top_right = cross_matrix(&p1) - cross_matrix(&p0);
    let bottom_right = cross_matrix(&p2) - cross_matrix(&p0);


    let matrix = Matrix::<f64, 6, 6>::try_from_blocks(
        &top_left, 
        &top_right, 
        &bottom_left, 
        &bottom_right
    ).unwrap();

    println!("{}", matrix);
    let m1 = matrix.get_block::<3, 3>(0, 0).unwrap();
    let m2 = matrix.get_block::<3, 3>(0, 3).unwrap();
    let m3 = matrix.get_block::<3, 3>(3, 0).unwrap();
    let m4 = matrix.get_block::<3, 3>(3, 3).unwrap();
    println!("{}\n{}\n{}\n{}", m1, m2, m3, m4);

    println!("{}", matrix.inverse().unwrap());

    // Matrix::try_from_slice()

    // println!("h{}: {} + t{}", i, p0, y0);
    // println!("h{}: {} + t{}", j, p1, y1);
    // println!("h{}: {} + t{}", k, p2, y2);
    // println!();
    // let z01 = (y0 - y1).cross(&(p0 - p1));
    // println!("z01: {}", z01);
    // let z02 = (y0 - y2).cross(&(p0 - p2));
    // println!("z02: {}", z02);
    // let z12 = (y1 - y2).cross(&(p1 - p2));
    // println!("z12: {}", z12);

    // let d01 = (y0 - y1).dot(&p0.cross(&p1));
    // let d02 = (y0 - y2).dot(&p0.cross(&p2));
    // let d12 = (y1 - y2).dot(&p1.cross(&p2));

    // let matrix = M3x3::new(
    //     &z01, &z02, &z12
    // );
    // println!("Matrix:\n{}", matrix);
    // let Some(inverse) = matrix.inverse() else { 
    //     panic!("Matrix is not invertible. Weird")
    // };
    // println!("Inverse:\n{}", inverse);

    // let scalars = Vec3::new(d01, d02, d12);

    // println!("Scalars: {}", scalars);
    // let position = inverse * &scalars;

    // println!("{}", position);

    // position.x + position.y + position.z
    0.
}


#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2023::data_structures::Vec;

    #[test]
    fn smol() {
        let data = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        // assert_eq!(solve_part1(data, 7, 27, true), 2);
        assert_eq!(solve_part2(data), 0.);
    }

    #[test]
    fn cross() {

        let r1 = Vec::<f64, 3>::new(1.0, 0.0, 5.0);
        let r2 = Vec::<f64, 3>::new(2.0, 1.0, 6.0);

        assert_eq!(r1.cross(&r2), Vec::<f64, 3>::new(-5.0, 4.0, 1.0));
        assert_eq!(r2.cross(&r1), -r1.cross(&r2));

        let r1 = Vec::<f64, 3>::new(1.0, -2.0, 0.);
        let r2 = Vec::new(1.0, -6.0, 8.0);
        println!("{} x {} = {}", r1, r2, r1.cross(&r2));
    }
}
