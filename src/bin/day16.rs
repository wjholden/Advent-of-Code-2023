use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code_2023::*;
use itertools::Itertools;
use num::Complex;

pub const PUZZLE: &str = include_str!("../../puzzles/day16.txt");

fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug)]
enum Item {
    MirrorR,
    MirrorL,
    HSplit,
    VSplit,
}

impl Item {
    fn from(c: char) -> Self {
        match c {
            '/' => Item::MirrorR,
            '\\' => Item::MirrorL,
            '-' => Item::HSplit,
            '|' => Item::VSplit,
            _ => panic!("unexpected input character"),
        }
    }
}

#[derive(Default, Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    items: HashMap<Complex<isize>, Item>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct State {
    position: Complex<isize>,
    direction: Complex<isize>,
}

impl Puzzle {
    fn more_energy_more_passion(&self) -> usize {
        let mut max_energy = 0;
        let south = Complex::new(0, -1);
        let north = Complex::new(0, 1);
        let west = Complex::new(-1, 0);
        let east = Complex::new(1, 0);
        for (x, y) in (self.min_x..=self.max_x).cartesian_product(self.min_y..=self.max_y) {
            let position = Complex::new(x, y);
            if x == self.min_x {
                max_energy = max_energy.max(self.energize(State {
                    position,
                    direction: east,
                }));
            }
            if x == self.max_x {
                max_energy = max_energy.max(self.energize(State {
                    position,
                    direction: west,
                }));
            }
            if y == self.min_y {
                max_energy = max_energy.max(self.energize(State {
                    position,
                    direction: north,
                }));
            }
            if y == self.max_y {
                max_energy = max_energy.max(self.energize(State {
                    position,
                    direction: south,
                }));
            }
        }
        max_energy
    }

    fn energize(&self, start: State) -> usize {
        let mut history = HashSet::<State>::new();
        let mut frontier = VecDeque::new();

        frontier.push_front(start);

        while let Some(state) = frontier.pop_front() {
            if state.position.re < self.min_x
                || state.position.re > self.max_x
                || state.position.im < self.min_y
                || state.position.im > self.max_y
            {
                continue;
            }
            history.insert(state);
            let directions = match self.items.get(&state.position) {
                None => vec![state.direction],
                Some(Item::VSplit) if state.direction.re == 0 => vec![state.direction],
                Some(Item::HSplit) if state.direction.im == 0 => vec![state.direction],
                Some(Item::VSplit | Item::HSplit) => {
                    vec![state.direction * Complex::I, state.direction * -Complex::I]
                }
                Some(Item::MirrorL) => vec![Complex::<isize>::I / state.direction],
                Some(Item::MirrorR) => vec![-Complex::<isize>::I / state.direction],
            };
            for direction in directions {
                let new_state = State {
                    position: state.position + direction,
                    direction,
                };
                if !history.contains(&new_state) {
                    frontier.push_back(new_state);
                }
            }
        }

        history
            .iter()
            .map(|&state| state.position)
            .collect::<HashSet<Complex<isize>>>()
            .len()
    }
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let mut instance = Self::default();
        for (row, line) in input.lines().enumerate() {
            let y = row as isize;
            instance.max_y = instance.max_y.max(y);
            for (col, c) in line.char_indices() {
                let x = col as isize;
                if c != '.' {
                    instance.items.insert(Complex::new(x, y), Item::from(c));
                    instance.max_x = instance.max_x.max(x);
                }
            }
        }
        instance
    }

    fn solve(mut self) -> Self {
        self.part1 = self.energize(State {
            position: Complex::new(0, 0),
            direction: Complex::new(1, 0),
        });
        self.part2 = self.more_energy_more_passion();
        self
    }
}

#[cfg(test)]
mod the_floor_will_be_lava {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day16.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 46);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 51);
    }
}
