use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let puzzle = include_str!("../../puzzles/day04.txt").trim();
    println!("Part 1: {}", part1(puzzle));
    println!("Part 2: {}", part2(puzzle));
}

fn part1(input: &str) -> u64 {
    let mut points = 0;
    for line in input.lines() {
        let [_card, winners, numbers] = line.split(|c| c == ':' || c == '|').map(str::trim).collect_array().unwrap();
        let winners: HashSet<&str> = HashSet::from_iter(winners.split_ascii_whitespace());
        let numbers: HashSet<&str> = HashSet::from_iter(numbers.split_ascii_whitespace());
        let n = winners.intersection(&numbers).count() as u32;
        if n > 0 {
            points += 2_u64.pow(n - 1);
        }
    }
    points
}

fn part2(input: &str) -> usize {
    let mut cards = vec![];
    for line in input.lines() {
        let tmp: Vec<_> = line.split(|c| c == ':' || c == '|').collect();
        let w: HashSet<_> = HashSet::from_iter(tmp[1].split_ascii_whitespace());
        let n: HashSet<_> = HashSet::from_iter(tmp[2].split_ascii_whitespace());
        let match_count = w.intersection(&n).count();
        cards.push(Card {
            match_count,
            card_count: 1
        });
    }
    //dbg!(&cards);
    for i in 0..cards.len() {
        let Card { match_count, card_count } = cards[i];
        for j in i+1..=i+match_count {
            cards[j].card_count += card_count;
        }
    }
    cards.into_iter().map(|card| card.card_count).sum()
}

#[derive(Debug)]
struct Card {
    match_count: usize,
    card_count: usize,
}

#[cfg(test)]
mod day04 {
    use super::*;

    const SAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test1() {
        assert_eq!(13, part1(SAMPLE));
    }

    #[test]
    fn test2() {
        assert_eq!(30, part2(SAMPLE));
    }
}