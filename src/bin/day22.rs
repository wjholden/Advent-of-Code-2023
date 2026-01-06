use advent_of_code_2023::*;

pub const PUZZLE: &str = include_str!("../../puzzles/day22.txt");

fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1.unwrap());
    //println!("Part 2: {}", d.part2.unwrap());
    //println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: Option<usize>,
    pub part2: Option<usize>,
    bricks: Vec<Brick>,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug)]
struct Brick {
    start: Point,
    end: Point,
}

impl Brick {
    fn new(line: &str) -> Self {
        let tokens: Vec<usize> = line
            .split(['~', ','])
            .flat_map(str::parse::<usize>)
            .collect();
        assert_eq!(tokens.len(), 6);

        // This is actually given in the prompt, but I think it's a good
        // practice to test our assumptions.
        //
        // ```mathematica
        // In[1]:= LogicalExpand[(! a && ! b && ! c) || (a && ! b && ! c) || (! a && b && ! c) || (! a && ! b && c)]
        // Out[1]= (!a && !b) || (!a && !c) || (!b && !c)
        // ```
        let a = tokens[0] != tokens[3];
        let b = tokens[1] != tokens[4];
        let c = tokens[2] != tokens[5];
        let zero = !a && !b && !c;
        let one = (a && !b && !c) || (!a && b && !c) || (!a && !b && c);
        assert!(zero || one);

        // This assumption might not be stated, but it gives us a small
        // optimization later for sorting the input by starting z values.
        // We don't need to worry about the ending z values.
        assert!(tokens[0] <= tokens[3]);
        assert!(tokens[1] <= tokens[4]);
        assert!(tokens[2] <= tokens[5]);

        Self {
            start: Point {
                x: tokens[0],
                y: tokens[1],
                z: tokens[2],
            },
            end: Point {
                x: tokens[3],
                y: tokens[4],
                z: tokens[5],
            },
        }
    }

    fn volume(&self) -> usize {
        1 + self.start.x.abs_diff(self.end.x)
            + self.start.y.abs_diff(self.end.y)
            + self.start.z.abs_diff(self.end.z)
    }
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let mut bricks: Vec<_> = input.lines().map(Brick::new).collect();
        bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
        let min_x = bricks.iter().map(|b| b.start.x).min().unwrap();
        let min_y = bricks.iter().map(|b| b.start.y).min().unwrap();
        let max_x = bricks.iter().map(|b| b.end.x).max().unwrap();
        let max_y = bricks.iter().map(|b| b.end.y).max().unwrap();

        dbg!([min_x, max_x, min_y, max_y]);
        Self {
            part1: None,
            part2: None,
            bricks,
        }
    }

    fn solve(mut self) -> Self {
        // println!("{:?}", self.bricks);
        // println!(
        //     "{:?} ({} total)",
        //     self.bricks
        //         .iter()
        //         .fold(BTreeMap::new(), |mut acc: BTreeMap<usize, usize>, brick| {
        //             *acc.entry(brick.volume()).or_default() += 1;
        //             acc
        //         }),
        //     self.bricks.len()
        // );

        // Let's just do this the naive way for starters.
        for i in 1..self.bricks.len() {
            let (left, right) = self.bricks.split_at_mut(i);
            let brick = &mut right[0];
            // for brick in self.bricks.iter().skip_while(|brick| brick.start.z == 1) {
            // We have a brick above z=1. Find the highest thing directly beneath it.
            if let Some(tallest_z_under) = left
                .iter()
                .filter_map(|other| {
                    let Point { x: x1, y: y1, z: _ } = brick.start;
                    let Point { x: x2, y: y2, z: _ } = brick.end;
                    let Point { x: x3, y: y3, z: _ } = other.start;
                    let Point { x: x4, y: y4, z: _ } = other.end;
                    let x_overlap_1 = x1 <= x3 && x3 <= x2;
                    let x_overlap_2 = x1 <= x4 && x3 <= x2;
                    let y_overlap_1 = y1 <= y3 && y3 <= y2;
                    let y_overlap_2 = y1 <= y4 && y3 <= y2;
                    let overlaps = (x_overlap_1 || x_overlap_2) && (y_overlap_1 || y_overlap_2);
                    if overlaps { Some(other.end.z) } else { None }
                })
                .max()
                && brick.start.z > tallest_z_under + 1
            {
                println!("The highest brick under {brick:?} is at z={tallest_z_under}.");
                let shift = brick.start.z - tallest_z_under - 1;
                println!("I can shift by {shift}.");
                brick.start.z -= shift;
                brick.end.z -= shift;
            }
        }

        for brick in self.bricks.iter() {
            println!("{brick:?}");
        }
        self
    }
}

#[cfg(test)]
mod sand_slabs {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day22.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, Some(5));
    }
}
