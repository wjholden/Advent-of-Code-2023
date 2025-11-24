use std::collections::HashMap;

fn main() {
    let mut springs = Springs::new(include_str!("../../puzzles/day12.txt"));
    println!("Part 1: {}", springs.part1());
    springs.unfold();
}

#[derive(Debug)]
struct Springs(Vec<SpringRow>);

impl Springs {
    fn new(input: &str) -> Self {
        Self(input.lines().map(SpringRow::new).collect())
    }

    fn part1(&self) -> usize {
        self.0.iter().map(SpringRow::arrangements).sum()
    }

    fn unfold(&mut self) {
        for row in &mut self.0 {
            row.unfold();
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Symbol {
    Operational, // .
    Damaged,     // #
    Unknown,     // ?
}

impl Symbol {
    fn new(input: &str) -> Vec<Self> {
        input
            .chars()
            .map(|c| match c {
                '.' => Symbol::Operational,
                '#' => Symbol::Damaged,
                '?' => Symbol::Unknown,
                _ => panic!(),
            })
            .collect()
    }
}

#[derive(Debug)]
struct SpringRow {
    springs: Vec<Symbol>,
    damaged: Vec<usize>,
}

impl SpringRow {
    fn new(input: &str) -> Self {
        let mut row = input.trim().split_ascii_whitespace();
        let springs = row.next().expect("symbols of spring status");
        let springs = Symbol::new(springs);
        let damaged = row
            .next()
            .expect("comma-delimited list of broken spring groups");
        let damaged = damaged
            .split(",")
            .map(|d| d.parse::<usize>().expect("numeric group size"))
            .collect();
        Self { springs, damaged }
    }

    fn unfold(&mut self) {
        let mut v = Vec::new();
        for i in 1..=5 {
            for e in &self.springs {
                v.push(e.clone());
            }
            if i < 5 {
                v.push(Symbol::Unknown);
            }
        }
        self.springs = v;
        self.damaged = self.damaged.repeat(5);
    }

    fn arrangements(&self) -> usize {
        self.matches(&self.springs[..], &self.damaged[..], State::May)
    }

    fn matches(&self, symbols: &[Symbol], group: &[usize], state: State) -> usize {
        match (symbols.get(0), state) {
            (_, State::Must(0)) => unreachable!(),
            (None, State::Must(_)) => 0, // end of symbols still needing a match
            (None, State::Not | State::May) if !group.is_empty() => 0, // end of symbols with groups not matched
            (None, State::Not | State::May) => 1,
            (Some(Symbol::Operational), State::Must(_)) => 0, // need #, but got .
            (Some(Symbol::Damaged), State::Not) => 0,         // need ., but got #
            (Some(Symbol::Unknown), State::Not) => self.matches(&symbols[1..], group, State::May), // singly-recursive case of state transition to negative match to free
            (Some(Symbol::Damaged | Symbol::Unknown), State::Must(1)) => {
                self.matches(&symbols[1..], group, State::Not)
            } // # or ? at the end of a group
            (Some(Symbol::Damaged | Symbol::Unknown), State::Must(n)) => {
                self.matches(&symbols[1..], group, State::Must(n - 1))
            } // # or ? inside a group
            (Some(Symbol::Operational), State::Not | State::May) => {
                self.matches(&symbols[1..], group, State::May)
            } // needed ., got ., now transition state to free
            (Some(Symbol::Unknown), State::May) => {
                if let Some(g) = group.get(0) {
                    self.matches(
                        &symbols[1..],
                        &group[1..],
                        if *g > 1 {
                            State::Must(g - 1)
                        } else {
                            State::Not
                        },
                    ) + self.matches(&symbols[1..], group, State::May)
                } else {
                    self.matches(&symbols[1..], group, State::May)
                }
            } // doubly-recursive case where the match ? in a maybe state
            (Some(Symbol::Damaged), State::May) => {
                if let Some(g) = group.get(0) {
                    self.matches(
                        &symbols[1..],
                        &group[1..],
                        if *g > 1 {
                            State::Must(g - 1)
                        } else {
                            State::Not
                        },
                    )
                } else {
                    0
                }
            } // singly-recursive state transition from may to must (or not if the group is just 1)
        }
    }
}

#[derive(Debug)]
enum State {
    Must(usize),
    Not,
    May,
}

#[cfg(test)]
mod day12 {
    use super::*;

    const SAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    const SMALL: &str = "???? 1,1
????? 1,1";

    #[test]
    fn test1() {
        assert_eq!(Springs::new(SAMPLE).part1(), 21)
    }

    #[test]
    fn small() {
        assert_eq!(Springs::new(SMALL).part1(), 3 + 6)
    }

    #[test]
    fn unfolding() {
        let mut s = SpringRow::new(".# 1");
        s.unfold();
        let t = SpringRow::new(".#?.#?.#?.#?.# 1,1,1,1,1");
        assert_eq!(s.springs, t.springs);
        assert_eq!(s.damaged, t.damaged);
    }
}
