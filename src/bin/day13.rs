use advent_of_code_2023::*;
use nalgebra::DMatrix;

pub const PUZZLE: &str = include_str!("../../puzzles/day13.txt");

/// Frustrating problems with lots of cases.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2); // 28957 too low
    //println!("{:?}", Puzzle::time(PUZZLE));
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
    fn reflection_line(&self, line_direction: Orientation) -> Discovery<usize> {
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

            // If we made it to here then we've found a reflection line.
            return match line_direction {
                Orientation::Horizontal => Discovery::HorizontalLine(i),
                Orientation::Vertical => Discovery::VerticalLine(i),
            };
        }
        Discovery::Nothing
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

    fn horizontal_reflection_line(&self, part: Part) -> Option<usize> {
        let nrows = self.mirrors.nrows();
        'outer: for i in 1..nrows {
            let mut di = 1;
            let mut rows_skipped = 0;

            while di <= i && i + di - 1 < nrows {
                let above = i - di;
                let below = i + di - 1;

                if self.mirrors.row(above) != self.mirrors.row(below) {
                    match part {
                        Part::One => continue 'outer,
                        Part::Two if rows_skipped == 0 => {
                            let diff = self.mirrors.row(above) - self.mirrors.row(below);
                            let count_ones = diff.iter().filter(|&&x| x == 1 || x == -1).count();
                            if count_ones == 1 {
                                let col = diff.iter().position(|&x| x == 1 || x == -1).unwrap();
                                println!(
                                    "smudge must be at ({above},{col}) or ({below},{col}) with i={i}"
                                );
                                // And now skip this one row.
                                rows_skipped = 1;
                            } else {
                                continue 'outer;
                            }
                        }
                        Part::Two => continue 'outer,
                    }
                }

                di += 1;
            }

            return Some(i);
        }
        None
    }

    fn vertical_reflection_line(&self, part: Part) -> Option<usize> {
        let ncols = self.mirrors.ncols();
        'outer: for i in 1..ncols {
            let mut di = 1;
            let mut cols_skipped = 0;

            while di <= i && i + di - 1 < ncols {
                let west = i - di;
                let east = i + di - 1;

                if self.mirrors.column(west) != self.mirrors.column(east) {
                    continue 'outer;
                }

                if self.mirrors.column(west) != self.mirrors.column(east) {
                    match part {
                        Part::One => continue 'outer,
                        Part::Two if cols_skipped == 0 => {
                            let diff = self.mirrors.column(west) - self.mirrors.column(east);
                            let count_ones = diff.iter().filter(|&&x| x == 1 || x == -1).count();
                            if count_ones == 1 {
                                let row = diff.iter().position(|&x| x == 1 || x == -1).unwrap();
                                println!(
                                    "smudge must be at ({row},{east}) or ({row},{east}) with i={i}"
                                );
                                // And now skip this one column.
                                cols_skipped = 1;
                            } else {
                                continue 'outer;
                            }
                        }
                        Part::Two => continue 'outer,
                    }
                }

                di += 1;
            }

            return Some(i);
        }
        None
    }

    fn _find_smudge_horizontal(&self) -> Option<usize> {
        // assert_eq!(self.horizontal_reflection_line(), None);
        let nrows = self.mirrors.nrows();
        'outer: for i in 1..nrows {
            let mut di = 1;

            while di <= i && i + di - 1 < nrows {
                let above = i - di;
                let below = i + di - 1;

                if self.mirrors.row(above) != self.mirrors.row(below) {
                    let diff = self.mirrors.row(above) - self.mirrors.row(below);
                    let count_ones = diff.iter().filter(|&&x| x == 1 || x == -1).count();
                    if count_ones == 1 {
                        let col = diff.iter().position(|&x| x == 1 || x == -1).unwrap();
                        println!("smudge must be at ({above},{col}) or ({below},{col}) with i={i}");
                        return Some(i);
                    }
                    continue 'outer;
                }

                di += 1;
            }
        }
        None
    }

    fn _find_smudge_vertical(&self) -> Option<usize> {
        // assert_eq!(self.vertical_reflection_line(), None);
        let ncols = self.mirrors.ncols();
        'outer: for i in 1..ncols {
            let mut di = 1;

            while di <= i && i + di - 1 < ncols {
                let west = i - di;
                let east = i + di - 1;

                if self.mirrors.column(west) != self.mirrors.column(east) {
                    let diff = self.mirrors.column(west) - self.mirrors.column(east);
                    let count_ones = diff.iter().filter(|&&x| x == 1 || x == -1).count();
                    if count_ones == 1 {
                        let row = diff.iter().position(|&x| x == 1 || x == -1).unwrap();
                        println!("smudge must be at ({row},{west}) or ({row},{east}) with i={i}");
                        return Some(i);
                    }
                    continue 'outer;
                }

                di += 1;
            }
        }
        None
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
            //println!("{mirrors}");
            instance.patterns.push(Pattern { mirrors });
        }
        instance
    }

    fn solve(mut self) -> Self {
        for (p, pattern) in self.patterns.iter_mut().enumerate() {
            // if let Some(cols) = pattern.vertical_reflection_line(Part::One) {
            //     self.part1 += cols;
            //     // self.part2 += 100
            //     //     * pattern
            //     //         .find_smudge_horizontal()
            //     //         .expect("rows above new horizontal reflection line");
            // }
            // if let Some(rows) = pattern.horizontal_reflection_line(Part::One) {
            //     self.part1 += 100 * rows;
            //     // self.part2 += pattern
            //     //     .find_smudge_vertical()
            //     //     .expect("cols west of new vertical reflection line");
            // }

            // if let Some(cols) = pattern.vertical_reflection_line(Part::Two) {
            //     self.part2 += cols;
            // }
            // if let Some(rows) = pattern.horizontal_reflection_line(Part::Two) {
            //     self.part2 += 100 * rows;
            // }

            println!(
                "{p}: {:?}",
                pattern.reflection_line(Orientation::Horizontal)
            );
            println!("{p}: {:?}", pattern.reflection_line(Orientation::Vertical));
            println!("{p}: {:?}", pattern.find_smudge(Orientation::Horizontal));
            println!("{p}: {:?}", pattern.find_smudge(Orientation::Vertical));
            let mut solution1 = Discovery::<usize>::Nothing;

            if let Discovery::HorizontalLine(rows) =
                pattern.reflection_line(Orientation::Horizontal)
            {
                // println!("[{p}] Part 1: Horizontal line at row {rows}");
                self.part1 += 100 * rows;
                solution1 = Discovery::HorizontalLine(rows);
            }
            if let Discovery::VerticalLine(cols) = pattern.reflection_line(Orientation::Vertical) {
                // println!("[{p}] Part 1: Vertical line at column {cols}");
                self.part1 += cols;
                solution1 = Discovery::VerticalLine(cols);
            }

            if let Discovery::Smudge(i, j) = pattern.find_smudge(Orientation::Horizontal) {
                // println!("[{p}] xor {i},{j}");
                pattern.mirrors[(i, j)] ^= 1;
            } else if let Discovery::Smudge(i, j) = pattern.find_smudge(Orientation::Vertical) {
                // println!("[{p}] xor {i},{j}");
                pattern.mirrors[(i, j)] ^= 1;
            } else {
                // println!("no smudge found for {p}");
            }

            println!(
                "{p}: {:?}",
                pattern.reflection_line(Orientation::Horizontal)
            );
            println!("{p}: {:?}", pattern.reflection_line(Orientation::Vertical));

            if let Discovery::HorizontalLine(rows) =
                pattern.reflection_line(Orientation::Horizontal)
                && Discovery::HorizontalLine(rows) != solution1
            {
                // println!("[{p}] Part 2: Horizontal line at row {rows}");
                self.part2 += 100 * rows;
            } else if let Discovery::VerticalLine(cols) =
                pattern.reflection_line(Orientation::Vertical)
                && Discovery::VerticalLine(cols) != solution1
            {
                // println!("[{p}] Part 2: Vertical line at column {cols}");
                self.part2 += cols;
            } else {
                println!("no reflection line for {p}");
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
