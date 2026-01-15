use core::panic;
use std::collections::HashMap;

use num::Integer;

pub const PUZZLE: &str = include_str!("../../puzzles/day08.txt");

fn main() {
    let n = Network::new(PUZZLE);
    println!("Part 1: {}", n.zzz("AAA").unwrap());
    println!("Part 2: {}", n.part2_lcm());
}

#[derive(Debug)]
pub struct Network<'a> {
    turns: String,
    directions: HashMap<&'a str, Position<'a>>,
}

impl<'a> Network<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lines = input.lines();
        let turns = lines.next().unwrap().to_owned();
        let mut directions = HashMap::default();
        lines.next(); // discard blank line
        for line in lines {
            let a = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            directions.insert(a, Position { left, right });
        }
        Self { turns, directions }
    }

    pub fn zzz(&self, start: &str) -> Result<usize, ()> {
        let mut p = start;
        let mut i = 0;
        for c in self.turns.chars().cycle() {
            if p.ends_with("Z") {
                return Ok(i);
            }
            let d = self.directions.get(p).unwrap();
            p = match c {
                'L' => d.left,
                'R' => d.right,
                _ => panic!(),
            };
            i += 1;
        }
        Err(())
    }

    #[allow(dead_code)]
    fn part2_naive(&self) -> Result<usize, ()> {
        let mut p: Vec<_> = self
            .directions
            .keys()
            .filter(|s| s.ends_with("A"))
            .collect();
        let mut i = 0;
        for c in self.turns.chars().cycle() {
            if p.iter().all(|s| s.ends_with("Z")) {
                return Ok(i);
            }
            for ghostp in &mut p {
                let d = self.directions.get(*ghostp).unwrap();
                *ghostp = match c {
                    'L' => &d.left,
                    'R' => &d.right,
                    _ => panic!(),
                };
            }
            i += 1;
        }

        Err(())
    }

    pub fn part2_lcm(&self) -> usize {
        self.directions
            .keys()
            .filter(|k| k.ends_with("A"))
            .map(|k| self.zzz(k).unwrap())
            .fold(1, |acc, d| acc.lcm(&d))
    }
}

#[derive(Debug)]
struct Position<'a> {
    left: &'a str,
    right: &'a str,
}

#[cfg(test)]
mod day08 {
    use super::*;

    const SAMPLE1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test1() {
        let n = Network::new(SAMPLE1);
        assert_eq!(n.zzz("AAA"), Ok(2));
    }

    #[test]
    fn test2() {
        let n = Network::new(SAMPLE2);
        assert_eq!(n.zzz("AAA"), Ok(6));
    }

    #[test]
    fn test3() {
        let n = Network::new(SAMPLE3);
        assert_eq!(n.part2_naive(), Ok(6));
    }

    #[test]
    fn test4() {
        let n = Network::new(SAMPLE3);
        assert_eq!(n.part2_naive().unwrap(), n.part2_lcm());
    }
}
