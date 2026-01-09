use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
};

use advent_of_code_2023::*;
use nalgebra::DMatrix;
use ndarray::{Array, Array2};

pub const PUZZLE: &str = include_str!("../../puzzles/day23.txt");

/// Extremely hard problem. Yet another puzzle where we need to recognize a
/// special case of an otherwise intractable problem.
///
/// At least, I think.
///
/// There is not, to my knowledge, an efficient algorithm to find the longest
/// path in a graph. Fortunately, *this* graph forms a DAG in part 1, so we
/// can quickly solve it with DFS.
///
/// For part 2, we turn the DAG into an undirected graph by adding the adjacency
/// matrix to its own transpose. We must maintain a history of the nodes we've
/// visited, otherwise we will get caught in an endless loop.
///
/// Not my fastest AoC program. I will be interested to see what others did to
/// make this quicker.
///
/// The object-oriented approach was not very helpful for this one. This was
/// more about pure functions that I can easily test.
///
/// See https://youtu.be/NTLYL7Mg2jU for a compact Python solution by HyperNeutrino.
fn main() {
    let d = Puzzle::new(PUZZLE).solve();
    println!("Part 1: {}", d.part1.unwrap());
    println!("Part 2: {}", d.part2.unwrap());
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: Option<usize>,
    pub part2: Option<usize>,
    map: Array2<Tile>,
    intersections: Vec<(usize, usize)>,
    start: (usize, usize),
    goal: (usize, usize),
}

#[derive(Debug, Clone)]
// Though the instructions mention moving on slopes facing < and ^, these
// directions don't actually exist in the sample nor puzzle input.
enum Direction {
    East,
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
            'v' => Self::Slope(Direction::South),
            _ => panic!("unrecognized symbol"),
        }
    }
}

#[cfg(not(feature = "faster"))]
fn longest_path(g: &DMatrix<usize>) -> Option<usize> {
    let path = RefCell::new(vec![]);
    let solutions = RefCell::new(Vec::new());
    longest_path_internal(&g, 0, g.nrows() - 1, &path, &solutions);
    debug_assert!(path.borrow().is_empty());
    solutions
        .borrow()
        .iter()
        .map(|v| {
            use itertools::Itertools;

            v.iter()
                .tuple_windows()
                .fold(0, |acc, (i, j)| acc + 1 + g[(*i, *j)])
        })
        .max()
}

#[cfg(feature = "faster")]
fn longest_path(g: &DMatrix<usize>) -> Option<usize> {
    let path = RefCell::new(Vec::new());
    longest_path_internal(&g, 0, g.nrows() - 1, &path)
}

#[cfg(not(feature = "faster"))]
/// This version of the algorithm writes all of the candidate solutions to a
/// vector. We find the longest path later, after they're all in here. Not an
/// artistic solution, but this is probably the only way to print the optimal
/// solution if you wanted to see it.
fn longest_path_internal(
    g: &DMatrix<usize>,
    src: usize,
    dst: usize,
    path: &RefCell<Vec<usize>>,
    solutions: &RefCell<Vec<Vec<usize>>>,
) {
    debug_assert!(!path.borrow().contains(&src));
    path.borrow_mut().push(src);

    if src == dst {
        use itertools::Itertools;

        debug_assert_eq!(
            path.borrow().iter().sorted().dedup().count(),
            path.borrow().len()
        );
        solutions.borrow_mut().push(path.borrow().to_vec());
    } else {
        for j in 0..g.ncols() {
            if let Some(w) = g.get((src, j))
                && *w > 0
                && !path.borrow().contains(&j)
            {
                longest_path_internal(g, j, dst, path, solutions);
            }
        }
    }
    let out = path.borrow_mut().pop();
    debug_assert_eq!(out, Some(src));
}

#[cfg(feature = "faster")]
/// This version implicitly maintains the solution on the stack with pure
/// recursion. More memory efficient than the other algorithm and slightly, but
/// not massively, faster.
fn longest_path_internal(
    g: &DMatrix<usize>,
    src: usize,
    dst: usize,
    path: &RefCell<Vec<usize>>,
) -> Option<usize> {
    if src == dst {
        Some(0)
    } else {
        let mut distance: Option<usize> = None;
        debug_assert!(!path.borrow().contains(&src));
        path.borrow_mut().push(src);
        for j in 0..g.ncols() {
            if let Some(w) = g.get((src, j))
                && *w > 0
                && !path.borrow().contains(&j)
            {
                if let Some(candidate) = longest_path_internal(g, j, dst, path) {
                    let candidate_distance = 1 + w + candidate;
                    if let Some(current) = distance {
                        distance = Some(current.max(candidate_distance));
                    } else {
                        distance = Some(candidate_distance);
                    }
                }
            }
        }
        let out = path.borrow_mut().pop();
        debug_assert_eq!(out, Some(src));
        distance
    }
}

impl Puzzle {
    fn to_graph(&self) -> DMatrix<usize> {
        let mut g = DMatrix::zeros(self.intersections.len(), self.intersections.len());
        for (u, intersection) in self.intersections.iter().enumerate() {
            if *intersection == self.goal {
                continue;
            }
            let directions = if *intersection == self.start {
                &[(0, 0)][..]
            } else {
                &[(1, 0), (0, 1)][..]
            };
            for &(dr, dc) in directions {
                let r = intersection.0.wrapping_add_signed(dr);
                let c = intersection.1.wrapping_add_signed(dc);
                match self.map.get((r, c)) {
                    Some(Tile::Slope(Direction::East) | Tile::Slope(Direction::South)) => {
                        let (w, dst) = self.explore_edge(&(r, c));
                        let v = self.intersections.iter().position(|&v| v == dst).unwrap();
                        g[(u, v)] = w;
                    }
                    Some(Tile::Path) => {
                        // Special case of the starting vertex. Verify this
                        // assumption before we do the same thing as above.
                        debug_assert!(self.intersections.contains(&(r, c)));
                        let (w, dst) = self.explore_edge(&(r, c));
                        let v = self.intersections.iter().position(|&v| v == dst).unwrap();
                        // We didn't shift the start, so we need to subtract here
                        // to avoid double-counting the start tile.
                        g[(u, v)] = w - 1;
                    }
                    Some(Tile::Forest) | None => {}
                }
            }
        }
        g
    }

    fn explore_edge(&self, start: &(usize, usize)) -> (usize, (usize, usize)) {
        let mut elements = HashSet::new();
        let mut queue = VecDeque::from([*start]);
        let mut dst = None;
        while let Some(current) = queue.pop_front() {
            elements.insert(current);
            let directions = match self.map[current] {
                Tile::Path => &[(-1, 0), (1, 0), (0, -1), (0, 1)][..],
                Tile::Slope(Direction::East) => &[(0, 1)][..],
                Tile::Slope(Direction::South) => &[(1, 0)][..],
                Tile::Forest => unreachable!(),
            };
            for &(dr, dc) in directions {
                let r = current.0.wrapping_add_signed(dr);
                let c = current.1.wrapping_add_signed(dc);
                if let Some(tile) = self.map.get((r, c))
                    && matches!(tile, Tile::Path | Tile::Slope(_))
                    && !elements.contains(&(r, c))
                {
                    if self.intersections.contains(&(r, c)) {
                        debug_assert!(dst.is_none());
                        dst = Some((r, c));
                    } else {
                        queue.push_back((r, c));
                    }
                }
            }
            debug_assert!(queue.len() <= 1);
        }
        (elements.len(), dst.unwrap())
    }
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
        let mut intersections = Vec::new();
        // These patterns almost work, but notice the 3rd and 4th are the same.
        // Where does the v go? Is it in the second or fourth position?
        // We need to look ahead in the input lines to find out.
        // (.>.>.)|(..>.#)|(#.>.#)|(#.>.#)|(#.>..)
        let signatures = ["#.>", ">.>", ">.#"];
        for (i, line) in lines.iter().enumerate() {
            for signature in signatures.iter() {
                for (j, _m) in line.match_indices(signature) {
                    if lines[i + 1].as_bytes()[j + 1] == 'v' as u8
                        || lines[i - 1].as_bytes()[j + 1] == 'v' as u8
                    {
                        // println!("matched {m} at {i},{j}");
                        intersections.push((i, j + 1));
                    }
                }
            }
        }

        // Also add origin and end to intersections.
        intersections.insert(0, (0, 1));
        intersections.push((rows - 1, cols - 2));
        // assert!(matches!(map[(rows - 1, cols - 2)], Tile::Path));

        Self {
            part1: None,
            part2: None,
            map,
            intersections,
            start: (0, 1),
            goal: (rows - 1, cols - 2),
        }
    }

    fn solve(mut self) -> Self {
        let g = self.to_graph();
        // println!("{g}");
        self.part1 = longest_path(&g);

        // The matrix is a DAG...right?
        debug_assert_eq!(g.component_mul(&g.transpose()).sum(), 0);

        let g2 = &g + &g.transpose();
        // println!("{g2}");
        self.part2 = longest_path(&g2);

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
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, Some(154));
    }

    #[test]
    fn intersections() {
        let puzzle = Puzzle::new(SAMPLE);
        println!("{:?}", puzzle.intersections);
        assert_eq!(puzzle.intersections.len(), 9);
    }
}
