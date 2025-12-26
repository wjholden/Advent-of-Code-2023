use advent_of_code_2023::*;
use nalgebra::{DMatrix, Vector2};
use pathfinding::prelude::dijkstra;

pub const PUZZLE: &str = include_str!("../../puzzles/day17.txt");

// This was a trickier one than I had expected, but unfortunately this may be
// more of a skill issue for my reading and not my computer science. I missed
// the critical detail that you don't include the loss at the starting vertex
// (2 in the example). I had also failed to account for the loss at the bottom-
// right corner (3 in the example), which produced the most maddening off-by-
// one error.
//
// Anyways, I restored to using a library Dijkstra solver for this. I'm fairly
// happy with the reduction, even if it wasn't necessary.
//
// The use of vectors was a little cumbersome. You'll eventually need usize to
// index into the input, so vectors gave me reasonably clean addition and
// turns.
//
// It looks like you can't gracefully match on Nalgebra's vectors. It should
// not be such a surprise, since they often contain lots of values that you
// wouldn't want to type out.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    blocks: DMatrix<u32>,
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Node {
    p: Vector2<i32>,
    d: Vector2<i32>,
    c: usize,
}

impl Puzzle {
    fn solve_with_pathfinding_library(&self, part: Part) -> u32 {
        let forward_limit = match part {
            Part::One => 3,
            Part::Two => 10,
        };
        let min_before_turns = match part {
            Part::One => 0,
            Part::Two => 4,
        };

        let start = Vector2::new(0, 0);
        // const LEFT: Vector2<i32> = Vector2::new(0, -1);
        const RIGHT: Vector2<i32> = Vector2::new(0, 1);
        const DOWN: Vector2<i32> = Vector2::new(1, 0);
        // const UP: Vector2<i32> = Vector2::new(-1, 0);
        let successors = |Node { p, d, c }: &Node| {
            let mut s = Vec::new();
            if self.blocks.get((p[0] as usize, p[1] as usize)).is_some() {
                let heat_loss = self.blocks[(p[0] as usize, p[1] as usize)];
                // starting position gets special treatment.
                if *p == start {
                    s.push((
                        Node {
                            p: p + RIGHT,
                            d: RIGHT,
                            c: 1,
                        },
                        0,
                    ));

                    s.push((
                        Node {
                            p: p + DOWN,
                            d: DOWN,
                            c: 1,
                        },
                        0,
                    ));
                } else {
                    if *c < forward_limit {
                        s.push((
                            Node {
                                p: p + d,
                                d: *d,
                                c: c + 1,
                            },
                            heat_loss,
                        ));
                    }
                    if *c >= min_before_turns {
                        let rotate_left = Vector2::new(-d[1], d[0]);
                        let rotate_right = Vector2::new(d[1], -d[0]);
                        s.push((
                            Node {
                                p: p + rotate_left,
                                d: rotate_left,
                                c: 1,
                            },
                            heat_loss,
                        ));
                        s.push((
                            Node {
                                p: p + rotate_right,
                                d: rotate_right,
                                c: 1,
                            },
                            heat_loss,
                        ));
                    }
                }
            }
            s.into_iter()
        };

        let target1 = Vector2::new(
            (self.blocks.nrows()) as i32,
            (self.blocks.ncols() - 1) as i32,
        );
        let target2 = Vector2::new(
            (self.blocks.nrows() - 1) as i32,
            (self.blocks.ncols()) as i32,
        );
        let goal = |node: &Node| node.p == target1 || node.p == target2;

        dijkstra(
            &Node {
                p: start,
                d: start,
                c: 0,
            },
            successors,
            goal,
        )
        .expect("path through graph")
        .1
    }
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();
        let blocks = DMatrix::from_row_iterator(
            rows,
            cols,
            input
                .lines()
                .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap())),
        );
        Self {
            part1: 0,
            part2: 0,
            blocks,
        }
    }

    fn solve(mut self) -> Self {
        self.part1 = self.solve_with_pathfinding_library(Part::One) as usize;
        self.part2 = self.solve_with_pathfinding_library(Part::Two) as usize;
        self
    }
}

#[cfg(test)]
mod clumsy_crucible {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day17.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 102);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 94);
    }
}
