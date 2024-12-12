//! # Ceres Search
//!
//! Parse the input as a grid of bytes.
//!
//! ## Part 1
//!
//! Move through the x-axis and scan:
//! - each column (〣), proceeding downwards (↓)
//! - the upper right triangle (◥), proceeding downwards and rightwards (↘)
//! - the upper left triangle (◤), proceeding downwards and leftwards (↙)
//!
//! Move through the y-axis and scan:
//! - each row (≡), proceeding rightwards (→)
//! - starting one level lower, the lower left triangle (◣), proceeding
//!   downwards and rightwards (↘)
//! - starting one level lower, the lower right triangle (◢), proceeding
//!   downwards and leftwards (↙)
//!
//! ## Part 2
//!
//! Consider each point not on the border of the grid as the center of the "X".
//! Check if:
//! - the point is an "A"
//! - the diagonal characters sum to 160, which only happens for "M" and "S"

use lazy_static::lazy_static;

use crate::util::{
  grid::Grid,
  point::{Point, DOWN, LEFT, RIGHT},
};

lazy_static! {
  // Pre-compute the ASCII values of "XMAS" and "SAMX" as u32, i.e., 4 bytes.
  static ref XMAS: u32 = u32::from_be_bytes(*b"XMAS");
  static ref SAMX: u32 = u32::from_be_bytes(*b"SAMX");
}

pub fn parse(input: &str) -> Grid<u8> {
  Grid::parse(input)
}

pub fn p1(grid: &Grid<u8>) -> u32 {
  let mut result = 0;
  let larger_size = grid.width.max(grid.height);
  let smaller_size = grid.width.min(grid.height);

  for x in 0..grid.width {
    // ↓ - for each column (〣)
    result += scan_line(grid, Point::new(x, 0), DOWN, grid.height);

    // Skip the last 3 columns, since they cannot form a diagonal of size 4.
    if x < grid.width - 3 {
      // ↘ - across upper right triangle (◥)
      result += scan_line(
        grid,
        Point::new(x, 0),
        DOWN + RIGHT,
        (larger_size - x).min(grid.height),
      );
      // ↙ - across upper left triangle (◤)
      result += scan_line(
        grid,
        Point::new(grid.width - 1 - x, 0),
        DOWN + LEFT,
        (larger_size - x).min(grid.height),
      );
    }
  }

  for y in 0..grid.height {
    //  → - for reach row (≡)
    result += scan_line(grid, Point::new(0, y), RIGHT, grid.width);

    if y + 1 < grid.height - 3 {
      // ↘ - across lower left triangle (◣)
      result += scan_line(
        grid,
        Point::new(0, y + 1),
        DOWN + RIGHT,
        (grid.width - y).min(smaller_size) - 1,
      );
      // ↙ - across lower right triangle (◢)
      result += scan_line(
        grid,
        Point::new(grid.width - 1, y + 1),
        DOWN + LEFT,
        (grid.width - y).min(smaller_size) - 1,
      );
    }
  }

  result
}

pub fn p2(grid: &Grid<u8>) -> u32 {
  let mut result = 0;

  for x in 1..grid.width - 1 {
    for y in 1..grid.height - 1 {
      let point = Point::new(x, y);

      if grid[point] == b'A' {
        let ul = grid[Point::new(x - 1, y - 1)];
        let ur = grid[Point::new(x + 1, y - 1)];
        let dl = grid[Point::new(x - 1, y + 1)];
        let dr = grid[Point::new(x + 1, y + 1)];

        // Given the ASCII for M = 77, S = 83, we can check if M + S = 160.
        result += u32::from(ul + dr == 160 && ur + dl == 160);
      }
    }
  }

  result
}

/// Searches a horizontal, vertical or diagonal line in both directions at once.
fn scan_line(
  grid: &Grid<u8>,
  mut point: Point,
  direction: Point,
  size: i32,
) -> u32 {
  let mut window = 0;
  let mut result = 0;

  for _ in 0..size {
    // Shift the window by 1 byte and add the new byte.
    window = (window << 8) | u32::from(grid[point]);

    // Check if the window matches either "XMAS" or "SAMX".
    result += u32::from(window == *XMAS || window == *SAMX);

    point += direction;
  }

  result
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "..X...
.SAMX.
.A..A.
XMAS.S
.X....",
    4
  )]
  #[case(
    "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    18
  )]
  #[case(
    "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
",
    18
  )]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........",
    9
  )]
  fn test_p2(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
