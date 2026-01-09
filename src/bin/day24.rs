use std::fmt::Display;

use advent_of_code_2023::*;
use itertools::Itertools;
use nalgebra::{DMatrix, DVector, Vector3, dvector};

pub const PUZZLE: &str = include_str!("../../puzzles/day24.txt");

fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1.unwrap()); // 7655 too low.
    //println!("Part 2: {}", d.part2.unwrap());
    //println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: Option<usize>,
    pub part2: Option<usize>,
    particles: Vec<Particle>,
}

#[derive(Debug, PartialEq)]
struct Particle {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
}

impl Display for Particle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {} @ {}, {}, {}",
            self.position[0],
            self.position[1],
            self.position[2],
            self.velocity[0],
            self.velocity[1],
            self.velocity[2],
        )?;
        Ok(())
    }
}

impl Particle {
    fn new(line: &str) -> Self {
        let x: Vec<f64> = line
            .replace(" ", "")
            .split([',', '@'])
            .flat_map(str::parse)
            .collect();
        assert_eq!(x.len(), 6);
        Self {
            position: Vector3::from_row_slice(&x[0..3]),
            velocity: Vector3::from_row_slice(&x[3..]),
        }
    }

    /// Intentionally doing this manually.
    ///
    /// We're looking to see if the two vectors intersect anywhere, not
    /// necessarily at the same time. Is there any times `s` and `t` such that
    /// `v1 s + p1 == v2 t + p2`? We can use linear algebra to find out.
    ///
    /// Rearranging terms,
    ///
    /// `v1 s - v2 t = p2 - p1`.
    ///
    /// Moving those terms `s` and `t` into their own vector,
    ///
    /// `(v1 - v2) [s; t] = p2 - p1`.
    ///
    /// Rename these as `A x = y` and we see that this problem could be
    /// solvable by taking `A^-1 A x = x = A^-1 y`. *Could be*, because
    /// the matrix is not invertible if `v1` and `v2` are multiples.
    ///
    /// We check for the parallel case with a tricky division. This is just
    /// `x1 / y1 = x2 / y2 ==> x1 * y2 = x2 * y1`.
    fn intersection_without_z(&self, other: &Self) -> Option<(DVector<f64>, DVector<f64>)> {
        let a = self.velocity[0];
        let b = other.velocity[0];
        let c = self.velocity[1];
        let d = other.velocity[1];
        let v1 = dvector![a, c];
        let v2 = dvector![b, d];
        let y1 = dvector![self.position[0], self.position[1]];
        let y2 = dvector![other.position[0], other.position[1]];

        if a * d == b * c {
            None
        } else {
            let a = DMatrix::from_columns(&[v1.clone(), -v2]);
            // println!("{a}");
            let y = &y2 - &y1;
            let x = &a.try_inverse().unwrap() * &y;
            let intersection = v1.column(0) * x[0] + &y1;

            // println!("{x}");
            // println!("{y}");

            Some((intersection, x))
            // if x[0].is_sign_negative() || x[1].is_sign_negative() {
            //     None
            // } else {
            //     Some(v1.column(0) * x[0] + &y1)
            // }
        }
    }
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let particles = input.lines().map(Particle::new).collect();
        Self {
            part1: None,
            part2: None,
            particles,
        }
    }

    fn solve(mut self) -> Self {
        let min_x;
        let min_y;
        let max_x;
        let max_y;

        #[cfg(test)]
        {
            min_x = 7.0;
            max_x = 27.0;
            min_y = 7.0;
            max_y = 27.0;
        }

        #[cfg(not(test))]
        {
            min_x = 200000000000000.0;
            max_x = 400000000000000.0;
            min_y = 200000000000000.0;
            max_y = 400000000000000.0;
        }

        let mut part1 = 0;
        let n = self.particles.len();
        for (i, j) in (0..n).cartesian_product(0..n) {
            if i >= j {
                continue;
            }
            let p1 = &self.particles[i];
            let p2 = &self.particles[j];
            #[cfg(test)]
            {
                println!("Hailstone A: {p1}");
                println!("Hailstone B: {p2}");
            }
            if let Some((intersection, t)) = p1.intersection_without_z(p2) {
                if min_x <= intersection[0]
                    && intersection[0] <= max_x
                    && min_y <= intersection[1]
                    && intersection[1] <= max_y
                {
                    if t[0].is_sign_negative() && t[1].is_sign_negative() {
                        #[cfg(test)]
                        {
                            println!("Hailstones' paths crossed in the past for both hailstones.");
                        }
                    } else if t[0].is_sign_negative() {
                        #[cfg(test)]
                        {
                            println!("Hailstones' paths crossed in the past for hailstone A.");
                        }
                    } else if t[1].is_sign_negative() {
                        #[cfg(test)]
                        {
                            println!("Hailstones' paths crossed in the past for hailstone B.");
                        }
                    } else {
                        #[cfg(test)]
                        {
                            println!(
                                "Hailstones' paths will cross inside the test area (at x={}, y={}).",
                                intersection[0], intersection[1]
                            );
                        }
                        part1 += 1;
                    }
                } else {
                    #[cfg(test)]
                    {
                        println!(
                            "Hailstones' paths will cross outside the test area (at x={}, y={}).",
                            intersection[0], intersection[1]
                        );
                    }
                }
            } else {
                #[cfg(test)]
                {
                    println!("Hailstone's paths are parallel; they never intersect.");
                }
            }
            #[cfg(test)]
            {
                println!();
            }
        }

        self.part1 = Some(part1);
        self
    }
}

#[cfg(test)]
mod puzzle_name {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day24.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, Some(2));
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, Some(47));
    }
}
