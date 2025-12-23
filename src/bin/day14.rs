#[cfg(not(feature = "faster"))]
use std::collections::HashMap;

use advent_of_code_2023::*;
#[cfg(feature = "faster")]
use ndarray::Array2;

pub const PUZZLE: &str = include_str!("../../puzzles/day14.txt");

/// First time I've ever needed to use `move` in a closure.
///
/// Yay, I guess. This is kinda slow (not so bad with `--release`). These
/// automata puzzles are OK, I guess. Definitely resorted to scatterplots in
/// Excel for this one. I would have never guessed that the system descends to
/// some minimum before cycling.
///
/// Switching from `std::collections::HashMap` to `ndarray::Array2` with the
/// `faster` feature tag gives about a 10x speed boost (~3.6s and ~360ms).
/// Flamegraph helped me to identify that the original version spends a lot of
/// time just getting values in the HashMap. We don't need a sparse collection
/// for this problem.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Default, Debug, Clone)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    lines: usize,
    columns: usize,
    #[cfg(not(feature = "faster"))]
    rocks: HashMap<(usize, usize), Rock>,
    #[cfg(feature = "faster")]
    rocks: Array2<char>,
}

#[cfg(not(feature = "faster"))]
#[derive(Debug, Clone)]
enum Rock {
    Round,
    Cube,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Puzzle {
    #[cfg(not(feature = "faster"))]
    fn tilt(mut self, direction: Direction) -> (Self, usize) {
        let mut changes = 0;
        let next = |row, col| match direction {
            Direction::North => (row - 1, col),
            Direction::South => (row + 1, col),
            Direction::East => (row, col + 1),
            Direction::West => (row, col - 1),
        };
        let row_iter: Vec<usize> = match direction {
            Direction::North => (1..=self.lines).collect(),
            Direction::South => (0..self.lines).rev().collect(),
            Direction::East => (0..=self.lines).rev().collect(),
            Direction::West => (0..=self.lines).collect(),
        };
        let col_iter: Vec<usize> = match direction {
            Direction::North => (0..=self.columns).collect(),
            Direction::South => (0..=self.columns).rev().collect(),
            Direction::East => (0..self.columns).rev().collect(),
            Direction::West => (1..=self.columns).rev().collect(),
        };
        for &row in row_iter.iter() {
            for &col in col_iter.iter() {
                let current_position = (row, col);
                let next_position = next(row, col);

                if let Some(Rock::Round) = self.rocks.get(&current_position)
                    && !self.rocks.contains_key(&next_position)
                {
                    self.rocks.remove(&current_position);
                    self.rocks.insert(next_position, Rock::Round);
                    changes += 1;
                }
            }
        }
        (self, changes)
    }

    #[cfg(feature = "faster")]
    fn tilt(mut self, direction: Direction) -> (Self, usize) {
        let mut changes = 0;
        let next = |row, col| match direction {
            Direction::North => (row - 1, col),
            Direction::South => (row + 1, col),
            Direction::East => (row, col + 1),
            Direction::West => (row, col - 1),
        };
        let row_iter: Vec<usize> = match direction {
            Direction::North => (1..self.lines).collect(),
            Direction::South => (0..self.lines - 1).rev().collect(),
            Direction::East => (0..self.lines).rev().collect(),
            Direction::West => (0..self.lines).collect(),
        };
        let col_iter: Vec<usize> = match direction {
            Direction::North => (0..self.columns).collect(),
            Direction::South => (0..self.columns).rev().collect(),
            Direction::East => (0..self.columns - 1).rev().collect(),
            Direction::West => (1..self.columns).rev().collect(),
        };
        for &row in row_iter.iter() {
            for &col in col_iter.iter() {
                let current_position = (row, col);
                let next_position = next(row, col);
                if self.rocks[current_position] == 'O' && self.rocks[next_position] == '.' {
                    self.rocks[current_position] = '.';
                    self.rocks[next_position] = 'O';
                    changes += 1;
                }
            }
        }
        (self, changes)
    }

    #[cfg(not(feature = "faster"))]
    fn load(&self) -> usize {
        self.rocks
            .iter()
            .map(|((row, _), rock)| match rock {
                Rock::Round => self.lines - row + 1,
                Rock::Cube => 0,
            })
            .sum()
    }

    #[cfg(feature = "faster")]
    fn load(&self) -> usize {
        self.rocks
            .indexed_iter()
            .map(|((i, _), v)| match v {
                'O' => self.lines - i,
                _ => 0,
            })
            .sum()
    }

    fn spin(mut self) -> Self {
        for direction in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            let mut changes;
            loop {
                (self, changes) = self.tilt(direction);
                if changes == 0 {
                    break;
                }
            }
        }
        self
    }
}

#[cfg(not(feature = "faster"))]
impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} rows and {} columns", self.lines, self.columns)?;
        for row in 0..=self.lines {
            for col in 0..=self.columns {
                write!(
                    f,
                    "{}",
                    match self.rocks.get(&(row, col)) {
                        Some(Rock::Round) => "O",
                        Some(Rock::Cube) => "#",
                        None => ".",
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(feature = "faster")]
impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} rows and {} columns", self.lines, self.columns)?;
        for row in 0..self.lines {
            for col in 0..self.columns {
                write!(f, "{}", self.rocks[(row, col)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let mut instance = Self::default();

        #[cfg(feature = "faster")]
        {
            use ndarray::Array;

            let rows = input.lines().count();
            let a = Array::from_iter(input.chars().filter(|&c| c == 'O' || c == '#' || c == '.'));
            let cols = a.len() / rows;
            assert_eq!(rows * cols, a.len());
            let a = a.into_shape_with_order((rows, cols)).unwrap();
            instance.rocks = a;
            instance.lines = rows;
            instance.columns = cols;
        }

        #[cfg(not(feature = "faster"))]
        {
            instance.rocks = input
                .lines()
                .enumerate()
                .flat_map(|(i, line)| {
                    instance.lines = instance.lines.max(i);
                    line.char_indices().filter_map(move |(j, c)| match c {
                        'O' => Some(((i, j), Rock::Round)),
                        '#' => Some(((i, j), Rock::Cube)),
                        '.' => None,
                        _ => panic!("unexpected input character"),
                    })
                })
                .collect();
            instance.columns = instance.rocks.keys().fold(0, |c, &(_, j)| c.max(j));
        }
        instance
    }

    fn solve(mut self) -> Self {
        let clone = self.clone();

        println!("{self}");
        let mut changes;
        loop {
            (self, changes) = self.tilt(Direction::North);
            if changes == 0 {
                break;
            }
        }
        let part1 = self.load();
        println!("{self}");
        self = clone;
        self.part1 = part1;

        let mut cycles = 0;
        let mut min = usize::MAX;

        // Walk down to the minimum "load" value.
        loop {
            self = self.spin();
            cycles += 1;
            let load = self.load();
            if load == min && cycles > 10 {
                break;
            } else if load < min {
                min = load;
            }
        }

        // Now that we've found the minimum, walk the cycle again for the period length.
        let mut period_length = 0;
        loop {
            self = self.spin();
            period_length += 1;
            cycles += 1;
            if min == self.load() {
                break;
            }
        }

        // Count up yet again until we find a modular equivalence to the target.
        while (cycles % period_length) != (1000000000 % period_length) {
            self = self.spin();
            cycles += 1;
        }

        self.part2 = self.load();

        self
    }
}

#[cfg(test)]
mod puzzle_name {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day14.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 136);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 64);
    }
}
