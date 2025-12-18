use std::collections::HashMap;

/// The matches function was originally in the SpringRow object, but
/// I had to move it because `&mut self` wasn't playing nice with
/// dynamic programming.
///
/// The approach here is pretty ugly to me, but it works and
/// it's quick enough. In Java, I would have created a HashMap
/// as a member of the SpringRow object and used that for my
/// cache. The doubly-recursive call would have happily mutated
/// the cache on both sides of the recursion tree. This is not
/// possible in Rust. In Rust, the borrow checker prevents us
/// from having two mutable pointers to the same data structure.
/// As a workaround, I'm passing ownership of the cache throughout
/// the program. The method signature is long and you have to keep
/// track of the cache throughout. I don't love it.
///
/// I *do* like the state table, though. Exhaustive pattern
/// matching is easily my favorite Rust feature. We match
/// the current "symbol" ('.', '#', or '?') from the input string
/// and determine if it must match, must not match, or may
/// match (Must, Not, and May states). The Must state has
/// an associated count of how many '#' or '?' symbols we need.
/// We never make it to Must(0) because we transition to
/// the Not state first, which means we cannot match '#',
/// and if we match '?' then this can only be interpreted as
/// '.'.
///
/// The [memorize](https://docs.rs/memorize/latest/memorize/)
/// crate might have been usable for this task, but probably
/// not when using the object-oriented approach.
pub fn main() {
    let mut springs = Springs::new(include_str!("../../puzzles/day12.txt"));
    println!("Part 1: {}", springs.total_arrangements());
    springs.unfold();
    println!("Part 2: {}", springs.total_arrangements());
}

#[derive(Debug)]
struct Springs(Vec<SpringRow>);

impl Springs {
    fn new(input: &str) -> Self {
        Self(input.lines().map(SpringRow::new).collect())
    }

    fn total_arrangements(&self) -> usize {
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
        matches(
            &self.springs[..],
            &self.damaged[..],
            State::May,
            HashMap::new(),
        )
        .0
    }
}

fn matches(
    symbols: &[Symbol],
    group: &[usize],
    state: State,
    cache: HashMap<(usize, usize, State), usize>,
) -> (usize, HashMap<(usize, usize, State), usize>) {
    let k = (symbols.len(), group.len(), state);
    if let Some(&value) = cache.get(&k) {
        (value, cache)
    } else {
        let (value, mut cache) = match (symbols.get(0), state) {
            (_, State::Must(0)) => unreachable!(),
            (None, State::Must(_)) => (0, cache), // end of symbols still needing a match
            (None, State::Not | State::May) if !group.is_empty() => (0, cache), // end of symbols with groups not matched
            (None, State::Not | State::May) => (1, cache),
            (Some(Symbol::Operational), State::Must(_)) => (0, cache), // need #, but got .
            (Some(Symbol::Damaged), State::Not) => (0, cache),         // need ., but got #
            (Some(Symbol::Unknown), State::Not) => matches(&symbols[1..], group, State::May, cache), // singly-recursive case of state transition to negative match to free
            (Some(Symbol::Damaged | Symbol::Unknown), State::Must(1)) => {
                matches(&symbols[1..], group, State::Not, cache)
            } // # or ? at the end of a group
            (Some(Symbol::Damaged | Symbol::Unknown), State::Must(n)) => {
                matches(&symbols[1..], group, State::Must(n - 1), cache)
            } // # or ? inside a group
            (Some(Symbol::Operational), State::Not | State::May) => {
                matches(&symbols[1..], group, State::May, cache)
            } // needed ., got ., now transition state to free
            (Some(Symbol::Unknown), State::May) => {
                let (when_taken, cache) = if let Some(g) = group.get(0) {
                    matches(
                        &symbols[1..],
                        &group[1..],
                        if *g > 1 {
                            State::Must(g - 1)
                        } else {
                            State::Not
                        },
                        cache,
                    )
                } else {
                    (0, cache)
                };
                let (when_not_taken, cache) = matches(&symbols[1..], group, State::May, cache);
                (when_taken + when_not_taken, cache)
            } // doubly-recursive case where the match ? in a maybe state
            (Some(Symbol::Damaged), State::May) => {
                if let Some(g) = group.get(0) {
                    matches(
                        &symbols[1..],
                        &group[1..],
                        if *g > 1 {
                            State::Must(g - 1)
                        } else {
                            State::Not
                        },
                        cache,
                    )
                } else {
                    (0, cache)
                }
            } // singly-recursive state transition from may to must (or not if the group is just 1)
        };
        cache.insert(k, value);
        (value, cache)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
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
        assert_eq!(Springs::new(SAMPLE).total_arrangements(), 21)
    }

    #[test]
    fn small() {
        assert_eq!(Springs::new(SMALL).total_arrangements(), 3 + 6)
    }

    #[test]
    fn unfolding() {
        let mut s = SpringRow::new(".# 1");
        s.unfold();
        let t = SpringRow::new(".#?.#?.#?.#?.# 1,1,1,1,1");
        assert_eq!(s.springs, t.springs);
        assert_eq!(s.damaged, t.damaged);
    }

    #[test]
    fn test2() {
        let mut s = Springs::new(SAMPLE);
        s.unfold();
        assert_eq!(s.total_arrangements(), 525152)
    }
}
