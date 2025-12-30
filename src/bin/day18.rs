use advent_of_code_2023::*;
use itertools::Itertools;

pub const PUZZLE: &str = include_str!("../../puzzles/day18.txt");

/// The shoelace formula alone isn't enough to get us our total area. The
/// tiles our vertices fit into have thickness. The dig plan isn't giving
/// us the bounding box on the edge of the polygon; the real perimeter of
/// the polygon is half a unit outside of where we dig.
///
/// We can take advantage of the right turns of this problem by making some
/// assumptions. First, there four corners that each add 3/4 of a unit to
/// the area. Second, for each additional corner turning inwards or
/// outwards, there is a corresponding corner going the opposite way. We
/// don't need to keep track of which ones add 3/4 of a unit and which ones
/// add 1/4 of a unit -- it averages out to 1/2 unit for each additional
/// corner. Finally, we also have to add the perimeter. Each tile on the
/// perimeter adds 1/2 a unit.
///
/// https://mathworld.wolfram.com/PolygonArea.html
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug)]
enum Direction {
    R(isize),
    D(isize),
    L(isize),
    U(isize),
}

#[derive(Debug, Default, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Polygon {
    pixels: Vec<Point>,
}

impl Polygon {
    fn from_directions(directions: &[Direction]) -> Self {
        let mut pixels = Vec::new();
        let mut position = Point::default();
        for direction in directions {
            let (dx, dy) = match *direction {
                Direction::R(dx) => (dx, 0),
                Direction::D(dy) => (0, -dy),
                Direction::L(dx) => (-dx, 0),
                Direction::U(dy) => (0, dy),
            };
            position.x += dx;
            position.y += dy;
            pixels.push(position.clone());
        }
        Self { pixels }
    }

    fn shoelace_area(&self) -> usize {
        (self
            .pixels
            .iter()
            .circular_tuple_windows()
            .map(|(p1, p2)| {
                let Point { x: x1, y: y1 } = p1;
                let Point { x: x2, y: y2 } = p2;
                x1 * y2 - x2 * y1 // determinant
            })
            .sum::<isize>()
            .abs()
            / 2) as usize
    }

    fn area(&self) -> usize {
        let perimeter = self
            .pixels
            .iter()
            .circular_tuple_windows()
            .map(|(p1, p2)| p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y))
            .sum::<usize>();
        let outer_area = (perimeter - self.pixels.len()) / 2;
        let four_corners = 3 * 4 / 4;
        let other_corners = (self.pixels.len() - 4) / 2;
        self.shoelace_area() + outer_area + four_corners + other_corners
    }
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    dig_plan: Vec<Direction>,
    dig_plan2: Vec<Direction>,
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let dig_plan = input
            .lines()
            .map(|s| {
                let mut it = s.split_ascii_whitespace();
                let direction = it.next().unwrap();
                let distance = it.next().unwrap().parse::<isize>().unwrap();
                match direction {
                    "R" => Direction::R(distance),
                    "D" => Direction::D(distance),
                    "L" => Direction::L(distance),
                    "U" => Direction::U(distance),
                    _ => panic!("unexpected direction"),
                }
            })
            .collect();
        let dig_plan2 = input
            .lines()
            .map(|s| {
                let n = s.len();
                let distance = isize::from_str_radix(&s[n - 7..n - 2], 16).unwrap();
                match &s[n - 2..n - 1] {
                    "0" => Direction::R(distance),
                    "1" => Direction::D(distance),
                    "2" => Direction::L(distance),
                    "3" => Direction::U(distance),
                    _ => panic!("unexpected direction"),
                }
            })
            .collect();
        Self {
            part1: 0,
            part2: 0,
            dig_plan,
            dig_plan2,
        }
    }

    fn solve(mut self) -> Self {
        let p = Polygon::from_directions(&self.dig_plan);
        self.part1 = p.area();
        let p2 = Polygon::from_directions(&self.dig_plan2);
        self.part2 = p2.area();
        self
    }
}

#[cfg(test)]
mod lavaduct_lagoon {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day18.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 62);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 952408144115);
    }
}
