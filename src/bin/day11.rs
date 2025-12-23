use std::collections::HashSet;

fn main() {
    let image = Image::new(include_str!("../../puzzles/day11.txt"));
    println!("Part 1: {}", image.predict(2));
    println!("Part 2: {}", image.predict(1_000_000));
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

struct Image {
    galaxies: Vec<Position>,
}

impl Image {
    fn new(input: &str) -> Self {
        let mut galaxies = Vec::new();
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        galaxies.push(Position { x: col, y: row });
                    }
                    '.' => {}
                    _ => unreachable!(),
                }
            }
        }
        Image { galaxies }
    }

    fn predict(&self, expansion: usize) -> usize {
        let occupied_rows: HashSet<usize> = self.galaxies.iter().map(|p| p.y).collect();
        let occupied_cols: HashSet<usize> = self.galaxies.iter().map(|p| p.x).collect();
        let mut total = 0;
        for (i, p1) in self.galaxies.iter().enumerate() {
            for p2 in self.galaxies[i + 1..].iter() {
                let dy = p2.y.abs_diff(p1.y);
                let dx = p2.x.abs_diff(p1.x);
                let y_empty = (p2.y.min(p1.y) + 1..p2.y.max(p1.y))
                    .filter(|y| !occupied_rows.contains(y))
                    .count();
                let x_empty = (p2.x.min(p1.x) + 1..p2.x.max(p1.x))
                    .filter(|x| !occupied_cols.contains(x))
                    .count();
                let distance = dy + dx + (expansion - 1) * y_empty + (expansion - 1) * x_empty;
                total += distance;
            }
        }
        total
    }
}

#[cfg(test)]
mod day11 {
    use super::*;

    const SAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    const SMALL: &str = "...#
....
..#.
#...";

    #[test]
    fn test1() {
        assert_eq!(Image::new(SAMPLE).predict(2), 374)
    }

    #[test]
    fn test2() {
        assert_eq!(Image::new(SAMPLE).predict(10), 1030)
    }

    #[test]
    fn test3() {
        assert_eq!(Image::new(SAMPLE).predict(100), 8410)
    }

    #[test]
    fn small() {
        assert_eq!(Image::new(SMALL).predict(2), 16)
    }
}
