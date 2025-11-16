use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let puzzle = include_str!("../../puzzles/day07.txt");
    println!("Part 1: {}", solve(puzzle, Part::One));
    println!("Part 2: {}", solve(puzzle, Part::Two));
}

enum Part {
    One,
    Two,
}

fn solve(input: &str, part: Part) -> usize {
    let mut hands = input.lines().map(Hand::new).collect::<Vec<_>>();
    let cards = match part {
        Part::One => "23456789TJQKA",
        Part::Two => "J23456789TQKA",
    };
    hands.sort_by(|a, b| {
        let hand_comparison = match part {
            Part::One => a.hand_type.cmp(&b.hand_type),
            Part::Two => a.joker_type.cmp(&b.joker_type),
        };
        match hand_comparison {
            Ordering::Equal => {
                for (c1, c2) in a.hand.chars().zip(b.hand.chars()) {
                    let v1 = cards.find(c1).unwrap();
                    let v2 = cards.find(c2).unwrap();
                    if v1 != v2 {
                        return v1.cmp(&v2);
                    }
                }
                Ordering::Equal
            }
            other => other,
        }
    });
    // So funny story: the *weakest* hand gets rank 1.
    // I was thinking strongest would be 1. If you
    // reverse this, then the total winnings will be
    // a maddening 6640 instead of 6440 on the test input.
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (rank, hand)| acc + (rank + 1) * hand.bid)
}

#[derive(Debug)]
struct Hand {
    hand: String,
    bid: usize,
    hand_type: HandType,
    joker_type: HandType,
}

impl Hand {
    fn new(line: &str) -> Self {
        match line.split_ascii_whitespace().collect::<Vec<_>>()[..] {
            [hand, bid] if hand.len() == 5 => Self {
                hand: hand.to_owned(),
                bid: bid.parse::<usize>().expect("numeric bid"),
                hand_type: HandType::get(hand),
                joker_type: HandType::get(hand.replace("J", "").as_str()),
            },
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn get(hand: &str) -> Self {
        let mut tally: HashMap<char, usize> = HashMap::new();
        for c in hand.chars() {
            *tally.entry(c).or_default() += 1;
        }
        let mut result = HandType::HighCard;
        for value in tally.values() {
            result = match value {
                1 => result,
                2 if result == HandType::OnePair => HandType::TwoPair,
                2 if result == HandType::ThreeOfAKind => HandType::FullHouse,
                2 => HandType::OnePair,
                3 if result == HandType::OnePair => HandType::FullHouse,
                3 => HandType::ThreeOfAKind,
                4 => HandType::FourOfAKind,
                5 => HandType::FiveOfAKind,
                _ => panic!("unexpected tally in hand"),
            };
        }

        // Part 2
        let mut i = hand.len();
        if i == 0 {
            i = 1; // ugly handling for special case of JJJJJ
        }
        while i < 5 {
            result = match result {
                HandType::HighCard => HandType::OnePair,
                HandType::OnePair => HandType::ThreeOfAKind,
                HandType::TwoPair => HandType::FullHouse,
                HandType::ThreeOfAKind => HandType::FourOfAKind,
                HandType::FourOfAKind => HandType::FiveOfAKind,
                HandType::FullHouse | HandType::FiveOfAKind => {
                    dbg!(hand);
                    unreachable!()
                }
            };
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod day07 {
    use super::*;

    const SAMPLE: &str = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";

    #[test]
    fn test1() {
        assert_eq!(solve(SAMPLE, Part::One), 6440);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(SAMPLE, Part::Two), 5905);
    }

    #[test]
    fn test_types() {
        for (hand, hand_type) in [
            ("32T3K", HandType::OnePair),
            ("KK677", HandType::TwoPair),
            ("KTJJT", HandType::TwoPair),
            ("T55J5", HandType::ThreeOfAKind),
            ("QQQJA", HandType::ThreeOfAKind),
        ] {
            assert_eq!(HandType::get(hand), hand_type)
        }
    }
}
