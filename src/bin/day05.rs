fn main() {
    let puzzle = include_str!("../../puzzles/day05.txt").trim();
    let (seeds, layers) = parse(puzzle);
    //dbg!(&seeds);
    //dbg!(&layers);
    println!("Part 1: {}", part1(&seeds, &layers));
}

fn part1(seeds: &[u64], layers: &[Layer]) -> u64 {
    seeds.iter().map(|&seed| {
        let mut value = seed;
        'outer: for layer in layers {
            for rule in layer {
                if rule.src <= value && value < rule.src + rule.len {
                    value = rule.dst + (value - rule.src);
                    continue 'outer
                }
            }
        }
        value
    }).min().unwrap()
}

fn parse(input: &str) -> (Vec<u64>, Vec<Layer>) {
    let mut lines = input.lines();
    let seeds = lines.next().unwrap().split_at(7).1.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
    let lines = lines.skip(1);
    let mut layers = Vec::new();
    for line in lines {
        match line {
            "" => continue,
            _ if line.ends_with("map:") => layers.push(Layer::default()),
            l => {
                let last = layers.len() - 1;
                layers[last].push(Rule::new(l))
            },
        }
    }
    (seeds, layers)
}

type Layer = Vec<Rule>;

#[derive(Debug)]
struct Rule {
    dst: u64,
    src: u64,
    len: u64,
}

impl Rule {
    fn new(line: &str) -> Self {
        let line: Vec<u64> = line.split_ascii_whitespace().map(str::parse).map(Result::unwrap).collect();
        Self { dst: line[0], src: line[1], len: line[2] }
    }
}

#[cfg(test)]
mod day05 {
    use super::*;

    const SAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test1() {
        let (seeds, layers) = parse(SAMPLE);
        assert_eq!(35, part1(&seeds, &layers));
    }

    #[test]
    fn test2() {
        //assert_eq!(30, part2(SAMPLE));
    }
}