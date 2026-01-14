use std::error::Error;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, space1};
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};
use nom::{IResult, Parser};

pub const PUZZLE: &str = include_str!("../../puzzles/day02.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle = PUZZLE.trim();
    let games = parse_games(puzzle)?;
    println!("Part 1: {}", part1(&games)?);
    println!("Part 2: {}", part2(&games)?);
    Ok(())
}

pub fn part1(games: &[Game]) -> Result<u32, Box<dyn Error>> {
    let mut part1 = 0;
    'outer: for game in games {
        for round in game.subsets.iter() {
            if round.red > 12 || round.green > 13 || round.blue > 14 {
                continue 'outer;
            }
        }
        part1 += game.id;
    }
    Ok(part1)
}

pub fn part2(games: &[Game]) -> Result<u32, Box<dyn Error>> {
    let mut part2 = 0;
    for game in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for round in game.subsets.iter() {
            red = red.max(round.red);
            blue = blue.max(round.blue);
            green = green.max(round.green);
        }
        part2 += red * green * blue;
    }
    Ok(part2)
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

#[derive(Debug)]
struct Subset {
    blue: u32,
    red: u32,
    green: u32,
}

pub fn parse_games(input: &'static str) -> Result<Vec<Game>, Box<dyn Error>> {
    let (_input, games) = separated_list1(tag("\n"), parse_game).parse(input)?;
    Ok(games)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = delimited(tag("Game "), digit1, tag(": ")).parse(input)?;
    let id = id.parse().unwrap();
    let (input, games) = separated_list1(tag("; "), parse_subset).parse(input)?;
    Ok((input, Game { id, subsets: games }))
}

// https://blog.logrocket.com/parsing-in-rust-with-nom/
// https://blog.adamchalmers.com/nom-chars/
// https://developerlife.com/2023/02/20/guide-to-nom-parsing/
// https://gist.github.com/ponbac/abde10b07a1a96886dd8b1e188eb4374
#[allow(unused_mut)]
fn parse_subset(input: &str) -> IResult<&str, Subset> {
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;

    let (input, draws) =
        separated_list1(tag(", "), separated_pair(digit1, space1, alpha1)).parse(input)?;
    for (count, color) in draws {
        match color {
            "blue" => blue = count.parse().unwrap(),
            "red" => red = count.parse().unwrap(),
            "green" => green = count.parse().unwrap(),
            _ => unreachable!(),
        }
    }

    Ok((input, Subset { blue, red, green }))
}

#[cfg(test)]
mod cube_conundrum {
    use super::*;

    const SAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test1() {
        let g = parse_games(SAMPLE).unwrap();
        assert_eq!(8, part1(&g).unwrap())
    }

    #[test]
    fn test2() {
        let g = parse_games(SAMPLE).unwrap();
        assert_eq!(2286, part2(&g).unwrap())
    }
}
