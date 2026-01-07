use std::collections::BTreeMap;

use advent_of_code_2023::*;
use itertools::Itertools;
use nalgebra::{DMatrix, DVector};

pub const PUZZLE: &str = include_str!("../../puzzles/day22.txt");

fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1.unwrap()); // 480 too low.
    //println!("Part 2: {}", d.part2.unwrap());
    //println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: Option<usize>,
    pub part2: Option<usize>,
    bricks: Vec<Brick>,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug)]
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
    fn fall_top_down(&mut self) {
        let max_x = self.bricks.iter().map(|b| b.end.x).max().unwrap();
        let max_y = self.bricks.iter().map(|b| b.end.y).max().unwrap();
        let mut height_matrix: DMatrix<usize> = DMatrix::zeros(max_x + 1, max_y + 1);
        println!("Initial heights: {height_matrix}");
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
            brick.start.z = brick.start.z - shift;
            brick.end.z = brick.end.z - shift;
            for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
                height_matrix[(x, y)] = brick.end.z;
            }
            println!("New heights: {height_matrix}");
        }
    }

    /// This technique iterates over all of the (sorted) bricks. It identifies
    /// the tallest z of any brick that would intersect the current brick in
    /// the x-y plane and shifts the brick down to that z+1.
    ///
    /// This algorithm passes the test case, but fails on the real input. I
    /// suspect that there could be overlapping bricks in here due to the
    /// partial order. You can have a brick at z1..z2 and another brick at
    /// z3..z4 where z1<z3<z4<z2.
    fn fall_sorted(&mut self) {
        for i in 1..self.bricks.len() {
            let (left, right) = self.bricks.split_at_mut(i);
            let brick = &mut right[0];
            // We have a brick above z=1. Find the highest thing directly beneath it.
            if let Some(tallest_z_under) = left
                .iter()
                .filter_map(|other| {
                    if brick.intersects_in_xy_plane(other) {
                        Some(other.end.z)
                    } else {
                        None
                    }
                })
                .max()
                && brick.start.z > tallest_z_under + 1
            {
                // println!("The highest brick under {brick:?} is at z={tallest_z_under}.");
                let shift = brick.start.z - tallest_z_under - 1;
                // println!("I can shift by {shift}.");
                brick.start.z -= shift;
                brick.end.z -= shift;
            }
        }
    }

    fn disintegratable_bricks(&self) -> Vec<&Brick> {
        let mut solution = vec![];
        let max_z = self.max_z_start();
        for z in 1..max_z {
            println!("====== z = {z} ======");
            // 1) Lower <- find all bricks at max z
            // 2) Upper <- find all bricks at min z
            // 3) For each brick in lower, find the x/y
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

            let support_matrix = DMatrix::from_fn(upper.len(), lower.len(), |i, j| {
                upper[i].intersects_in_xy_plane(lower[j]) as u8
            });
            // let support_matrix = Array::from_shape_fn((lower.len(), upper.len()), |(i, j)| {
            //     lower[i].intersects_in_xy_plane(upper[j]) as u8
            // });
            // println!("Lower ({}): {lower:?}", lower.len());
            // println!("Upper ({}): {upper:?}", upper.len());
            println!("Support: {support_matrix}");
            for i in 0..lower.len() {
                // let selection = Array::from_shape_fn(upper.len(), |j| if j == i { 0 } else { 1 });
                let selection = DVector::from_fn(lower.len(), |j, _| if j == i { 0 } else { 1 });
                // println!("Selection vector: {selection}");
                let product = &support_matrix * &selection;
                // println!("Support if disintegrated: {product}");
                if !product.iter().contains(&0) {
                    println!("{:?} can be disintegrated.", lower[i]);
                    solution.push(lower[i]);
                } else {
                    println!("{:?} CANNOT be disintegrated.", lower[i]);
                }
            }
        }
        // All bricks at the top are solutions as well.
        for top in self.bricks.iter().filter(|brick| brick.start.z >= max_z) {
            solution.push(top);
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
        // self.fall_sorted();
        // self.part1 = Some(self.disintegratable_bricks().len());
        self.fall_top_down();
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
}
