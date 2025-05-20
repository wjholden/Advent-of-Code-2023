use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let puzzle = include_str!("../../puzzles/dayXX.txt").trim();
    println!("Part 1: {}", part1(puzzle)?);
    //println!("Part 2: {}", part2(puzzle)?);
    Ok(())
}

fn part1(input: &str) -> Result<u64, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod dayXX {
    use super::*;

    const SAMPLE: &str = "";

    #[test]
    fn test1() {
        assert_eq!(todo!(), part1(SAMPLE).unwrap());
    }

    #[test]
    fn test2() {
        //assert_eq!(todo!(), part2(SAMPLE).unwrap());
    }
}