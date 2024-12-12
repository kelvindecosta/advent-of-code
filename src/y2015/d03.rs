//! # Perfectly Spherical Houses in a Vacuum
//!
//! The path taken by Santa is parsed as a sequence of directions.
//! Each point on the way is stored in a set to keep track of the unique houses
//! visited.
//!
//! For part 2, Santa follows every other instruction while the robot follows
//! the others.

use std::collections::HashSet;

use crate::util::point::{Point, ORIGIN};

pub fn parse(input: &str) -> Vec<Point> {
  input.trim().bytes().map(Point::from).collect()
}

fn deliver(input: &[Point], predicate: fn(usize) -> bool) -> usize {
  let mut visited = HashSet::new();
  let mut santa = ORIGIN;
  let mut robot = ORIGIN;
  visited.insert(ORIGIN);

  for (index, &direction) in input.iter().enumerate() {
    if predicate(index) {
      santa += direction;
      visited.insert(santa);
    } else {
      robot += direction;
      visited.insert(robot);
    }
  }

  visited.len()
}

pub fn p1(input: &[Point]) -> usize {
  deliver(input, |_| true)
}

pub fn p2(input: &[Point]) -> usize {
  deliver(input, |index| index % 2 == 0)
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(">", 2)]
  #[case("^>v<", 4)]
  #[case("^v^v^v^v^v", 2)]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case("^v", 3)]
  #[case("^>v<", 3)]
  #[case("^v^v^v^v^v", 11)]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}
