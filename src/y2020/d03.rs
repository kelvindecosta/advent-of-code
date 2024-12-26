//! # Toboggan Trajectory
//!
//! We parse the input as a `Grid` and traverse it according to the slope.
//! When we reach the horizontal limit, we wrap around to the beginning, while
//! maintaining the vertical position.

use crate::util::{
  grid::Grid,
  point::{Point, ORIGIN},
};

pub fn parse(input: &str) -> Grid<u8> {
  Grid::parse(input.trim())
}

pub fn predict_encounters(forest: &Grid<u8>, slope: (i32, i32)) -> u64 {
  let mut pos = ORIGIN;
  let mut encounters = 0;
  let slope = Point::new(slope.0, slope.1);

  while pos.y < forest.height - 1 {
    pos += slope;
    pos.x %= forest.width;

    if forest[pos] == b'#' {
      encounters += 1;
    }
  }

  encounters
}

pub fn p1(input: &Grid<u8>) -> u64 {
  predict_encounters(input, (3, 1))
}

pub fn p2(input: &Grid<u8>) -> u64 {
  [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    .iter()
    .map(|&slope| predict_encounters(input, slope))
    .product()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
    7
  )]
  fn test_p1(#[case] input: &str, #[case] expected: u64) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
    336
  )]
  fn test_p2(#[case] input: &str, #[case] expected: u64) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
