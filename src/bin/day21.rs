use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display};

use advent_of_code_2023::*;
use ndarray::Array2;
use num::Integer;

pub const PUZZLE: &str = include_str!("../../puzzles/day21.txt");

/// I needed help with this one. I was pretty close with my model of the
/// problem, but I had made a critical error in my understanding of the
/// corners. I had thought that you could "donate" included corners on one side
/// to those excluded corners on the other side, but because they have
/// different parities (one is tiles that are reachable by even numbers of
/// steps, the other odd) you can't do this donation thing.
///
/// The math is surprisingly tricky.
///
/// https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
/// https://www.reddit.com/r/adventofcode/comments/18nol3m/2023_day_21_a_geometric_solutionexplanation_for/
///
/// No regrets on using ndarray, binary heap, and OOP on this one. A small
/// takeaway is that `matches!` might have some limitations for dynamic data
/// after the first argument.
///
/// Couldn't get the provided test cases to work but oh well, it's day 21.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1.unwrap());
    println!("Part 2: {}", d.part2.unwrap());
    // 615601255180299 too high
    // 615595169187099 too high
    // 611181302011715 too low
    // 613391278595915 wrong (odd+even squares, odd diamond)
    // 613391278596099 wrong (odd+even squares, even diamond)
    // 613391267671715 wrong (biased odd/even squares where 0 is odd, odd diamond)
    // 613391241380619 wrong (Villuna's method)
    // 613391241372715 wrong (Villuna's method, corrected but still wrong)
    // 613391241979618 wrong, even with new corners technique.
    // 613391294577878 (finally!)
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug, Clone)]
enum State {
    Odd,
    Even,
    Unknown,
}

#[derive(Debug, Clone)]
enum Plot {
    Garden(State),
    Rock,
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: Option<usize>,
    pub part2: Option<usize>,
    start_position: (usize, usize),
    array: Array2<Plot>,
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for inner in self.array.outer_iter() {
            for plot in inner {
                match plot {
                    Plot::Garden(State::Unknown) => write!(f, ".")?,
                    Plot::Garden(State::Odd) => write!(f, "O")?,
                    Plot::Garden(State::Even) => write!(f, "E")?,
                    Plot::Rock => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Puzzle {
    fn reset(&mut self) {
        for plot in self.array.iter_mut() {
            if let Plot::Garden(state) = plot
                && matches!(state, State::Even | State::Odd)
            {
                *state = State::Unknown;
            }
        }
    }

    fn explore(&mut self, step_goal: usize, start_position: (usize, usize)) {
        let mut queue: BinaryHeap<Reverse<(usize, (usize, usize))>> = BinaryHeap::new();
        queue.push(Reverse((0, start_position)));
        while let Some(Reverse((steps, (row, col)))) = queue.pop() {
            match self.array[(row, col)] {
                Plot::Garden(State::Unknown) => {
                    self.array[(row, col)] = Plot::Garden(if steps.is_odd() {
                        State::Odd
                    } else {
                        State::Even
                    });
                    if steps < step_goal {
                        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let next_pos =
                                (row.wrapping_add_signed(dr), col.wrapping_add_signed(dc));
                            if let Some(next) = self.array.get(next_pos)
                                && matches!(next, Plot::Garden(State::Unknown))
                            {
                                queue.push(Reverse((steps + 1, next_pos)));
                            }
                        }
                    }
                }
                Plot::Garden(State::Odd) => assert!(steps.is_odd()),
                Plot::Garden(State::Even) => assert!(steps.is_even()),
                Plot::Rock => unreachable!(),
            }
        }
    }

    /// Looks like `matches!` has some limitation with dynamic data after the
    /// first argument.
    fn count_even(&self) -> usize {
        self.array
            .iter()
            .filter(|plot| matches!(plot, Plot::Garden(State::Even)))
            .count()
    }

    fn count_odd(&self) -> usize {
        self.array
            .iter()
            .filter(|plot| matches!(plot, Plot::Garden(State::Odd)))
            .count()
    }
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let mut start_position = None;
        let data = input.lines().flat_map(|line| {
            line.chars().map(|c| match c {
                '.' | 'S' => Plot::Garden(State::Unknown),
                '#' => Plot::Rock,
                _ => unreachable!(),
            })
        });
        for (r, line) in input.lines().enumerate() {
            if let Some(c) = line.chars().position(|c| c == 'S') {
                start_position = Some((r, c));
                break;
            }
        }
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();
        // See https://docs.rs/ndarray/latest/ndarray/struct.ArrayBase.html.
        let array = Array2::from_shape_vec((rows, cols), data.collect()).unwrap();
        Self {
            part1: None,
            part2: None,
            start_position: start_position.unwrap(),
            array,
        }
    }

    fn solve(mut self) -> Self {
        let step_goal;
        let step_goal2 = 26501365;
        #[cfg(test)]
        {
            step_goal = 6;
        }
        #[cfg(not(test))]
        {
            step_goal = 64;
        }

        self.explore(step_goal, self.start_position);
        self.part1 = Some(self.count_even());

        self.reset();
        // 65 steps. This is the distance we need to go to escape the center
        // square. I think.
        self.explore(step_goal2 % self.array.nrows(), self.start_position);
        let diamond_odd = self.count_odd();
        let diamond_even = self.count_even();
        // println!("{self}");

        self.reset();
        self.explore(self.array.ncols(), self.start_position);
        let odd_square = self.count_odd();
        let even_square = self.count_even();
        let odd_corners = odd_square - diamond_odd;
        let even_corners = even_square - diamond_even;

        let radius = step_goal2 / self.array.ncols();
        let even_tiles = radius.pow(2);
        let odd_tiles = (radius + 1).pow(2);
        #[cfg(not(test))]
        {
            // Alternative ways to count the odd, even, and total tiles.
            assert_eq!(even_tiles, (1..=radius).step_by(2).sum::<usize>() * 4);
            assert_eq!(odd_tiles, 1 + (2..=radius).step_by(2).sum::<usize>() * 4);
            let tiles = |radius: usize| 1 + 4 * radius * (radius + 1) / 2; // +1 to include the center square.
            assert_eq!(even_tiles + odd_tiles, tiles(radius));
        }

        self.part2 = Some(
            odd_tiles * odd_square + even_tiles * even_square - (radius + 1) * odd_corners
                + radius * even_corners,
        );
        self
    }
}

#[cfg(test)]
mod step_counter {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day21.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, Some(16));
    }
}
