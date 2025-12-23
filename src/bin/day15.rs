use std::array;

use advent_of_code_2023::*;
use nom::Input;

pub const PUZZLE: &str = include_str!("../../puzzles/day15.txt");

/// Fun little puzzle where you build your own hash and hashmap.
/// Lifetimes for string references were too tricky so I gave up and passed
/// owned strings instead.
///
/// Type system to the rescue as always, I feel like I'm repeating myself.
///
/// Had initially reached for a BTreeMap, incorrectly thinking that it
/// preserves insertion order. BTreeMap does not preserve insertion order,
/// nor does HashMap. I ended up doing linear scans over a boring array,
/// which works very well in this small case.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
    println!("{:?}", Puzzle::time(PUZZLE));
}

enum Step<'a> {
    Assign(&'a str, usize),
    Remove(&'a str),
}

impl<'a> Step<'a> {
    fn from(step: &'a str) -> Self {
        if let Some(i) = step.position(|c| c == '=') {
            Self::Assign(&step[..i], step[i + 1..].parse::<usize>().unwrap())
        } else {
            Self::Remove(&step[0..step.len() - 1])
        }
    }
}

pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    steps: Vec<String>,
}

fn hash_byte(a: u8, b: u8) -> u8 {
    let mut d = (a as u16) + (b as u16);
    d *= 17;
    //d %= 256; // We don't need to explicitly do this.
    // The conversion to u8 drops the leading 8 bits for us.
    d as u8
}

fn hash_string(s: &str) -> u8 {
    s.as_bytes().iter().fold(0, |a, &b| hash_byte(a, b))
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let input = input.trim();
        //let mut instance = Self::default();
        let steps = input.split(',').map(str::to_owned).collect();
        Self {
            part1: 0,
            part2: 0,
            steps,
        }
    }

    fn solve(mut self) -> Self {
        self.part1 = self
            .steps
            .iter()
            .map(|step| hash_string(step) as usize)
            .sum::<usize>();

        let mut boxes: [Vec<(&str, usize)>; 256] = array::from_fn(|_| Vec::new());

        for step in self.steps.iter() {
            let step = Step::from(step);
            let box_no = hash_string(match step {
                Step::Assign(label, _) | Step::Remove(label) => label,
            }) as usize;
            match step {
                Step::Assign(lens, focal_length) => {
                    if let Some(i) = boxes[box_no].iter().position(|(label, _)| **label == *lens) {
                        boxes[box_no][i].1 = focal_length;
                    } else {
                        boxes[box_no].push((lens, focal_length))
                    }
                }
                Step::Remove(lens) => {
                    boxes[box_no].retain(|(label, _)| **label != *lens);
                }
            };
        }

        for (box_no, slots) in boxes.iter().enumerate() {
            for (slot_no, (_label, focal_length)) in slots.iter().enumerate() {
                self.part2 += (1 + box_no) * (1 + slot_no) * focal_length;
            }
        }

        self
    }
}

#[cfg(test)]
mod lens_library {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day15.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 1320);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 145);
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash_string("HASH"), 52);
    }
}
