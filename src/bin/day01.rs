use advent_of_code_2023::*;

pub const PUZZLE: &str = include_str!("../../puzzles/day01.txt");

/// So OOP was a surprisingly *bad* approach for this particular problem.
/// Not as easy as it looks! They never are.
fn main() {
    println!("Part 1: {}", solve(PUZZLE, Part::One));
    println!("Part 2: {}", solve(PUZZLE, Part::Two));
    //println!("Part 2: {}", d.part2.unwrap());
    //println!("{:?}", Puzzle::time(PUZZLE));
}

pub fn solve(input: &str, part: Part) -> usize {
    let words = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];
    let words = match part {
        Part::One => &words[..10],
        Part::Two => &words[..],
    };
    input
        .lines()
        .map(|line| {
            let (_, x) = words
                .iter()
                .enumerate()
                .flat_map(|(i, w)| {
                    if let Some(p) = line.find(w) {
                        Some((p, i % 10))
                    } else {
                        None
                    }
                })
                .min()
                .expect("first digit in line");
            let (_, y) = words
                .iter()
                .enumerate()
                .flat_map(|(i, w)| {
                    if let Some(p) = line.rfind(w) {
                        Some((p, i % 10))
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or((x, x));
            (x, y)
        })
        .map(|(x, y)| 10 * x + y)
        .sum()
}

#[cfg(test)]
mod puzzle_name {
    use super::*;

    const SAMPLE1: &str = include_str!("../../samples/day01-1.txt");
    const SAMPLE2: &str = include_str!("../../samples/day01-2.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(SAMPLE1, Part::One), 142);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(SAMPLE2, Part::Two), 281);
    }
}
