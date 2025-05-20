use std::error::Error;
use ndarray::Array2;

pub fn text_to_grid(input: &str) -> Result<Array2<char>, Box<dyn Error>> {
    let cols = input.lines().next().ok_or("malformed input")?.len();
    let rows = input.lines().count();
    let mut grid: Array2<char> = Array2::from_elem((rows, cols), '.');
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[(i, j)] = c;
        }
    }
    Ok(grid)
}
