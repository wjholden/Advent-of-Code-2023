use std::fmt::Display;

use advent_of_code_2023::*;
use itertools::Itertools;
use nalgebra::{DMatrix, DVector, Matrix4, Vector3, Vector4, dvector};

pub const PUZZLE: &str = include_str!("../../puzzles/day24.txt");

/// MiniZinc wasn't so great this time. The integer literals are huge (Geocode
/// only handles 32-bit integers), and even so the solver might not have enough
/// information to find a solution quickly. You can get this to work by adding
/// bounds to each of the variables, but that won't work for the real puzzle.
///
/// ```minizinc
/// % Use this editor as a MiniZinc scratch book
/// var -100..100: vx;
/// var -100..100: vy;
/// var -100..100: vz;
/// var -100..100: px;
/// var -100..100: py;
/// var -100..100: pz;
///
/// var 1..100: a;
/// var 1..100: b;
/// var 1..100: c;
/// var 1..100: d;
/// var 1..100: e;
///
/// % Hailstone: 19, 13, 30 @ -2, 1, -2
/// constraint -2 * a + 19 == vx * a + px;
/// constraint 1 * a + 13 == vy * a + py;
/// constraint -2 * a + 30 == vz * a + pz;
///
/// % Hailstone: 18, 19, 22 @ -1, -1, -2
/// constraint -1 * b + 18 == vx * b + px;
/// constraint -1 * b + 19 == vy * b + py;
/// constraint -2 * b + 22 == vz * b + pz;
///
/// % Hailstone: 20, 25, 34 @ -2, -2, -4
/// constraint -2 * c + 20 == vx * c + px;
/// constraint -2 * c + 25 == vy * c + py;
/// constraint -4 * c + 34 == vz * c + pz;
///
/// % Hailstone: 12, 31, 28 @ -1, -2, -1
/// constraint -1 * d + 12 == vx * d + px;
/// constraint -2 * d + 31 == vy * d + py;
/// constraint -1 * d + 28 == vz * d + pz;
///
/// % Hailstone: 20, 19, 15 @ 1, -5, -3
/// constraint 1 * e + 20 == vx * e + px;
/// constraint -5 * e + 19 == vy * e + py;
/// constraint -3 * e + 15 == vz * e + pz;
/// ```
///
/// Surprisingly, Mathematica can solve the example quickly with just four
/// of the sample points:
/// ```mathematica
/// Solve[And[-2 a + 19 == vx a + px,
///  a + 13 == vy a + py, -2 a + 30 == vz a + pz, -b + 18 ==
///   vx b + px, -b + 19 == vy b + py, -2 b + 22 ==
///   vz b + pz, -2 c + 20 == vx c + px, -2 c + 25 ==
///   vy c + py, -4 c + 34 == vz c + pz, -d + 12 ==
///   vx d + px, -2 d + 31 == vy d + py, -d + 28 == vz d + pz], {a, b, c,
///   d, vx, vy, vz, px, py, pz}]
/// ```
///
/// Pumpkin might be unusuable due to the size of the inputs.
///
/// Not trivial, but here's a nice approach by Andy Tockman:
/// https://reddit.com/r/adventofcode/comments/18pnycy/2023_day_24_solutions/kepu26z/
///
/// I also want to try this brilliant solution by evouga:
/// https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kepu26z/
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1.unwrap()); // 7655 too low.
    println!("Part 2: {}", d.part2.unwrap());
    println!("{:?}", Puzzle::time(PUZZLE));

    println!(
        "\nThe above part 2 solution is close, but contains a precision error. Use Mathematica:\n"
    );
    mathematica(&d.hailstones);
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: Option<usize>,
    pub part2: Option<f64>,
    hailstones: Vec<Hailstone>,
}

#[derive(Debug, PartialEq)]
struct Hailstone {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
}

impl Display for Hailstone {
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

impl Hailstone {
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
    ///
    /// If successful, this function returns two vectors: the position of the
    /// intersection (only for `self`) and the times that the intersection
    /// would occur.
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
            // Hailstones have parallel velocities.
            None
        } else {
            let velocities = DMatrix::from_columns(&[v1.clone(), -v2]);
            let positions = &y2 - &y1;
            let times = &velocities.try_inverse().unwrap() * &positions;
            let intersection_location = v1.column(0) * times[0] + &y1;

            Some((intersection_location, times))
        }
    }

    fn to_constraints(&self, time: usize) -> Vec<String> {
        ["x", "y", "z"]
            .iter()
            .enumerate()
            .map(|(i, var)| {
                format!(
                    "{velocity}*t{time}+{position}==v{var}*t{time}+p{var}",
                    velocity = self.velocity[i],
                    position = self.position[i]
                )
            })
            .collect()
    }
}

fn mathematica(hailstones: &[Hailstone]) {
    let n = 3;
    let constraints = hailstones
        .iter()
        .enumerate()
        .take(n)
        .fold(Vec::new(), |mut acc, (i, hailstone)| {
            acc.append(&mut hailstone.to_constraints(i));
            acc
        })
        .join(",");
    let vars = (0..n).map(|i| format!("t{i}")).join(",");
    let mathematica_solution = format!("Solve[And[{constraints}],{{{vars},px,py,pz,vx,vy,vz}}]");
    println!("{mathematica_solution}");
}

/// https://www.reddit.com/r/adventofcode/comments/18q40he/2023_day_24_part_2_a_straightforward_nonsolver/
///
/// Let `vr` and `pr` be the velocity and position of the rock. Let `vi` and `pi` be
/// the velocity and position of hailstone `i`.
///
/// We get an intersection when `vr * ti + pr = vi * ti + pi`. That's really
/// seven unknowns: `x`, `y`, and `z` for both `vr` and `pr`, and also `ti`. Because `ti` is
/// different for every hailstone, adding another equation would only add yet
/// another unknown.
///
/// If we break these out into components (`x`, `y`, and `z`) we get
///
/// `ti = (pxr - pxi) / (vxi - vxr) == (pyr - pyi) / (pyi - pyr) = (pzr - pzi) / (pzi - pzr)`.
///
/// We can use this to ignore the times altogether!
///
/// Take only the `x` and `y` components.
///
/// `(pxr - pxi)(vyi - vyr) == (pyr - pyi)(vxi - vxr)`
///
/// Expand.
///
/// `-(pxi vyi) + pxr vyi + pxi vyr - pxr vyr == -(pyi vxi) + pyr vxi + pyi vxr - pyr vxr`
///
/// Now we rearrange the terms to put the rock-only terms on the left-hand side.
///
/// `pyr vxr - pxr vyr == -(pyi vxi) + pyr vxi + pyi vxr +(pxi vyi) -pxr vyi -pxi vyr`
///
/// The sum `pyr vxr - pxr vyr` never changes. We can choose another hailstone, `j`,
/// and now we can set those two equations equal to another.
///
/// `-(pyi vxi) + pyr vxi + pyi vxr +(pxi vyi) -pxr vyi -pxi vyr == -(pyj vxj) + pyr vxj + pyj vxr +(pxj vyj) -pxr vyj -pxj vyr`
///
/// Move the rock terms to one side.
///
/// `pyr vxi + pyi vxr -pxr vyi -pxi vyr - pyr vxj - pyj vxr +pxr vyj +pxj vyr == -(pyj vxj) +(pxj vyj) +(pyi vxi) - (pxi vyi)`
///
/// `(-vyi + vyj) pxr + (vxi - vxj) pyr + (pyi - pyj) vxr + (-pxi + pxj) vyr == -(pyj vxj) + (pxj vyj) + (pyi vxi) -(pxi vyi)`
///
/// Looks scary, but with just two more points we can solve this with linear algebra.
fn tockman(hailstones: &[Hailstone]) -> (f64, f64, f64) {
    let mut a1 = Matrix4::zeros();
    let mut y1 = Vector4::zeros();
    let mut a2 = Matrix4::zeros();
    let mut y2 = Vector4::zeros();

    for (i, (hi, hj)) in hailstones
        .iter()
        // .skip(1)
        .tuple_windows()
        .take(4)
        .enumerate()
    {
        if let [pxi, pyi, pzi] = hi.position.as_slice()
            && let [pxj, pyj, pzj] = hj.position.as_slice()
            && let [vxi, vyi, vzi] = hi.velocity.as_slice()
            && let [vxj, vyj, vzj] = hj.velocity.as_slice()
        {
            // (dy'-dy) X + (dx-dx') Y + (y-y') DX + (x'-x) DY
            a1[(i, 0)] = -vyi + vyj; // pxr
            a1[(i, 1)] = vxi - vxj; // pyr
            a1[(i, 2)] = pyi - pyj; // vxr
            a1[(i, 3)] = -pxi + pxj; // vyr

            a2[(i, 0)] = -vzi + vzj; // pxr
            a2[(i, 1)] = vxi - vxj; // pzr
            a2[(i, 2)] = pzi - pzj; // vxr
            a2[(i, 3)] = -pxi + pxj; // vzr

            // = x' dy' - y' dx' - x dy + y dx
            y1[i] = -(pyj * vxj) + (pxj * vyj) + (pyi * vxi) - (pxi * vyi);
            y2[i] = -(pzj * vxj) + (pxj * vzj) + (pzi * vxi) - (pxi * vzi);
        }
    }

    let x1 = a1.try_inverse().unwrap() * y1;
    let x2 = a2.try_inverse().unwrap() * y2;
    // let x1 = a1.lu().solve(&y1).unwrap();
    // let x2 = a2.lu().solve(&y2).unwrap();

    debug_assert_eq!(x1[0].round(), x2[0].round());

    (x1[0].round(), x1[1].round(), x2[1].round())
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let hailstones = input.lines().map(Hailstone::new).collect();
        Self {
            part1: None,
            part2: None,
            hailstones,
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
        let n = self.hailstones.len();
        for (i, j) in (0..n).cartesian_product(0..n) {
            if i >= j {
                continue;
            }
            let p1 = &self.hailstones[i];
            let p2 = &self.hailstones[j];

            if p1.position == p2.position {
                println!("Same starting position:");
                println!("- {p1}");
                println!("- {p2}");
            }

            if p1.velocity == p2.velocity {
                println!("Same velocity:");
                println!("- {p1}");
                println!("- {p2}");
            }

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

        let (a, b, c) = tockman(&self.hailstones);
        self.part2 = Some(a + b + c);

        self
    }
}

#[cfg(test)]
mod never_tell_me_the_odds {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day24.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, Some(2));
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, Some(47.0));
    }
}
