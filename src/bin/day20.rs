use std::collections::{HashMap, VecDeque};

use advent_of_code_2023::*;
use petgraph::{
    Direction::Incoming,
    graph::{DiGraph, NodeIndex},
    visit::{EdgeRef, NodeRef},
};

pub const PUZZLE: &str = include_str!("../../puzzles/day20.txt");

/// LCM FTW! For part 2 you need to find the period length for each of the four
/// inputs leading into module `lx`. `lx` is a conjunction module. We need to
/// record the first instance of each of its four inputs sending a high pulse.
/// The first time all four are high is their product. (Ah, actually I think it
/// would be their lowest common multiple, but since all four occur at a prime
/// number, the product of 4093, 4091, 3733, and 3911 is their LCM.)
///
/// This problem was a design challenge for me. I didn't start out using
/// Petgraph, and if I was going to do it again then I probably wouldn't use it
/// a second time. Petgraph does not have node lookup features that I had
/// expected, so it isn't convenient to find an edge from a vertex you created
/// from a `String` label.
///
/// The problem was all in the conjunction modules (`&` nodes). These need to
/// remember their inputs, and we need to eagerly populate these inputs with
/// default values (low pulse).
///
/// I think a quicker design would have been to just populate adjacency lists
/// for both directions: outputs from and inputs into each node. Oh well.
///
/// I'm pretty OK with the object-oriented design here.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1.unwrap());
    println!("Part 2: {}", d.part2.unwrap());
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Default, Debug)]
pub struct Puzzle {
    pub part1: Option<usize>,
    pub part2: Option<usize>,
    graph: DiGraph<String, ()>,
    modules: HashMap<String, Module>,
}

#[derive(Debug, Clone, Copy)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
enum Module {
    FlipFlop { on: bool },
    Conjunction { inputs: HashMap<String, Pulse> },
    Broadcast,
    ReceiveOnly,
}

impl Module {
    fn send_receive(&mut self, pulse: Pulse, src: &str) -> Option<Pulse> {
        match self {
            Module::FlipFlop { on } => match pulse {
                Pulse::High => None,
                Pulse::Low => {
                    *on = *on ^ true;
                    if *on {
                        Some(Pulse::High)
                    } else {
                        Some(Pulse::Low)
                    }
                }
            },
            Module::Conjunction { inputs } => {
                if inputs.insert(src.to_owned(), pulse).is_none() {
                    dbg!(src);
                    panic!("this conjunction node should have already known about this sender");
                }
                // "If it remembers high pulses for all inputs, it sends a low
                // pulse; otherwise, it sends a high pulse."
                //
                // I really like that Rust's rich syntax allows me to describe
                // the problem so exactly. This doesn't really need testing, it
                // just reads like the English specification.
                if inputs.values().all(|pulse| matches!(pulse, Pulse::High)) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            Module::Broadcast => Some(pulse),
            Module::ReceiveOnly => None,
        }
    }
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let mut g = DiGraph::<String, ()>::new();
        let mut node_ids: HashMap<_, _> = input
            .lines()
            .map(|line| {
                let mut name = line.split_ascii_whitespace().next().unwrap();
                if &name[0..1] == "%" || &name[0..1] == "&" {
                    name = &name[1..];
                }
                let id = g.add_node(name.to_owned());
                (name, id)
            })
            .collect();

        let mut modules: HashMap<String, Module> = HashMap::new();

        let input = input.replace(",", "");
        for line in input.lines() {
            let mut it = line.split_ascii_whitespace();
            let mut src = it.next().unwrap();
            if &src[0..1] == "%" || &src[0..1] == "&" {
                src = &src[1..];
            }
            for dst in it.skip(1) {
                // Something weird about this puzzle is that the input contains
                // an "rx" module, but neither of the examples have this node.
                if !node_ids.contains_key(dst) {
                    assert!(dst == "rx" || dst == "output");
                    let node_id = g.add_node(dst.to_owned());
                    node_ids.insert(&dst, node_id);
                    modules.insert(dst.to_owned(), Module::ReceiveOnly);
                }
                // dbg!(&[src, dst]);
                g.add_edge(node_ids[src], node_ids[dst], ());
            }
        }

        // Iterate over the input yet again, this time creating module objects.
        for line in input.lines() {
            let (name, _) = line.split_once(" -> ").unwrap();
            let (name, module) = match &name[0..1] {
                "b" => (name, Module::Broadcast),
                "%" => (&name[1..], Module::FlipFlop { on: false }),
                "&" => {
                    let mut inputs = HashMap::new();
                    let name = &name[1..];
                    let id = node_ids[name];
                    for edge in g.edges_directed(id, Incoming) {
                        // Petgraph doesn't make it obvious how to get a node's
                        // label (value, whatever) from its vertices.
                        let x = edge.source().id();
                        let y = &g[x];
                        inputs.insert(y.clone(), Pulse::Low);
                    }
                    // println!("Conjunction node {name} has inputs {:?}", inputs.keys());
                    (name, Module::Conjunction { inputs })
                }
                _ => unreachable!(),
            };
            modules.insert(name.to_owned(), module);
        }

        Self {
            part1: None,
            part2: None,
            graph: g,
            modules,
        }
    }

    fn solve(mut self) -> Self {
        let mut highs = 0;
        let mut lows = 0;
        let mut queue: VecDeque<(Pulse, &str, &str)> = VecDeque::new();
        let mut part2: HashMap<String, usize> = HashMap::new();

        'outer: for button_presses in 1..10000 {
            queue.push_back((Pulse::Low, "button", "broadcaster"));
            let mut dh = 0;
            let mut dl = 0;

            while let Some((pulse, src, dst)) = queue.pop_front() {
                if dst == "rx"
                    && let Module::Conjunction { inputs } = &self.modules["lx"]
                {
                    for (key, value) in inputs.iter() {
                        if matches!(value, Pulse::High) && !part2.contains_key(key) {
                            part2.insert(key.to_owned(), button_presses);
                        }
                    }
                    if part2.len() == 4 {
                        println!("{part2:?}");
                        self.part2 = Some(part2.values().product());
                        break 'outer;
                    }
                }
                match pulse {
                    Pulse::High => dh += 1,
                    Pulse::Low => dl += 1,
                };
                if let Some(output) = self.modules.get_mut(dst).unwrap().send_receive(pulse, src) {
                    // Starting to really regret my choice of data structure here.
                    // https://stackoverflow.com/a/77729229/5459668
                    let src_id = NodeIndex::new(
                        self.graph
                            .raw_nodes()
                            .iter()
                            .position(|n| n.weight == dst)
                            .unwrap(),
                    );
                    for edge in self.graph.edges(src_id) {
                        let next_dst = &self.graph[edge.target().id()];
                        queue.push_back((output, dst, next_dst));
                    }
                }
            }

            highs += dh;
            lows += dl;
            if button_presses == 1000 {
                self.part1 = Some(highs * lows);
            }
        }
        self
    }
}

#[cfg(test)]
mod pulse_propagation {
    use super::*;

    const SAMPLE1: &str = include_str!("../../samples/day20-1.txt");
    const SAMPLE2: &str = include_str!("../../samples/day20-2.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE1).solve().part1, Some(32000000));
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE2).solve().part1, Some(11687500));
    }
}
