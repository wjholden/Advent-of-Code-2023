use std::collections::HashMap;

use advent_of_code_2023::*;
use regex::Regex;

pub const PUZZLE: &str = include_str!("../../puzzles/day19.txt");

/// Small insight: if you see a bunch of zeros at the end of a number, it might
/// be the product of a lot of smaller numbers.
///
/// (Actually a false positive! Maybe none of the test cases reduced all four
/// variables. This meant that all the test cases contained a factor of 4000
/// that produced the string of zeros at the end.)
///
/// I *suspect* that the rules form a DAG. If so, we don't need to think about
/// overlapping leaf nodes.
///
/// Guess I was right. So the compression thing disappointingly doesn't help.
///
/// This was a hard one! More procedural than I was expecting. Thank goodness
/// the input formed a DAG. Might have gone overboard with the object-oriented
/// programming here, but it's OK. It was not obvious which object should do
/// the searches.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    workflows: HashMap<String, Workflow>,
    parts: Vec<MachinePart>,
}

#[derive(Debug, Clone)]
struct MachinePart {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug, Clone)]
struct MachinePartRange {
    lower: MachinePart,
    upper: MachinePart,
}

impl MachinePartRange {
    fn acceptable_rating_combination_count(&self) -> usize {
        assert!(self.lower.x < self.upper.x);
        assert!(self.lower.m < self.upper.m);
        assert!(self.lower.a < self.upper.a);
        assert!(self.lower.s < self.upper.s);
        (1 + self.upper.x - self.lower.x)
            * (1 + self.upper.m - self.lower.m)
            * (1 + self.upper.a - self.lower.a)
            * (1 + self.upper.s - self.lower.s)
    }

    fn set(&mut self, rule: &Rule, matches: bool) {
        let (target, val) = match (rule.cond, matches) {
            (std::cmp::Ordering::Less, true) => (&mut self.upper, rule.val - 1),
            (std::cmp::Ordering::Less, false) => (&mut self.lower, rule.val),
            (std::cmp::Ordering::Greater, true) => (&mut self.lower, rule.val + 1),
            (std::cmp::Ordering::Greater, false) => (&mut self.upper, rule.val),
            (std::cmp::Ordering::Equal, true | false) => unreachable!(),
        };
        match rule.var.as_str() {
            "x" => target.x = val,
            "m" => target.m = val,
            "a" => target.a = val,
            "s" => target.s = val,
            _ => unreachable!(),
        }
    }
}

impl Default for MachinePartRange {
    fn default() -> Self {
        Self {
            lower: MachinePart {
                x: 1,
                m: 1,
                a: 1,
                s: 1,
            },
            upper: MachinePart {
                x: 4000,
                m: 4000,
                a: 4000,
                s: 4000,
            },
        }
    }
}

impl MachinePart {
    pub fn evaluate(&self, workflows: &HashMap<String, Workflow>) -> Evaluation {
        let mut current = "in";

        loop {
            match current {
                "A" => return Evaluation::Accept,
                "R" => return Evaluation::Reject,
                other => {
                    let workflow = &workflows[other];
                    let mut next = None;
                    for rule in workflow.rules.iter() {
                        if rule.evaluate(self) {
                            next = Some(&rule.next);
                            break;
                        }
                    }
                    current = next.unwrap_or(&workflow.default);
                }
            }
        }
    }

    fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
struct Rule {
    var: String,
    cond: std::cmp::Ordering,
    val: usize,
    next: String,
}

impl Rule {
    fn evaluate(&self, part: &MachinePart) -> bool {
        let var = match self.var.as_str() {
            "x" => &part.x,
            "m" => &part.m,
            "a" => &part.a,
            "s" => &part.s,
            _ => unreachable!(),
        };
        var.cmp(&self.val) == self.cond
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    default: String,
}

impl Workflow {
    /// We can prune the workflows a little bit. Consider `lnx{m>1548:A,A}`.
    /// If `m>1548`, we accept. If `m<=1548`, we...also accept. So, we don't
    /// actually need to consider `m` at all. In fact, we could even not bother
    /// with `lnx` altogether: if we get here, we're always going to decide to
    /// accept the machine part.
    fn compress(&mut self) {
        while let Some(last) = self.rules.last()
            && last.next == self.default
        {
            self.rules.pop();
        }
    }
}

#[derive(Eq, PartialEq)]
enum Evaluation {
    Accept,
    Reject,
}

impl Puzzle {
    fn part2(&self) -> usize {
        self.acceptable_combination_count("in", MachinePartRange::default())
    }

    fn acceptable_combination_count(
        &self,
        workflow_label: &str,
        mut parts: MachinePartRange,
    ) -> usize {
        match workflow_label {
            "A" => parts.acceptable_rating_combination_count(),
            "R" => 0,
            _ => {
                let workflow = &self.workflows[workflow_label];
                let mut total = 0;
                for rule in workflow.rules.iter() {
                    let mut new_parts = parts.clone();
                    new_parts.set(rule, true);
                    total += self.acceptable_combination_count(&rule.next, new_parts);
                    parts.set(rule, false);
                }
                total += self.acceptable_combination_count(&workflow.default, parts);
                total
            }
        }
    }
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        // https://xkcd.com/1171/
        // This *almost* works, but we're only capturing the last rule.
        // We need to capture a repeated group.
        //
        // (?<name>\w+)\{(?<rule>(?<var>[xmas])(?<cond>[<>])(?<val>\d+):(?<next>\w+),)+(?<out>\w+)\}
        let rules_outer_re = Regex::new(r"(?<name>\w+)\{(?<rules>.+?),(?<out>\w+)\}").unwrap();
        let rules_inner_re =
            Regex::new(r"(?<var>[xmas])(?<cond>[<>])(?<val>\d+):(?<next>\w+)").unwrap();
        let workflows = rules_outer_re
            .captures_iter(input)
            .map(|co| {
                let rules = rules_inner_re
                    .captures_iter(&co["rules"])
                    .map(|ci| {
                        let cond = match &ci["cond"] {
                            "<" => std::cmp::Ordering::Less,
                            ">" => std::cmp::Ordering::Greater,
                            _ => panic!("unexpected comparison operator"),
                        };
                        let val = ci["val"].parse::<usize>().unwrap();
                        let var = ci["var"].to_owned();
                        Rule {
                            var,
                            cond,
                            val,
                            next: ci["next"].to_owned(),
                        }
                    })
                    .collect();
                (
                    co["name"].to_owned(),
                    Workflow {
                        rules,
                        default: co["out"].to_owned(),
                    },
                )
            })
            .collect();
        let parts_re = Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();
        let parts = parts_re
            .captures_iter(input)
            .map(|c| MachinePart {
                x: c["x"].parse::<usize>().unwrap(),
                m: c["m"].parse::<usize>().unwrap(),
                a: c["a"].parse::<usize>().unwrap(),
                s: c["s"].parse::<usize>().unwrap(),
            })
            .collect();
        Self {
            part1: 0,
            part2: 0,
            workflows,
            parts,
        }
    }

    fn solve(mut self) -> Self {
        for (_, workflow) in self.workflows.iter_mut() {
            workflow.compress();
        }
        for part in self.parts.iter() {
            if part.evaluate(&self.workflows) == Evaluation::Accept {
                self.part1 += part.rating();
            }
        }
        self.part2 = self.part2();
        self
    }
}

#[cfg(test)]
mod aplenty {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day19.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 19114);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 167409079868000);
    }
}
