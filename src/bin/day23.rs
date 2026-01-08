use std::collections::HashSet;

use advent_of_code_2023::*;
use ndarray::{Array, Array2};

pub const PUZZLE: &str = include_str!("../../puzzles/day23.txt");

fn main() {
    let d = Puzzle::new(PUZZLE);
    println!("{} intersections", d.intersections.len());
    // let d = d.solve();
    // println!("Part 1: {}", d.part1.unwrap());
    //println!("Part 2: {}", d.part2.unwrap());
    //println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: Option<usize>,
    pub part2: Option<usize>,
    map: Array2<Tile>,
    intersections: HashSet<(usize, usize)>,
}

#[derive(Debug, Clone)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Debug, Clone)]
enum Tile {
    Path,
    Slope(Direction),
    Forest,
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '.' => Self::Path,
            '#' => Self::Forest,
            '>' => Self::Slope(Direction::East),
            '<' => Self::Slope(Direction::West),
            'v' => Self::Slope(Direction::South),
            // '^' is mentioned in the puzzle text, but it not in the input.
            _ => panic!("unrecognized symbol"),
        }
    }
}

impl Puzzle {
    fn explore(&self) {}
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let v: Vec<_> = input
            .lines()
            .flat_map(|line| line.chars().map(Tile::new))
            .collect();
        let rows = input.lines().count();
        let cols = v.len() / rows;
        assert_eq!(rows * cols, v.len());
        let map = Array::from_shape_vec((rows, cols), v).expect("puzzle map");

        let lines: Vec<_> = input.lines().collect();
        let mut intersections = HashSet::new();
        // These patterns almost work, but notice the 3rd and 4th are the same.
        // Where does the v go? Is it in the second or fourth position?
        // We need to look ahead in the input lines to find out.
        // (.>.>.)|(..>.#)|(#.>.#)|(#.>.#)|(#.>..)
        let signatures = ["#.>", ">.>", ">.#"];
        for (i, line) in lines.iter().enumerate() {
            for signature in signatures.iter() {
                for (j, m) in line.match_indices(signature) {
                    if lines[i + 1].as_bytes()[j + 1] == 'v' as u8
                        || lines[i - 1].as_bytes()[j + 1] == 'v' as u8
                    {
                        println!("matched {m} at {i},{j}");
                        intersections.insert((i, j + 1));
                    }
                }
            }
        }

        Self {
            part1: None,
            part2: None,
            map,
            intersections,
        }
    }

    fn solve(mut self) -> Self {
        println!(
            "there are {} slope tiles",
            self.map
                .iter()
                .filter(|tile| matches!(tile, Tile::Slope(_)))
                .count()
        );

        println!("there are {} intersections", self.intersections.len());
        println!("{:?}", self.intersections);
        self
    }
}

#[cfg(test)]
mod a_long_walk {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day23.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, Some(94));
    }

    #[test]
    fn intersections() {
        let puzzle = Puzzle::new(SAMPLE);
        println!("{:?}", puzzle.intersections);
        assert_eq!(puzzle.intersections.len(), 7);
    }
}
