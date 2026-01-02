use std::collections::HashMap;

use advent_of_code_2023::*;

pub const PUZZLE: &str = include_str!("../../puzzles/day20.txt");

fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    //println!("Part 2: {}", d.part2);
    //println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Default, Debug)]
enum Pulse {
    #[default]
    Low,
    High,
}

#[derive(Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop(State),
    Conjunction,
    Broadcast,
}

impl Module {
    fn new(line: &str) -> Self {
        let module_type = match &line[0..1] {
            "%" => ModuleType::FlipFlop(State::default()),
            "&" => ModuleType::Conjunction,
            "b" => ModuleType::Broadcast,
            _ => unreachable!(),
        };
        let name_end = line.find(' ').expect("first space in the string");
        let name = line[..name_end].to_owned();
        let outputs = line
            .split_ascii_whitespace()
            .skip(2)
            .map(str::to_owned)
            .collect();
        Self {
            name,
            module_type,
            outputs,
            inputs: Vec::new(),
        }
    }

    fn io(&mut self, pulse: Pulse, last: &HashMap<String, Pulse>) -> Option<Pulse> {
        match &mut self.module_type {
            ModuleType::Broadcast => Some(pulse),
            ModuleType::FlipFlop(state) => match pulse {
                Pulse::High => None,
                Pulse::Low => {
                    *state = match state {
                        State::On => State::Off,
                        State::Off => State::On,
                    };
                    match state {
                        State::On => Some(Pulse::High),
                        State::Off => Some(Pulse::Low),
                    }
                }
            },
            ModuleType::Conjunction => todo!(),
        }
    }
}

#[derive(Default, Debug)]
enum State {
    #[default]
    Off,
    On,
}

impl State {
    fn flip(&self) -> Self {
        match self {
            State::Off => State::On,
            State::On => State::Off,
        }
    }
}

#[derive(Default, Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    modules: HashMap<String, Module>,
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let mut modules: HashMap<String, Module> = input
            .lines()
            .map(|line| {
                let module = Module::new(line);
                (module.name.clone(), module)
            })
            .collect();

        Self {
            part1: 0,
            part2: 0,
            modules,
        }
    }

    fn solve(mut self) -> Self {
        println!("{:?}", self.modules);
        self
    }
}

#[cfg(test)]
mod puzzle_name {
    use super::*;

    const SAMPLE1: &str = include_str!("../../samples/day20-1.txt");
    const SAMPLE2: &str = include_str!("../../samples/day20-2.txt");

    #[test]
    fn test1_1() {
        assert_eq!(Puzzle::new(SAMPLE1).solve().part1, 32000000);
    }

    #[test]
    fn test1_2() {
        assert_eq!(Puzzle::new(SAMPLE2).solve().part1, 11687500);
    }
}
