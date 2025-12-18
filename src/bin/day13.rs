use advent_of_code_2023::*;
use nalgebra::DMatrix;

pub const PUZZLE: &str = include_str!("../../puzzles/day13.txt");

/// Frustrating problems with lots of cases.
///
/// Type system for the win! I needed to be able to distinguish vertical from
/// horizontal lines. Rust's enums really helped enforce correctness here.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2); // 28957 too low, 36010 also too low.
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Default, Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    patterns: Vec<Pattern>,
}

#[derive(Debug)]
struct Pattern {
    mirrors: DMatrix<i8>,
}

#[derive(Debug, PartialEq, Eq)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Eq)]
enum Discovery<T> {
    HorizontalLine(T),
    VerticalLine(T),
    Smudge(T, T),
    Nothing,
}

impl Pattern {
    fn reflection_line(&self, line_direction: Orientation) -> Vec<Discovery<usize>> {
        let mut solutions = Vec::new();
        let n = match line_direction {
            Orientation::Horizontal => self.mirrors.nrows(),
            Orientation::Vertical => self.mirrors.ncols(),
        };

        'outer: for i in 1..n {
            let mut di = 1;

            while di <= i && i + di - 1 < n {
                let a = i - di;
                let b = i + di - 1;

                // I don't like this duplication. I've been trying to make some
                // closures to compress the logic into something more abstract,
                // but I keep running into type mismatches, either with the
                // closures themselves or with their return types.
                match line_direction {
                    Orientation::Horizontal => {
                        let v1 = self.mirrors.row(a);
                        let v2 = self.mirrors.row(b);
                        let count_ones = (v2 - v1).iter().filter(|x| x.abs() == 1).count();
                        match count_ones {
                            0 => (),              // Candidate solution for a reflection line.
                            _ => continue 'outer, // not a smudge nor a reflection line.
                        }
                    }
                    Orientation::Vertical => {
                        let v1 = self.mirrors.column(a);
                        let v2 = self.mirrors.column(b);
                        let count_ones = (v2 - v1).iter().filter(|x| x.abs() == 1).count();
                        match count_ones {
                            0 => (),              // Candidate solution for a reflection line.
                            _ => continue 'outer, // not a smudge nor a reflection line.
                        }
                    }
                }

                di += 1;
            }

            solutions.push(match line_direction {
                Orientation::Horizontal => Discovery::HorizontalLine(i),
                Orientation::Vertical => Discovery::VerticalLine(i),
            });
        }
        solutions
    }

    fn find_smudge(&self, line_direction: Orientation) -> Discovery<usize> {
        let n = match line_direction {
            Orientation::Horizontal => self.mirrors.nrows(),
            Orientation::Vertical => self.mirrors.ncols(),
        };

        'outer: for i in 1..n {
            let mut di = 1;
            let mut candidate_smudge = Discovery::Nothing;

            while di <= i && i + di - 1 < n {
                let a = i - di;
                let b = i + di - 1;

                // I don't like this duplication. I've been trying to make some
                // closures to compress the logic into something more abstract,
                // but I keep running into type mismatches, either with the
                // closures themselves or with their return types.
                match line_direction {
                    Orientation::Horizontal => {
                        let v1 = self.mirrors.row(a);
                        let v2 = self.mirrors.row(b);
                        let count_ones = (v2 - v1).iter().filter(|x| x.abs() == 1).count();
                        match count_ones {
                            0 => (),
                            1 if matches!(candidate_smudge, Discovery::Nothing) => {
                                candidate_smudge = Discovery::Smudge(
                                    a,
                                    (v2 - v1).iter().position(|x| x.abs() == 1).unwrap(),
                                )
                            }
                            _ => continue 'outer,
                        }
                    }
                    Orientation::Vertical => {
                        let v1 = self.mirrors.column(a);
                        let v2 = self.mirrors.column(b);
                        let count_ones = (v2 - v1).iter().filter(|x| x.abs() == 1).count();
                        match count_ones {
                            0 => (), // Candidate solution for discovering ONE smudge.
                            1 if matches!(candidate_smudge, Discovery::Nothing) => {
                                candidate_smudge = Discovery::Smudge(
                                    (v2 - v1).iter().position(|x| x.abs() == 1).unwrap(),
                                    a, // Are we completely sure the smudge is at a and not b?
                                )
                            }
                            _ => continue 'outer, // We found a second smudge or too many differences.
                        }
                    }
                }

                di += 1;
            }

            if let Discovery::Smudge(_, _) = candidate_smudge {
                return candidate_smudge;
            }
        }
        Discovery::Nothing
    }
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let input = input.replace("\r", "");
        let mut instance = Self::default();
        for pattern in input.split("\n\n") {
            let rows = pattern.lines().count();
            let cols = pattern.lines().next().unwrap().len();
            let mirrors = DMatrix::from_row_iterator(
                rows,
                cols,
                pattern.chars().filter_map(|c| match c {
                    '#' => Some(1),
                    '.' => Some(0),
                    '\n' => None,
                    _ => panic!("unexpected character"),
                }),
            );
            instance.patterns.push(Pattern { mirrors });
        }
        instance
    }

    fn solve(mut self) -> Self {
        for (p, pattern) in self.patterns.iter_mut().enumerate() {
            let mut solution1 = vec![];

            let candidate = pattern.reflection_line(Orientation::Horizontal);
            if let [Discovery::HorizontalLine(rows)] = candidate[..] {
                self.part1 += 100 * rows;
                solution1 = candidate;
            }
            let candidate = pattern.reflection_line(Orientation::Vertical);
            if let [Discovery::VerticalLine(cols)] = candidate[..] {
                self.part1 += cols;
                solution1 = candidate;
            }

            if let Discovery::Smudge(i, j) = pattern.find_smudge(Orientation::Horizontal) {
                pattern.mirrors[(i, j)] ^= 1;
            } else if let Discovery::Smudge(i, j) = pattern.find_smudge(Orientation::Vertical) {
                pattern.mirrors[(i, j)] ^= 1;
            } else {
                panic!("no smudge found for {p}");
            }

            let mut s2_h = pattern.reflection_line(Orientation::Horizontal);
            s2_h.retain(|x| !solution1.contains(x));
            if let [Discovery::HorizontalLine(rows)] = s2_h[..] {
                self.part2 += 100 * rows;
            }

            let mut s2_v = pattern.reflection_line(Orientation::Vertical);
            s2_v.retain(|x| !solution1.contains(x));
            if let [Discovery::VerticalLine(cols)] = s2_v[..] {
                self.part2 += cols;
            }

            if s2_h.len() != 1 && s2_v.len() != 1 {
                println!(
                    "Check {p}: solution 1 = {solution1:?} but s2_h = {s2_h:?} and s2_v = {s2_v:?}"
                );
            }
        }

        self
    }
}

#[cfg(test)]
mod point_of_incidence {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day13.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 405);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 400);
    }
}
