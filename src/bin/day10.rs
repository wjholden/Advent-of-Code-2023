use std::{
    collections::HashMap,
    fmt,
    ops::{Add, Mul},
    panic,
};

use num::Complex;

fn main() {
    let pipes = Pipes::new(include_str!("../../puzzles/day10.txt"));
    let (part1, part2) = pipes.solve();
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    println!("{pipes}");
}

struct Pipes {
    input: String,
    rows: i64,
    cols: i64,
    start: Complex<i64>,
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

impl Mul<Direction> for Complex<i64> {
    type Output = Complex<i64>;

    fn mul(self, rhs: Direction) -> Self::Output {
        self * match rhs {
            Direction::LEFT => Complex::new(-1, 0),
            Direction::RIGHT => Complex::new(1, 0),
            Direction::DOWN => Complex::new(0, -1),
            Direction::UP => Complex::new(0, 1),
        }
    }
}

enum Side {
    Left,
    Right,
}

impl Pipes {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let cols = lines.next().expect("first line from input").len();
        let rows = lines.count() + 1;

        let mut start = None;
        'outer: for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Some(Complex::new(col as i64, (rows - row - 1) as i64));
                    break 'outer;
                }
            }
        }

        Self {
            input: input.to_owned(),
            rows: rows as i64,
            cols: cols as i64,
            start: start.unwrap(),
        }
    }

    fn get(&self, position: Complex<i64>) -> char {
        let x = position.re;
        let y = position.im;
        if position == self.start {
            self.s()
        } else {
            self.input
                .chars()
                .nth(((self.rows - y - 1) * (self.cols + 1) + x) as usize)
                .expect(format!("current pipe in position {x},{y}").as_str())
        }
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
        //let s = self.initial_position().expect("initial position");
        let s = self.start;

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

    fn path(&self) -> HashMap<Complex<i64>, Direction> {
        let mut path = HashMap::new();
        //let s = self.initial_position().expect("initial position");
        let s = self.start;
        let mut p = s;
        let mut d = self.initial_directions()[0];

        loop {
            path.insert(p, d);
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

    fn sides(&self, path: &HashMap<Complex<i64>, Direction>) -> HashMap<Complex<i64>, Side> {
        let mut sides = HashMap::new();
        for (position, direction) in path {
            let l = position + Complex::i() * *direction;
            let r = position - Complex::I * *direction;
            match (self.get(*position), direction) {
                ('|' | '-', _) => {}
                ('J', UP) => {
                    sides.insert(*position - Complex::ONE * *direction, Side::Right);
                }
                ('J', LEFT) => {
                    sides.insert(*position - Complex::ONE * *direction, Side::Left);
                }
                ('L', UP) => {
                    sides.insert(*position - Complex::ONE * *direction, Side::Left);
                }
                ('L', RIGHT) => {
                    sides.insert(*position - Complex::ONE * *direction, Side::Right);
                }
                ('7', LEFT) => {
                    sides.insert(*position - Complex::ONE * *direction, Side::Right);
                }
                ('7', DOWN) => {
                    sides.insert(*position - Complex::ONE * *direction, Side::Left);
                }
                ('F', RIGHT) => {
                    sides.insert(*position - Complex::ONE * *direction, Side::Left);
                }
                ('F', DOWN) => {
                    sides.insert(*position - Complex::ONE * *direction, Side::Right);
                }
                ('S', _) => {
                    unreachable!()
                }
                _ => unreachable!(),
            };
            sides.insert(l, Side::Left);
            sides.insert(r, Side::Right);
        }
        sides.retain(|k, _| !path.contains_key(k));
        sides
    }

    fn solve(&self) -> (usize, usize) {
        let path = self.path();
        let sides = self.sides(&path);
        (path.len() / 2, sides.len())
    }
}

impl fmt::Display for Pipes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = self.path();
        let sides = self.sides(&path);
        for y in (0..self.rows).rev() {
            for x in 0..self.cols {
                let position = Complex::new(x, y);
                write!(
                    f,
                    "{}",
                    match path.get(&position) {
                        Some(LEFT) => "<",
                        Some(RIGHT) => ">",
                        Some(UP) => "^",
                        Some(DOWN) => "v",
                        None => "",
                    }
                )?;
                write!(
                    f,
                    "{}",
                    match sides.get(&position) {
                        Some(Side::Left) => "1",
                        Some(Side::Right) => "2",
                        None => "",
                    }
                )?;
                if !path.contains_key(&position) && !sides.contains_key(&position) {
                    write!(f, " ")?;
                }
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

    const SAMPLE4: &str = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";

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

    #[test]
    fn test4() {
        let pipes = Pipes::new(SAMPLE4);
        println!("{pipes}");
        assert_eq!(pipes.solve().1, 4);
    }
}
