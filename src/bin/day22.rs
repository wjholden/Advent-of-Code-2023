use std::collections::BTreeMap;

use advent_of_code_2023::*;
use itertools::Itertools;
use nalgebra::{DMatrix, DVector};
use rayon::iter::{ParallelBridge, ParallelIterator};

pub const PUZZLE: &str = include_str!("../../puzzles/day22.txt");

/// Surprisingly tractable! This puzzle took me a few days. I had thought that
/// it would be too computationally expensive to do the obvious thing. I had
/// envisioned a tricky dependency graph of stacked bricks, but it turns out
/// you can just clone the whole thing, delete a brick, and see how many this
/// moves.
///
/// They say that if it's stupid but it works, then it isn't stupid...
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1.unwrap());
    println!("Part 2: {}", d.part2.unwrap());
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub part1: Option<usize>,
    pub part2: Option<usize>,
    bricks: Vec<Brick>,
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone)]
struct Brick {
    start: Point,
    end: Point,
}

impl Brick {
    fn new(line: &str) -> Self {
        let tokens: Vec<usize> = line
            .split(['~', ','])
            .flat_map(str::parse::<usize>)
            .collect();
        assert_eq!(tokens.len(), 6);

        // This is actually given in the prompt, but I think it's a good
        // practice to test our assumptions.
        //
        // ```mathematica
        // In[1]:= LogicalExpand[(! a && ! b && ! c) || (a && ! b && ! c) || (! a && b && ! c) || (! a && ! b && c)]
        // Out[1]= (!a && !b) || (!a && !c) || (!b && !c)
        // ```
        let a = tokens[0] != tokens[3];
        let b = tokens[1] != tokens[4];
        let c = tokens[2] != tokens[5];
        let zero = !a && !b && !c;
        let one = (a && !b && !c) || (!a && b && !c) || (!a && !b && c);
        assert!(zero || one);

        // This assumption might not be stated, but it gives us a small
        // optimization later for sorting the input by starting z values.
        // We don't need to worry about the ending z values.
        assert!(tokens[0] <= tokens[3]);
        assert!(tokens[1] <= tokens[4]);
        assert!(tokens[2] <= tokens[5]);

        Self {
            start: Point {
                x: tokens[0],
                y: tokens[1],
                z: tokens[2],
            },
            end: Point {
                x: tokens[3],
                y: tokens[4],
                z: tokens[5],
            },
        }
    }

    fn volume(&self) -> usize {
        1 + self.start.x.abs_diff(self.end.x)
            + self.start.y.abs_diff(self.end.y)
            + self.start.z.abs_diff(self.end.z)
    }

    fn intersects_in_xy_plane(&self, other: &Self) -> bool {
        let Point { x: x1, y: y1, z: _ } = self.start;
        let Point { x: x2, y: y2, z: _ } = self.end;
        let Point { x: x3, y: y3, z: _ } = other.start;
        let Point { x: x4, y: y4, z: _ } = other.end;
        let x_overlap_1 = x1 <= x3 && x3 <= x2;
        let x_overlap_2 = x1 <= x4 && x3 <= x2;
        let y_overlap_1 = y1 <= y3 && y3 <= y2;
        let y_overlap_2 = y1 <= y4 && y3 <= y2;
        (x_overlap_1 || x_overlap_2) && (y_overlap_1 || y_overlap_2)
    }
}

impl Puzzle {
    fn fall_top_down(&mut self) -> usize {
        let mut moves = 0;
        let max_x = self.bricks.iter().map(|b| b.end.x).max().unwrap();
        let max_y = self.bricks.iter().map(|b| b.end.y).max().unwrap();
        let mut height_matrix: DMatrix<usize> = DMatrix::zeros(max_x + 1, max_y + 1);
        // println!("Initial heights: {height_matrix}");
        for brick in self.bricks.iter_mut() {
            let Brick {
                start: Point { x: x1, y: y1, z: _ },
                end: Point { x: x2, y: y2, z: _ },
            } = *brick;
            let zh = (x1..=x2)
                .cartesian_product(y1..=y2)
                .map(|(x, y)| height_matrix[(x, y)])
                .max()
                .expect("maximum height under current brick");
            let distance = brick.start.z - zh;
            let shift = distance - 1;
            if shift > 0 {
                brick.start.z = brick.start.z - shift;
                brick.end.z = brick.end.z - shift;
                moves += 1;
            }
            for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
                height_matrix[(x, y)] = brick.end.z;
            }
            // println!("New heights: {height_matrix}");
        }
        moves
    }

    fn disintegratable_bricks(&self) -> Vec<&Brick> {
        let mut solution = vec![];
        let max_z = self.max_z_start();

        // All bricks at the top are solutions for part 1.
        // None of these bricks support others.
        for top in self.bricks.iter().filter(|brick| brick.start.z >= max_z) {
            solution.push(top);
        }

        for z in (1..max_z).rev() {
            let lower: Vec<&Brick> = self
                .bricks
                .iter()
                .filter(|brick| brick.end.z == z)
                .collect();
            let upper: Vec<&Brick> = self
                .bricks
                .iter()
                .filter(|brick| brick.start.z == z + 1)
                .collect();
            assert!(!lower.is_empty());
            assert!(!upper.is_empty());

            // A little backwards from what you might expect: rows are the
            // upper bricks, columns are the lower bricks. We will multiply by
            // a vector of all-but-one ones. The single zero in our vector
            // is the lower brick we are removing.
            let support_matrix = DMatrix::from_fn(upper.len(), lower.len(), |i, j| {
                upper[i].intersects_in_xy_plane(lower[j]) as u8
            });

            for (i, _brick) in lower.iter().enumerate() {
                let selection = DVector::from_fn(lower.len(), |j, _| if j == i { 0 } else { 1 });
                let product = &support_matrix * &selection;
                // If the matrix product produces a zero, this means the
                // corresponding upper brick is now supported by zero lower
                // bricks. There might have been a more direct way to do
                // this, but the linear combination works.
                if !product.iter().contains(&0) {
                    // Here, the product does *not* contain a zero, so we know
                    // it is safe to disintegrate this lower brick.
                    solution.push(lower[i]);
                }
            }
        }

        solution
    }

    fn max_z_start(&self) -> usize {
        self.bricks.iter().map(|brick| brick.start.z).max().unwrap()
    }

    #[allow(dead_code)]
    fn tally(&self) {
        println!(
            "{:?} ({} total)",
            self.bricks
                .iter()
                .fold(BTreeMap::new(), |mut acc: BTreeMap<usize, usize>, brick| {
                    *acc.entry(brick.volume()).or_default() += 1;
                    acc
                }),
            self.bricks.len()
        );
    }
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let mut bricks: Vec<_> = input.lines().map(Brick::new).collect();
        bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
        Self {
            part1: None,
            part2: None,
            bricks,
        }
    }

    fn solve(mut self) -> Self {
        self.fall_top_down();
        // let (part1, part2) = self.disintegratable_bricks();
        // self.part1 = Some(part1.len());
        // self.part2 = Some(part2);
        let can_delete = self.disintegratable_bricks();
        self.part1 = Some(can_delete.len());
        self.part2 = Some(
            (0..self.bricks.len())
                .par_bridge()
                .map(|i| {
                    let mut tmp = self.clone();
                    tmp.bricks.remove(i);
                    tmp.fall_top_down()
                })
                .sum(),
        );
        self
    }
}

#[cfg(test)]
mod sand_slabs {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day22.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, Some(5));
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, Some(7));
    }
}
