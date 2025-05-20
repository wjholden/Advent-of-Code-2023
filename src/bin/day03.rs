use regex::Regex; // 1.11.1
use itertools::Itertools; // 0.14.0
use std::{collections::HashMap, error::Error};
use advent_of_code_2023::text_to_grid;

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle = include_str!("../../puzzles/day03.txt").trim();
    let (part1, part2) = solve(puzzle)?;
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let mut part1 = 0;
    let mut part2 = HashMap::new();
    let grid = text_to_grid(input)?;
    let re = Regex::new(r"\d+")?;
    for (i, line) in input.lines().enumerate() {
        for m in re.find_iter(line) {
            let r1 = if i > 0 { i - 1 } else { 0 };
            let r2 = if i < grid.nrows() - 1 { i + 1 } else { i };
            let c1 = if m.start() > 0 { m.start() - 1 } else { 0 };
            let c2 = if m.end() == grid.ncols() { m.end() - 1 } else { m.end() };
 
            let adjacent = !(r1..=r2).cartesian_product(c1..=c2).all(|(r,c)|
                grid[(r,c)] == '.' || grid[(r,c)].is_ascii_digit()
            );
            
            let value = m.as_str().parse::<u32>()?;
            
            let gears = (r1..=r2).cartesian_product(c1..=c2).into_iter().filter(|&x| grid[x] == '*').collect::<Vec<_>>();
            assert!(gears.len() <= 1);
            if gears.len() == 1 {
                part2.entry(gears[0]).or_insert(vec![]).push(value);
            }

            if adjacent {
                part1 += value;
            }
        }
    }
    let gear_ratio = part2.into_values().filter(|v| v.len() == 2).map(|v| v[0] * v[1]).sum();
    Ok((part1, gear_ratio))
}

#[cfg(test)]
mod day03 {
    use super::*;

    const SAMPLE1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    const SAMPLE2: &str = "467..114.
...*.....
..35..633
......#..
617*.....
.....+.58
..592....
......755
...$.*...
.664.598.";

    #[test]
    fn test1() {
        assert_eq!(4361, solve(SAMPLE1).unwrap().0);
        assert_eq!(4361, solve(SAMPLE2).unwrap().0);
    }

    #[test]
    fn test2() {
        assert_eq!(467835, solve(SAMPLE1).unwrap().1);
        assert_eq!(467835, solve(SAMPLE2).unwrap().1);
    }
}