pub const PUZZLE: &str = include_str!("../../puzzles/day09.txt");

fn main() {
    let mut histories = parse(PUZZLE);
    println!(
        "Part 1: {}",
        histories.iter().map(|v| predict(v)).sum::<isize>()
    );

    // This is a clear idiom for mutating all elements of an array.
    // https://stackoverflow.com/a/28651397/5459668
    for history in &mut histories {
        history.reverse();
    }

    println!(
        "Part 2: {}",
        histories.iter().map(|v| predict(v)).sum::<isize>()
    );
}

/// This is probably more clever than it should be.
/// In the first pass, we would have (for example):
/// ```
/// [ 1 2 3 4 ]
/// ```
/// We're going to *replace* 1, 2, and 3 with the differences:
/// ```
/// [ 1 1 1 4 ]
/// ```
/// Now again replace the first two 1's with the new differences:
/// ```
/// [ 0 0 1 4 ]
/// ```
/// This algorithm unconditionally counts all the way down, then
/// predicts the new term in the original sequence by adding.
///
/// It's a neat take on differentiation.
pub fn predict(history: &[isize]) -> isize {
    let n = history.len();
    let mut v = history.to_vec();
    for i in 0..n - 1 {
        for j in 0..(n - i) - 1 {
            v[j] = v[j + 1] - v[j];
        }
    }
    v.iter().sum()
}

pub fn parse(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .flat_map(str::parse::<isize>)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod day09 {
    use super::*;

    const SAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test1() {
        let histories = parse(SAMPLE);
        assert_eq!(predict(&histories[0]), 18);
        assert_eq!(predict(&histories[1]), 28);
        assert_eq!(predict(&histories[2]), 68);
    }

    #[test]
    fn test2() {
        let mut histories = parse(SAMPLE);
        histories[0].reverse();
        histories[1].reverse();
        histories[2].reverse();
        assert_eq!(predict(&histories[0]), -3);
        assert_eq!(predict(&histories[1]), 0);
        assert_eq!(predict(&histories[2]), 5);
    }
}
