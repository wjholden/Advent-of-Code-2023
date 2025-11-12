use std::error::Error;

/// Daily themes: calculus!
fn main() {
    let puzzle = include_str!("../../puzzles/day06.txt");
    println!("Part 1: {}", part1(puzzle));
    println!("Part 2: {}", part2(puzzle));
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
    let (time, distance) = parse2(input).unwrap();
    (0..=time).fold(0, |count, hold| {
        if (time - hold) * hold > distance {
            count + 1
        } else {
            count
        }
    })
}

#[derive(Default)]
struct Record {
    time: u64,
    distance: u64,
}

fn parse1(input: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    if let [time, distance] = input.lines().collect::<Vec<_>>()[0..2] {
        let time = time.split_ascii_whitespace().skip(1);
        let distance = distance.split_ascii_whitespace().skip(1);
        time.zip(distance)
            .map(|(time, distance)| {
                Ok(Record {
                    time: time.parse::<u64>()?,
                    distance: distance.parse::<u64>()?,
                })
            })
            .collect()
    } else {
        Err("failed to parse input")?
    }
}

fn parse2(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let input = input.replace(" ", "");
    let mut input = input.split([':', '\n']);
    input.next();
    let time = input.next().ok_or("no time")?.parse::<usize>()?;
    input.next();
    let distance = input.next().ok_or("no distance")?.parse()?;
    Ok((time, distance))
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
    fn no_kerning() {
        assert_eq!(parse2(SAMPLE).unwrap(), (71530, 940200))
    }
}
