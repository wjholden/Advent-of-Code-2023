use std::{
    collections::{HashMap, HashSet},
    fmt,
    ops::{Add, Mul},
    panic,
};

use num::Complex;

pub const PUZZLE: &str = include_str!("../../puzzles/day10.txt");

/// This was a tricky one! I decided to first explore the path using
/// an object-oriented approach. Then, I follow the path again to
/// trace which empty tiles are to the left and right. This gets
/// us closer to a solution, but we will end up with an empty area
/// in the middle that the path doesn't touch. We do a breadth-
/// first search on that center area to fill in all the innermost
/// tiles.
///
/// Complex arithmetic wasn't as elegant as I had expected for today.
///
/// Looks like there were some much better approaches to this puzzle
/// (https://www.reddit.com/r/adventofcode/comments/18f1sgh/2023_day_10_part_2_advise_on_part_2/):
/// - [Scanline](https://www.reddit.com/r/adventofcode/comments/18f1sgh/comment/kcripvi/)
///   (I tried this but couldn't think through all the cases)
/// - [Pick's algorithm](https://www.reddit.com/r/adventofcode/comments/18f1sgh/comment/kcr8tyf/)
///   to count the integer coordinates inside the pipe, and
/// - [Shoelace formula](https://www.reddit.com/r/adventofcode/comments/18f1sgh/comment/kcugm6t/)
///   for the area.
fn main() {
    let pipes = Pipes::new(PUZZLE);
    let (part1, part2) = pipes.solve();
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    // println!("{pipes}");
}

pub struct Pipes {
    area: HashMap<Complex<i64>, char>,
    rows: i64,
    cols: i64,
    start: Complex<i64>,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

use Direction::*;

impl Add<Direction> for Complex<i64> {
    type Output = Complex<i64>;

    fn add(self, rhs: Direction) -> Self::Output {
        self + match rhs {
            Direction::Left => Complex::new(-1, 0),
            Direction::Right => Complex::new(1, 0),
            Direction::Down => Complex::new(0, -1),
            Direction::Up => Complex::new(0, 1),
        }
    }
}

impl Mul<Direction> for Complex<i64> {
    type Output = Complex<i64>;

    fn mul(self, rhs: Direction) -> Self::Output {
        self * match rhs {
            Direction::Left => Complex::new(-1, 0),
            Direction::Right => Complex::new(1, 0),
            Direction::Down => Complex::new(0, -1),
            Direction::Up => Complex::new(0, 1),
        }
    }
}

#[derive(Eq, PartialEq)]
enum Side {
    Left,
    Right,
    Inside,
    Outside,
}

impl Pipes {
    pub fn new(input: &str) -> Self {
        let mut lines = input.trim().lines();
        let cols = lines.next().expect("first line from input").len();
        let rows = lines.count() + 1;
        let mut area = HashMap::new();

        let mut start = None;
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let position = Complex::new(col as i64, (rows - row) as i64);
                if c == 'S' {
                    start = Some(position);
                }
                area.insert(position, c);
            }
        }

        Self {
            area,
            rows: rows as i64,
            cols: cols as i64,
            start: start.unwrap(),
        }
    }

    fn get(&self, position: Complex<i64>) -> char {
        match self.area.get(&position) {
            Some('S') => self.s(),
            Some(c) => *c,
            None => '.',
        }
    }

    fn initial_directions(&self) -> [Direction; 2] {
        //let s = self.initial_position().expect("initial position");
        let s = self.start;

        let mut d = vec![];

        if matches!(self.get(s + Left), '-' | 'L' | 'F') {
            d.push(Left);
        }

        if matches!(self.get(s + Right), '-' | 'J' | '7') {
            d.push(Right);
        }

        if matches!(self.get(s + Down), '|' | 'L' | 'J') {
            d.push(Down);
        }

        if matches!(self.get(s + Up), '|' | '7' | 'F') {
            d.push(Up);
        }

        d.try_into().expect("two initial directions")
    }

    fn s(&self) -> char {
        match self.initial_directions() {
            [Left, Right] | [Right, Left] => '-',
            [Left, Down] | [Down, Left] => '7',
            [Left, Up] | [Up, Left] => 'J',
            [Right, Down] | [Down, Right] => 'F',
            [Right, Up] | [Up, Right] => 'L',
            [Down, Up] | [Up, Down] => '|',
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
                (Down, 'L') | (Up, 'F') => Right,
                (Left, 'F') | (Right, '7') => Down,
                (Up, '7') | (Down, 'J') => Left,
                (Right, 'J') | (Left, 'L') => Up,
                _ => panic!("unexpected direction/pipe combination"),
            };
        }
        path
    }

    fn sides(&self, path: &HashMap<Complex<i64>, Direction>) -> HashMap<Complex<i64>, Side> {
        let mut sides = HashMap::new();
        // Walk the full path. If there's a space to your left or right that wasn't on the
        // path, then remember that this tile was on that side of you. If the pipe bends,
        // then also remember the tile behind us. Our path records the direction of travel
        // after the rotation.
        for (position, direction) in path {
            let l = position + Complex::I * *direction;
            let r = position - Complex::I * *direction;
            let backwards = position + Complex::<i64>::I * (Complex::I * *direction);
            match (self.get(*position), direction) {
                ('|' | '-', _) => {}
                ('J', Up) | ('F', Down) | ('L', Right) | ('7', Left) => {
                    if !path.contains_key(&backwards) {
                        sides.insert(backwards, Side::Right);
                    }
                }
                ('J', Left) | ('L', Up) | ('7', Down) | ('F', Right) => {
                    if !path.contains_key(&backwards) {
                        sides.insert(backwards, Side::Left);
                    }
                }
                ('S', _) => {
                    unreachable!()
                }
                _ => unreachable!(),
            };
            if !path.contains_key(&l) {
                sides.insert(l, Side::Left);
            }
            if !path.contains_key(&r) {
                sides.insert(r, Side::Right);
            }
        }
        self.select(&mut sides);
        self.fill(&mut sides, path);
        sides
    }

    /// Decide whether the left or right was inside or outside.
    fn select(&self, sides: &mut HashMap<Complex<i64>, Side>) {
        let l = sides.values().filter(|s| **s == Side::Left).count();
        let r = sides.values().filter(|s| **s == Side::Right).count();
        for side in sides.values_mut() {
            *side = match (l < r, &side) {
                (true, Side::Left) => Side::Inside,
                (true, Side::Right) => Side::Outside,
                (false, Side::Left) => Side::Outside,
                (false, Side::Right) => Side::Inside,
                _ => unreachable!(),
            };
        }
    }

    // Not quite there. We still need to fill in the empty
    // space on the inside that isn't touching the path.
    fn fill(
        &self,
        sides: &mut HashMap<Complex<i64>, Side>,
        path: &HashMap<Complex<i64>, Direction>,
    ) {
        let mut frontier: Vec<Complex<i64>> = sides
            .iter()
            .filter_map(|(k, v)| match v {
                Side::Inside => Some(*k),
                _ => None,
            })
            .collect();
        let mut explored: HashSet<Complex<i64>> = HashSet::new();
        loop {
            if let Some(position) = frontier.pop()
                && !explored.contains(&position)
            {
                let mut d = Complex::ONE;
                for _ in 1..=4 {
                    d *= Complex::I;
                    let p = position + d;
                    if 0 <= p.re
                        && p.re < self.cols
                        && 0 <= p.im
                        && p.im < self.cols
                        && !sides.contains_key(&p)
                        && !path.contains_key(&p)
                        && !explored.contains(&p)
                    {
                        frontier.push(p);
                        sides.insert(p, Side::Inside);
                    }
                }
                explored.insert(position);
            } else {
                break;
            }
        }
    }

    pub fn solve(&self) -> (usize, usize) {
        let path = self.path();
        let sides = self.sides(&path);
        (
            path.len() / 2,
            sides.values().filter(|s| **s == Side::Inside).count(),
        )
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
                        Some(Left) => "<",
                        Some(Right) => ">",
                        Some(Up) => "^",
                        Some(Down) => "v",
                        None => "",
                    }
                )?;
                write!(
                    f,
                    "{}",
                    match sides.get(&position) {
                        Some(Side::Left) => "1",
                        Some(Side::Right) => "2",
                        Some(Side::Inside) => "I",
                        Some(Side::Outside) => "O",
                        None => "",
                    }
                )?;
                if !path.contains_key(&position) && !sides.contains_key(&position) {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
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
