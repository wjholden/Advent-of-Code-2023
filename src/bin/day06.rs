use std::error::Error;

/// Daily themes: calculus!
fn main() {
    let puzzle = include_str!("../../puzzles/day06.txt");
    println!("Part 1: {} ({})", part1(puzzle), quadratic(puzzle));
    println!("Part 2: {} ({})", part2(puzzle), parse2(puzzle).unwrap().quadratic());
}

fn quadratic(input: &str) -> usize {
    parse1(input)
        .unwrap()
        .iter()
        .map(|record| {
            record.quadratic()
        })
        .product()
}

fn part1(input: &str) -> usize {
    parse1(input)
        .unwrap()
        .iter()
        .map(|record| {
            let Record { time, distance } = record;
            (0..=*time)
                .filter(|hold| (time - hold) * hold > *distance)
                .count()
        })
        .product()
}

fn part2(input: &str) -> usize {
    let Record { time, distance } = parse2(input).unwrap();
    (0..=time).fold(0, |count, hold| {
        if (time - hold) * hold > distance {
            count + 1
        } else {
            count
        }
    })
}

#[derive(PartialEq, Eq, Debug)]
struct Record {
    time: usize,
    distance: usize,
}

impl Record {
    fn quadratic(&self) -> usize {
        // Quadratic formula: x = (-b ± √(b² - 4ac)) / 2a
        // d = h(t-h) → d = -h² + ht → 0 = -h² + ht - d
        // ± (0177), → (8730), ² (0178), √ (251)
        let a = -1.0;
        let b = self.time as f64;
        let c = -(self.distance as f64);
        let x1 = ((-b - (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a)).ceil();
        let x2 = ((-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a)).floor();
        let winners = (x1 - x2 - 1.0) as usize;
        winners
    }
}

fn parse1(input: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    if let [time, distance] = input.lines().collect::<Vec<_>>()[0..2] {
        let time = time.split_ascii_whitespace().skip(1);
        let distance = distance.split_ascii_whitespace().skip(1);
        time.zip(distance)
            .map(|(time, distance)| {
                Ok(Record {
                    time: time.parse::<usize>()?,
                    distance: distance.parse::<usize>()?,
                })
            })
            .collect()
    } else {
        Err("failed to parse input")?
    }
}

fn parse2(input: &str) -> Result<Record, Box<dyn Error>> {
    let input = input.replace(" ", "");
    let mut input = input.split([':', '\n']);
    input.next();
    let time = input.next().ok_or("no time")?.parse::<usize>()?;
    input.next();
    let distance = input.next().ok_or("no distance")?.parse()?;
    Ok(Record { time, distance })
}

#[cfg(test)]
mod day06 {
    use super::*;

    const SAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 288);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(SAMPLE), 71503);
    }

    #[test]
    fn test3() {
        assert_eq!(part1(SAMPLE), quadratic(SAMPLE));
    }

    #[test]
    fn test4() {
        assert_eq!(part2(SAMPLE), parse2(SAMPLE).unwrap().quadratic());
    }

    #[test]
    fn no_kerning() {
        assert_eq!(parse2(SAMPLE).unwrap(), Record { time: 71530, distance: 940200 })
    }
}
