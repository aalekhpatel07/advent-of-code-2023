use colored::Colorize;
use nalgebra::*;
use std::fmt::Display;

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

    pub fn position_and_velocity(&self) -> (Vector3<f64>, Vector3<f64>) {
        let pos = Vector3::new(self.x0 as f64, self.y0 as f64, self.z0 as f64);
        let vel = Vector3::new(self.vx as f64, self.vy as f64, self.vz as f64);
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
    let hailstones = data
        .lines()
        .map(parse_hailstone)
        .collect::<std::vec::Vec<_>>();
    let mut counter = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            if hailstones[i].crosses_2d(&hailstones[j], min_pos, max_pos, debug) {
                counter += 1;
            }
            if debug {
                println!();
            }
        }
    }
    counter
}

/// Given a row vector of dimension 3, build a skew-symmetric matrix
/// that represents [cross product as matrix multiplication] and
/// appears in the analytical solution for finding an
/// initial position and a velocity for a trajectory in R3 that intersects
/// a given set of other trajectories at different points of time.
///
/// [cross product as matrix multiplication]: https://en.wikipedia.org/wiki/Skew-symmetric_matrix#Cross_product
pub fn build_skew_symmetric_matrix(v: &Vector3<f64>) -> Matrix3<f64> {
    let v = v.as_ref();
    Matrix3::new(0., -v[2], v[1], v[2], 0., -v[0], -v[1], v[0], 0.)
}

pub fn solve_part2(data: &str) -> f64 {
    let hailstones = data
        .lines()
        .map(parse_hailstone)
        .collect::<std::vec::Vec<_>>();

    // Pick the first three hailstones to compute the unique position/velocity for the rock.
    let (i, j, k) = (0, 1, 2);

    let (p0, v0) = hailstones[i].position_and_velocity();
    let (p1, v1) = hailstones[j].position_and_velocity();
    let (p2, v2) = hailstones[k].position_and_velocity();

    // The solution comes from the observation that (P - Pi) X (V - Vi) = 0,
    // and then simplifying to get
    // (Pi X V) + (P X Vi) - (Pi X Vi) = (Pj X V) + (P X Vj) - (Pj X Vj) for all i, j.

    // Velocity terms for equations between h0 and h1.
    let top_left = build_skew_symmetric_matrix(&v0) - build_skew_symmetric_matrix(&v1);
    // Velocity terms for equations between h0 and h2.
    let bottom_left = build_skew_symmetric_matrix(&v0) - build_skew_symmetric_matrix(&v2);
    // Position terms for equations between h0 and h1.
    let top_right = build_skew_symmetric_matrix(&p1) - build_skew_symmetric_matrix(&p0);
    // Position terms for equations between h0 and h2.
    let bottom_right = build_skew_symmetric_matrix(&p2) - build_skew_symmetric_matrix(&p0);

    // The solution then is found by solving Ax=b where A is a 6x6 matrix formed
    // from the smaller skew symmetric matrices as quadrants, x is the 6x1 row vector
    // representing the positions and velocities (unknowns), and b is the row vector of
    // cross product differences between pairs of trajectories.
    let mut coefficient_matrix: Matrix6<f64> = Matrix6::new(
        top_left.m11,
        top_left.m12,
        top_left.m13,
        top_right.m11,
        top_right.m12,
        top_right.m13,
        top_left.m21,
        top_left.m22,
        top_left.m23,
        top_right.m21,
        top_right.m22,
        top_right.m23,
        top_left.m31,
        top_left.m32,
        top_left.m33,
        top_right.m31,
        top_right.m32,
        top_right.m33,
        bottom_left.m11,
        bottom_left.m12,
        bottom_left.m13,
        bottom_right.m11,
        bottom_right.m12,
        bottom_right.m13,
        bottom_left.m21,
        bottom_left.m22,
        bottom_left.m23,
        bottom_right.m21,
        bottom_right.m22,
        bottom_right.m23,
        bottom_left.m31,
        bottom_left.m32,
        bottom_left.m33,
        bottom_right.m31,
        bottom_right.m32,
        bottom_right.m33,
    );

    if !coefficient_matrix.try_inverse_mut() {
        panic!("coefficient matrix isn't invertible, so no solutions exist.");
    }

    let row_vector_upper = p1.cross(&v1) - p0.cross(&v0);
    let row_vector_lower = p2.cross(&v2) - p0.cross(&v0);

    let row_vector = Vector6::new(
        row_vector_upper.x,
        row_vector_upper.y,
        row_vector_upper.z,
        row_vector_lower.x,
        row_vector_lower.y,
        row_vector_lower.z,
    );

    let rock_pos_vel = coefficient_matrix * row_vector;
    let arr = rock_pos_vel.as_ref();
    let mut pos = Vector3::from_row_slice(&arr[..3]);

    // Floating point precision issue.
    // Aggressively round values if they don't lie
    // on quarters.
    if pos.y.fract() < 0.2 || pos.y.fract() > 0.8 {
        pos.y = pos.y.round();
    }
    if pos.x.fract() < 0.2 || pos.x.fract() > 0.8 {
        pos.x = pos.x.round();
    }
    if pos.z.fract() < 0.2 || pos.z.fract() > 0.8 {
        pos.z = pos.z.round();
    }
    pos.sum().floor()
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
        assert_eq!(solve_part1(data, 7, 27, true), 2);
        assert_eq!(solve_part2(data), 47.);
    }
}
