use std::{collections::HashSet, fmt, ops::Add, panic};

use num::Complex;

fn main() {
    let pipes = Pipes::new(include_str!("../../puzzles/day10.txt"));
    let (part1, part2) = pipes.solve();
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

struct Pipes {
    input: String,
    rows: i64,
    cols: i64,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Direction {
    LEFT,
    RIGHT,
    DOWN,
    UP,
}

use Direction::*;

impl Add<Direction> for Complex<i64> {
    type Output = Complex<i64>;

    fn add(self, rhs: Direction) -> Self::Output {
        self + match rhs {
            Direction::LEFT => Complex::new(-1, 0),
            Direction::RIGHT => Complex::new(1, 0),
            Direction::DOWN => Complex::new(0, -1),
            Direction::UP => Complex::new(0, 1),
        }
    }
}

impl Pipes {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let cols = lines.next().expect("first line from input").len();
        let rows = lines.count() + 1;
        Self {
            input: input.to_owned(),
            rows: rows as i64,
            cols: cols as i64,
        }
    }

    fn get(&self, position: Complex<i64>) -> char {
        let x = position.re;
        let y = position.im;
        self.input
            .chars()
            .nth(((self.rows - y - 1) * (self.cols + 1) + x) as usize)
            .expect(format!("current pipe in position {x},{y}").as_str())
    }

    fn initial_position(&self) -> Result<Complex<i64>, ()> {
        for (y, line) in self.input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    return Ok(Complex::new(x as i64, self.rows - y as i64 - 1));
                }
            }
        }
        Err(())
    }

    fn initial_directions(&self) -> [Direction; 2] {
        let s = self.initial_position().expect("initial position");

        let mut d = vec![];

        if matches!(self.get(s + LEFT), '-' | 'L' | 'F') {
            d.push(LEFT);
        }

        if matches!(self.get(s + RIGHT), '-' | 'J' | '7') {
            d.push(RIGHT);
        }

        if matches!(self.get(s + DOWN), '|' | 'L' | 'J') {
            d.push(DOWN);
        }

        if matches!(self.get(s + UP), '|' | '7' | 'F') {
            d.push(UP);
        }

        d.try_into().expect("two initial directions")
    }

    fn s(&self) -> char {
        match self.initial_directions() {
            [LEFT, RIGHT] | [RIGHT, LEFT] => '-',
            [LEFT, DOWN] | [DOWN, LEFT] => '7',
            [LEFT, UP] | [UP, LEFT] => 'J',
            [RIGHT, DOWN] | [DOWN, RIGHT] => 'F',
            [RIGHT, UP] | [UP, RIGHT] => 'L',
            [DOWN, UP] | [UP, DOWN] => '|',
            [x, y] if x == y => unreachable!(),
            _ => panic!(),
        }
    }

    fn path(&self) -> HashSet<Complex<i64>> {
        let mut path = HashSet::new();
        let s = self.initial_position().expect("initial position");
        let mut p = s;
        let mut d = self.initial_directions()[0];

        loop {
            path.insert(p);
            p = p + d;
            if p == s {
                break;
            }
            d = match (d, self.get(p)) {
                (_, '-' | '|') => d,
                (DOWN, 'L') | (UP, 'F') => RIGHT,
                (LEFT, 'F') | (RIGHT, '7') => DOWN,
                (UP, '7') | (DOWN, 'J') => LEFT,
                (RIGHT, 'J') | (LEFT, 'L') => UP,
                _ => panic!("unexpected direction/pipe combination"),
            };
        }
        path
    }

    fn solve(&self) -> (usize, usize) {
        let path = self.path();
        let mut inner = HashSet::new();
        enum State {
            Outside,
            Inside,
        }
        let mut state;
        for y in 0..self.rows {
            state = State::Outside;
            for x in 0..self.cols {
                let position = Complex::new(x, y);
                match (&state, path.contains(&position)) {
                    (State::Outside, true) => {
                        state = State::Inside;
                    }
                    (State::Outside, false) => {}
                    (State::Inside, true) => {
                        state = State::Outside;
                    }
                    (State::Inside, false) => {
                        inner.insert(position);
                    }
                };
            }
        }
        (path.len() / 2, inner.len())
    }
}

impl fmt::Display for Pipes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = self.path();
        for y in (0..self.rows).rev() {
            for x in 0..self.cols {
                let position = Complex::new(x, y);
                match path.contains(&position) {
                    true => {
                        write!(f, "{}", self.get(position))?;
                    }
                    false => {
                        write!(f, " ")?;
                    }
                };
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod day10 {
    use super::*;

    const SAMPLE1: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const SAMPLE2: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const SAMPLE3: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    #[test]
    fn test1() {
        let pipes = Pipes::new(SAMPLE1);
        assert_eq!(pipes.solve().0, 8);
    }

    #[test]
    fn test2() {
        let pipes = Pipes::new(SAMPLE2);
        println!("{pipes}");
        assert_eq!(pipes.solve().1, 4);
    }

    #[test]
    fn test3() {
        let pipes = Pipes::new(SAMPLE3);
        println!("{pipes}");
        assert_eq!(pipes.solve().1, 8);
    }
}
